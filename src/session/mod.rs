mod handlers;
mod session;
use self::session::Session;
use crate::error::HarnessResult;
use futures::future::BoxFuture;
use futures::future::Shared;
use futures::prelude::*;
use std::sync::Arc;
use tokio::task::JoinHandle;

/// The high-level interface to the Harness system.
/// It operates as a queue pair where you send submissions and receive events.
pub struct Harness {
    // pub(crate) tx_sub: Sender<Submission>,
    // pub(crate) rx_event: Receiver<Event>,
    // // Last known status of the agent.
    // pub(crate) agent_status: watch::Receiver<AgentStatus>,
    session: Arc<Session>,
    // Shared future for the background submission loop completion so multiple
    // callers can wait for shutdown.
    session_loop_termination: SessionLoopTermination,
}

pub(crate) type SessionLoopTermination = Shared<BoxFuture<'static, ()>>;

impl Harness {
    pub(crate) async fn spawn() -> HarnessResult<Self> {
        let session = Session::new().await;

        let session_for_loop = Arc::clone(&session);
        let session_loop_handle = tokio::spawn(async move {
            handlers::submission_loop(session_for_loop).await;
        });
        Ok(Self {
            session,
            session_loop_termination: session_loop_termination_from_handle(session_loop_handle),
        })
    }

    pub async fn shutdown_and_wait(&self) -> HarnessResult<()> {
        let session_loop_termination = self.session_loop_termination.clone();
        // match self.submit(Op::Shutdown).await {
        //     Ok(_) => {}
        //     Err(HarnessErr::InternalAgentDied) => {}
        //     Err(err) => return Err(err),
        // }
        session_loop_termination.await;
        Ok(())
    }
}

pub(crate) fn session_loop_termination_from_handle(
    handle: JoinHandle<()>,
) -> SessionLoopTermination {
    async move {
        let _ = handle.await;
    }
    .boxed()
    .shared()
}
