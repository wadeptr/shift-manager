use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShiftError {
    #[error("agent error: {0}")]
    Agent(String),

    #[error("platform error: {0}")]
    Platform(String),

    #[error("state error: {0}")]
    State(String),

    #[error("session not found: {0}")]
    SessionNotFound(String),

    #[error("suspension failed: {0}")]
    SuspendFailed(String),

    #[error("wake failed: {0}")]
    WakeFailed(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, ShiftError>;
