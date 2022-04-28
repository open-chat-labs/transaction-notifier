use candid::CandidType;
use serde::Deserialize;
use types::CanisterId;

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub ledger_canister_id: CanisterId,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success,
    AlreadyAdded,
    LedgerError(String),
}
