use crate::error::{HarnessErr, HarnessResult};
use crate::harness_thread::HarnessThread;
use crate::session::Harness;
use std::collections::BTreeMap;
use std::fmt::Display;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::warn;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct ThreadId {
    uuid: Uuid,
}

impl ThreadId {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::now_v7(),
        }
    }
}

impl Display for ThreadId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.uuid, f)
    }
}

/// [`ThreadManager`] is responsible for creating threads and maintaining
/// them in memory.
pub struct ThreadManager {
    threads: Arc<RwLock<BTreeMap<ThreadId, Arc<HarnessThread>>>>,
}

impl ThreadManager {
    pub fn new() -> Self {
        Self {
            threads: Arc::new(RwLock::new(BTreeMap::new())),
        }
    }

    pub async fn start_thread(&self) -> HarnessResult<Arc<HarnessThread>> {
        // Implementation for starting a new thread
        let harness = Harness::spawn().await?;

        let thread_id = ThreadId::new();

        let mut threads = self.threads.write().await;
        match threads.entry(thread_id) {
            std::collections::btree_map::Entry::Occupied(_) => {
                if let Err(err) = harness.shutdown_and_wait().await {
                    warn!("failed to shut down duplicate thread {thread_id}: {err}");
                }
                Err(HarnessErr::InvalidRequest(format!(
                    "thread {thread_id} is already running"
                )))
            }
            std::collections::btree_map::Entry::Vacant(e) => {
                let thread = Arc::new(HarnessThread::new(harness));
                e.insert(thread.clone());
                Ok(thread)
            }
        }
    }
}
