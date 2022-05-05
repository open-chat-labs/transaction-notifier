use crate::NotifyTransactionArgs;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use types::CanisterId;

#[derive(Serialize, Deserialize, Default)]
pub struct Notifications {
    queue: VecDeque<Notification>,
    total_sent: u64,
}

impl Notifications {
    pub fn enqueue(&mut self, notification: Notification) {
        self.queue.push_back(notification);
    }

    pub fn dequeue(&mut self) -> Option<Notification> {
        self.queue.pop_front()
    }

    pub fn mark_sent(&mut self) {
        self.total_sent += 1;
    }

    pub fn total_sent(&self) -> u64 {
        self.total_sent
    }

    pub fn queue_len(&self) -> usize {
        self.queue.len()
    }

    pub fn is_queue_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Notification {
    pub canister_id: CanisterId,
    pub args: NotifyTransactionArgs,
}
