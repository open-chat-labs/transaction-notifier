use candid::CandidType;
use ic_ledger_types::{Block, BlockIndex};
use serde::{Deserialize, Serialize};
use types::CanisterId;

mod lifecycle;
mod queries;
mod updates;

pub use lifecycle::*;
pub use queries::*;
pub use updates::*;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct NotifyTransactionArgs {
    pub token_symbol: String,
    pub ledger_canister_id: CanisterId,
    pub block_index: BlockIndex,
    pub block: Block,
}
