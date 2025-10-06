pub mod clientbound;
pub mod serverbound;

/// TODO: Try to make a nice derive or smth for this.
/// Optimally we'd just like to have an attribute like this:
/// ```rust
/// #[mc_packet(0x01)]
/// pub struct MyPacket { ... }
/// ```
pub trait McPacket {
    fn packet_id() -> &'static usize;
}
