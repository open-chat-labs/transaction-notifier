use ic_ledger_types::BlockIndex;
use serde::{Deserialize, Serialize};
use types::TimestampMillis;

#[derive(Serialize, Deserialize)]
pub struct LedgerSyncState {
    in_progress: bool,
    synced_up_to: BlockIndex,
    last_sync_started_at: TimestampMillis,
}

impl LedgerSyncState {
    pub fn new(latest_block_index: BlockIndex) -> LedgerSyncState {
        LedgerSyncState {
            in_progress: false,
            synced_up_to: latest_block_index,
            last_sync_started_at: 0,
        }
    }

    pub fn try_start(&mut self, now: TimestampMillis) -> TryStartSyncResult {
        if !self.in_progress {
            self.in_progress = true;
            self.last_sync_started_at = now;
            TryStartSyncResult::Success(self.synced_up_to)
        } else {
            TryStartSyncResult::AlreadyInProgress
        }
    }

    pub fn mark_sync_complete(&mut self) {
        self.in_progress = false;
    }

    pub fn synced_up_to(&self) -> BlockIndex {
        self.synced_up_to
    }

    pub fn set_synced_up_to(&mut self, block_index: BlockIndex) {
        self.synced_up_to = block_index;
    }

    pub fn last_sync_started_at(&self) -> TimestampMillis {
        self.last_sync_started_at
    }
}

pub enum TryStartSyncResult {
    Success(BlockIndex),
    AlreadyInProgress,
}
