use crate::session::Harness;

pub struct HarnessThread {
    pub(crate) harness: Harness,
}

/// Conduit for the bidirectional stream of messages that compose a thread
/// (formerly called a conversation) in Harness.
impl HarnessThread {
    pub(crate) fn new(harness: Harness) -> Self {
        Self { harness }
    }
}
