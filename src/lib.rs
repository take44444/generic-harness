// Prevent accidental direct writes to stdout/stderr in library code. All
// user-visible output must go through the appropriate abstraction (e.g.,
// the TUI or the tracing stack).
#![deny(clippy::print_stdout, clippy::print_stderr)]

mod error;
mod harness_thread;
mod session;
pub use harness_thread::HarnessThread;
mod thread_manager;
