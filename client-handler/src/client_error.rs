use mc_coms::{
    packet_reader::PacketReadError,
    packet_writer::PacketWriteError,
    ser::{ReadingError, WritingError},
};

use crate::client_state::ClientState;

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
}
