use crate::constants::CRATE_VERSION;

pub fn crate_user_agent() -> String {
    format!("csgoempire-rs v{} API Bot", CRATE_VERSION)
}

#[cfg(feature = "base64")]
pub fn base64_encode(string: impl AsRef<[u8]>) -> String {
    base64::Engine::encode(&base64::prelude::BASE64_STANDARD, string)
}
