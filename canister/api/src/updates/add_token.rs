use candid::CandidType;
use ic_ledger_types::BlockIndex;
use serde::Deserialize;
use types::CanisterId;

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub ledger_canister_id: CanisterId,
    pub enable_sync: bool,
    pub sync_from_block_index: Option<BlockIndex>,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success,
    AlreadyAdded,
    LedgerError(String),
}
