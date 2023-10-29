use crate::constants::CRATE_VERSION;

pub fn crate_user_agent() -> String {
    format!("csgoempire-rs v{} API Bot", CRATE_VERSION)
}
