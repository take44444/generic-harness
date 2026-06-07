use std::sync::Arc;

/// Context for an initialized model agent
///
/// A session has at most 1 running task at a time, and can be interrupted by user input.
pub(crate) struct Session {}

impl Session {
    pub(crate) async fn new() -> Arc<Self> {
        Arc::new(Session {})
    }
}
