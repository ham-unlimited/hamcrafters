#[forbid(unsafe_code)]

/// Special special minecraft types e.g. VarInt etc.
pub mod codec;
/// Minecraft packet definitions.
pub mod messages;
/// Network coms for sending / receiving MC Packets.
pub mod net;
/// Reading / writing (a bit unclear tbh).
pub mod serial;
