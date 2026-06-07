use thiserror::Error;

pub type HarnessResult<T> = std::result::Result<T, HarnessErr>;

#[derive(Error, Debug)]
pub enum HarnessErr {
    /// Invalid request.
    #[error("{0}")]
    InvalidRequest(String),
    /// Agent loop died unexpectedly
    #[error("internal error; agent loop died unexpectedly")]
    InternalAgentDied,
}
