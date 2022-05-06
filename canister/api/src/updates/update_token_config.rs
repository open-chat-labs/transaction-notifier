use candid::CandidType;
use ic_ledger_types::BlockIndex;
use serde::Deserialize;

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub token_symbol: String,
    pub sync_enabled: Option<bool>,
    pub sync_from_block_index: Option<BlockIndex>,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success,
    TokenNotFound,
}
