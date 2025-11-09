use mc_coms::{
    client_state::ClientState,
    key_store::EncryptionError,
    messages::McPacketError,
    packet_reader::PacketReadError,
    packet_writer::PacketWriteError,
    ser::{ReadingError, WritingError},
};

/// An error that occurrs whilst communicating with a client.
#[allow(missing_docs)]
#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error("An error occurred whilst reading a packet `{0}`")]
    PacketReadError(#[from] PacketReadError),
    #[error("An error occurred whilst reading packet contents `{0}`")]
    ReadingError(#[from] ReadingError),
    #[error("An error occurred whilst writing packet contents `{0}`")]
    WritingError(#[from] WritingError),
    #[error("Failed to write packet to output `{0}`")]
    PacketWriteError(#[from] PacketWriteError),
    #[error(
        "Invalid minecraft protocol version received {received_version}, supported version is {supported_version}"
    )]
    InvalidProtocolVersion {
        received_version: usize,
        supported_version: usize,
    },
    #[error("Invalid packet ID {packet_id} for state {state:?}")]
    UnsupportedPacketId { packet_id: i32, state: ClientState },
    #[error(
        "An error occurred during encryption / decryption using the pub/priv key scheme, err: {0}"
    )]
    PubPrivEncryptionError(#[from] EncryptionError),
    #[error("Failed to parse UUID `{0}`")]
    UuidParseError(#[from] uuid::Error),
    #[error("Received verify token was invalid")]
    InvalidVerifyToken,
    #[error("Shared secret was invalid")]
    InvalidSharedSecret,
    #[error("McPacket error, err: `{0}`")]
    PacketError(#[from] McPacketError),
}
