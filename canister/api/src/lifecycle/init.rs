use candid::{CandidType, Principal};
use serde::Deserialize;
use types::Version;

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub admins: Vec<Principal>,
    pub notification_method_name: Option<String>,
    pub wasm_version: Version,
    pub test_mode: bool,
}
