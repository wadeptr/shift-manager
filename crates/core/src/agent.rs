use crate::error::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SessionId(pub Uuid);

impl SessionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for SessionId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for SessionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SessionStatus {
    Running,
    Pausing,
    Paused,
    Resuming,
    Failed,
}

/// Live session — a running agent process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: SessionId,
    pub agent_type: String,
    pub pid: Option<u32>,
    pub working_dir: std::path::PathBuf,
    pub status: SessionStatus,
    pub started_at: DateTime<Utc>,
    pub label: Option<String>,
}

/// Serialized snapshot of a session, written to the state backend before suspend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionState {
    pub session_id: SessionId,
    pub agent_type: String,
    pub working_dir: std::path::PathBuf,
    /// Agent-specific fields (e.g. conversation_id for Claude Code).
    pub metadata: HashMap<String, String>,
    pub paused_at: DateTime<Utc>,
    pub label: Option<String>,
    /// Shell command to re-launch this session.
    pub resume_command: String,
}

#[async_trait]
pub trait AgentAdapter: Send + Sync {
    fn agent_type(&self) -> &'static str;

    /// Find all running sessions managed by this adapter.
    async fn discover(&self) -> Result<Vec<Session>>;

    /// Gracefully pause a session and return its serialized state.
    async fn pause(&self, session: &Session) -> Result<SessionState>;

    /// Re-launch a session from saved state.
    async fn resume(&self, state: &SessionState) -> Result<Session>;

    /// Check whether a session is still alive.
    async fn is_alive(&self, session: &Session) -> Result<bool>;
}

#[cfg(any(test, feature = "test-utils"))]
pub mod tests {
    use super::*;
    use mockall::mock;

    mock! {
        pub AgentAdapter {}

        #[async_trait]
        impl AgentAdapter for AgentAdapter {
            fn agent_type(&self) -> &'static str;
            async fn discover(&self) -> Result<Vec<Session>>;
            async fn pause(&self, session: &Session) -> Result<SessionState>;
            async fn resume(&self, state: &SessionState) -> Result<Session>;
            async fn is_alive(&self, session: &Session) -> Result<bool>;
        }
    }

    pub fn make_session(agent_type: &str) -> Session {
        Session {
            id: SessionId::new(),
            agent_type: agent_type.to_string(),
            pid: Some(12345),
            working_dir: std::path::PathBuf::from("/tmp"),
            status: SessionStatus::Running,
            started_at: Utc::now(),
            label: None,
        }
    }

    pub fn make_state(session: &Session) -> SessionState {
        SessionState {
            session_id: session.id.clone(),
            agent_type: session.agent_type.clone(),
            working_dir: session.working_dir.clone(),
            metadata: HashMap::new(),
            paused_at: Utc::now(),
            label: None,
            resume_command: "claude --continue abc123".to_string(),
        }
    }

    #[test]
    fn session_id_display() {
        let id = SessionId::new();
        assert_eq!(id.to_string(), id.0.to_string());
    }

    #[test]
    fn session_status_serializes() {
        let s = SessionStatus::Running;
        let json = serde_json::to_string(&s).unwrap();
        assert_eq!(json, "\"running\"");
    }
}
