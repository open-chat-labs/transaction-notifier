use crate::{LedgerSyncState, TokenMetrics};
use ic_ledger_types::BlockIndex;
use serde::{Deserialize, Serialize};
use types::CanisterId;

#[derive(Serialize, Deserialize)]
pub struct TokenData {
    token_symbol: String,
    ledger_canister_id: CanisterId,
    ledger_sync_state: LedgerSyncState,
}

impl TokenData {
    pub fn new(
        token_symbol: String,
        ledger_canister_id: CanisterId,
        sync_from_block_index: BlockIndex,
    ) -> TokenData {
        TokenData {
            token_symbol,
            ledger_canister_id,
            ledger_sync_state: LedgerSyncState::new(sync_from_block_index),
        }
    }

    pub fn token_symbol(&self) -> &str {
        &self.token_symbol
    }

    pub fn ledger_canister_id(&self) -> CanisterId {
        self.ledger_canister_id
    }

    pub fn ledger_sync_state_mut(&mut self) -> &mut LedgerSyncState {
        &mut self.ledger_sync_state
    }

    pub fn metrics(&self) -> TokenMetrics {
        TokenMetrics {
            token_symbol: self.token_symbol.clone(),
            ledger_canister_id: self.ledger_canister_id,
            sync_enabled: self.ledger_sync_state.enabled(),
            synced_up_to: self.ledger_sync_state.next_block_to_sync().checked_sub(1),
            last_sync_started_at: self.ledger_sync_state.last_sync_started_at(),
            last_successful_sync: self.ledger_sync_state.last_successful_sync(),
            last_failed_sync: self.ledger_sync_state.last_failed_sync(),
        }
    }
}
