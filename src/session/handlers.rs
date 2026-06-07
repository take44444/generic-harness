use crate::session::session::Session;
use std::sync::Arc;
use tracing::debug;

pub(super) async fn submission_loop(sess: Arc<Session>) {
    debug!("Agent loop exited");
}
