use crate::NotifyTransactionArgs;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use types::CanisterId;

#[derive(Serialize, Deserialize, Default)]
pub struct NotificationsQueue {
    queue: VecDeque<Notification>,
}

impl NotificationsQueue {
    pub fn add(&mut self, notification: Notification) {
        self.queue.push_back(notification);
    }

    pub fn take(&mut self) -> Option<Notification> {
        self.queue.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Notification {
    pub canister_id: CanisterId,
    pub args: NotifyTransactionArgs,
}
