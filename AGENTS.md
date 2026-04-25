# Hamcrafters — Agent Guidelines

## Project Overview

This is a Minecraft Java Edition server implementation in Rust. It includes a proxy, packet codec, NBT handling, world loading, and a simulation layer.

## Primary Reference: Minecraft Protocol

**All protocol-level code must be verified against the official Minecraft Wiki protocol page:**

> https://minecraft.wiki/w/Java_Edition_protocol/Packets

This is the authoritative source for:
- Packet IDs and their play/login/configuration/status phase
- Field names, types, and ordering
- Enumerations, bit flags, and conditional fields
- Data types such as `VarInt`, `VarLong`, `Identifier`, `UUID`, NBT, and bit sets

When implementing or modifying a packet, cross-reference the wiki before writing any code. If a field name, type, or packet ID in the codebase disagrees with the wiki, treat the wiki as correct unless there is explicit rationale in a code comment.

For data types not on the packet page, also consult:
- https://minecraft.wiki/w/Java_Edition_protocol/Data_types

The broader Minecraft Wiki (https://minecraft.wiki) is also an acceptable reference for game mechanics, world formats, NBT structures, registries, and any other Minecraft-specific domain knowledge. When in doubt, prefer wiki sources over assumptions or secondary sources.

## Code Style

Write idiomatic Rust. Run `cargo clippy` after every non-trivial change and resolve all warnings before considering work complete:

```sh
cargo clippy --all-targets --all-features -- -D warnings
```

Clippy is the canonical linter for this project. Do not suppress warnings with `#[allow(...)]` unless there is a specific, documented reason.

Additional style rules:
- Prefer `derive` macros over manual trait impls where appropriate (`Debug`, `Clone`, etc.)
- Use `thiserror` for error types; avoid `Box<dyn Error>` in library code
- Avoid `unwrap()` and `expect()` in non-test code unless the invariant is documented
- Packet structs use `#[mc_packet(0xID)]` from `mc-packet-macros` — keep packet IDs in sync with the wiki
- Field documentation comments (`///`) should describe the semantic meaning, not just the type

## Architecture

| Crate | Purpose |
|---|---|
| `coms` | Network codec: packet structs, data types, serialization/deserialization |
| `client-handler` | Per-connection state machine and packet dispatch |
| `proxy` | Transparent proxy for observing client↔server traffic |
| `server` | Entry point and top-level orchestration |
| `world` | World loading and chunk management |
| `nbt` | NBT encoding/decoding |
| `auth` | Mojang authentication |
| `mc-packet-macros` | Proc macros for packet registration |

Packet structs live under `coms/src/messages/` organised by direction (`clientbound`/`serverbound`) and phase (`play`, `login`, `configuration`, `status`).

## Run Modes

The server crate has two **mutually exclusive** Cargo features that control how the binary behaves:

| Feature | Purpose |
|---|---|
| `proxy` | Transparent proxy in front of a real Minecraft server. No simulation logic. Used for testing and protocol discovery. |
| `server` | Full server implementation with simulation. Default for production use. |

Only one feature may be active at a time. `server` is the default.

```sh
cargo run                        # run as server (default)
cargo run --no-default-features --features proxy   # run as proxy
```

When implementing new gameplay or simulation features, they belong under the `server` feature path and should be gated accordingly. Code that is shared between both modes (codec, packet structs, NBT, etc.) lives in the common crates and must not depend on either feature.

## Build and Test

```sh
cargo build          # build all crates
cargo test           # run all tests
cargo clippy --all-targets --all-features -- -D warnings   # lint
```

## Conventions

- Generated data under `generated/` comes from the Minecraft data generator — do not hand-edit those files
- Packet field order in struct definitions must match the wire order documented on the wiki
- Bit sets in chunk/light packets are `PrefixedArray<i64>`, matching the wiki's "Array of Long" representation
