use crate::{LedgerSyncState, Subscriptions, TokenMetrics};
use ic_ledger_types::BlockIndex;
use serde::{Deserialize, Serialize};
use types::CanisterId;

#[derive(Serialize, Deserialize)]
pub struct TokenData {
    token_symbol: String,
    ledger_canister_id: CanisterId,
    ledger_sync_state: LedgerSyncState,
    subscriptions: Subscriptions,
}

impl TokenData {
    pub fn new(
        token_symbol: String,
        ledger_canister_id: CanisterId,
        from_block_index: BlockIndex,
    ) -> TokenData {
        TokenData {
            token_symbol,
            ledger_canister_id,
            ledger_sync_state: LedgerSyncState::new(from_block_index),
            subscriptions: Subscriptions::default(),
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

    pub fn subscriptions(&self) -> &Subscriptions {
        &self.subscriptions
    }

    pub fn subscriptions_mut(&mut self) -> &mut Subscriptions {
        &mut self.subscriptions
    }

    pub fn metrics(&self) -> TokenMetrics {
        TokenMetrics {
            token_symbol: self.token_symbol.clone(),
            ledger_canister_id: self.ledger_canister_id,
            sync_enabled: self.ledger_sync_state.enabled(),
            synced_up_to: self.ledger_sync_state.synced_up_to(),
            last_sync_started_at: self.ledger_sync_state.last_sync_started_at(),
            last_successful_sync: self.ledger_sync_state.last_successful_sync(),
            last_failed_sync: self.ledger_sync_state.last_failed_sync(),
            subscriptions: self.subscriptions.len().try_into().unwrap(),
        }
    }
}
