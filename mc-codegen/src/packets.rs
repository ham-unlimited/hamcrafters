use std::{collections::BTreeMap, fs, io::BufReader};

use serde::Deserialize;

const PACKETS_PATH: &str = "reports/packets.json";

pub type PhaseName = String;
pub type PacketName = String;

#[derive(Debug, Deserialize)]
pub struct Packets(
    /// A map from phase to the packets in that phase
    BTreeMap<PhaseName, PhasePackets>,
);

impl Packets {
    pub fn read_from_file(base_path: &str) -> eyre::Result<Self> {
        let path = format!("{base_path}/{PACKETS_PATH}");
        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);
        let packets = serde_json::from_reader(reader)?;
        Ok(packets)
    }
}

#[derive(Debug, Deserialize)]
pub struct PhasePackets {
    pub clientbound: Option<BTreeMap<PacketName, Packet>>,
    pub serverbound: Option<BTreeMap<PacketName, Packet>>,
}

#[derive(Debug, Deserialize)]
pub struct Packet {
    pub protocol_id: usize,
}
