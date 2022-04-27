use candid::{CandidType, Principal};
use serde::Deserialize;
use types::{CanisterId, Version};

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub token_symbol: String,
    pub ledger_canister_id: CanisterId,
    pub notification_method_name: Option<String>,
    pub admins: Vec<Principal>,
    pub wasm_version: Version,
    pub test_mode: bool,
}
