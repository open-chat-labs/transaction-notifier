use candid::{CandidType, Principal};
use serde::Deserialize;
use types::{CanisterId, Version};

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub ledger_canister_id: CanisterId,
    pub admins: Vec<Principal>,
    pub wasm_version: Version,
    pub test_mode: bool,
}
