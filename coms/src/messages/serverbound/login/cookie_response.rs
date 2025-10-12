use crate::codec::prefixed_optional::PrefixedOptional;

#[derive(Debug, Deserialize)]
#[mc_packet(0x04)]
pub struct CookieResponse {
    pub key: String,
    pub payload: PrefixedOptional<PrefixedArray<u8>>,
}
