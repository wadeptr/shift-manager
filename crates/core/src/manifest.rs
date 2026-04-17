use crate::agent::SessionState;
use crate::error::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Written to disk/state-backend before every suspend. Used to restore sessions on wake.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub version: u32,
    pub suspended_at: DateTime<Utc>,
    pub sessions: Vec<SessionState>,
}

impl Manifest {
    pub fn new(sessions: Vec<SessionState>) -> Self {
        Self {
            version: 1,
            suspended_at: Utc::now(),
            sessions,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.sessions.is_empty()
    }

    pub fn serialize(&self) -> Result<String> {
        serde_json::to_string_pretty(self).map_err(|e| anyhow::anyhow!(e).into())
    }

    pub fn deserialize(s: &str) -> Result<Self> {
        serde_json::from_str(s).map_err(|e| anyhow::anyhow!(e).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::tests::{make_session, make_state};

    #[test]
    fn manifest_round_trips_json() {
        let session = make_session("claude-code");
        let state = make_state(&session);
        let manifest = Manifest::new(vec![state]);

        let json = manifest.serialize().unwrap();
        let restored = Manifest::deserialize(&json).unwrap();

        assert_eq!(restored.version, 1);
        assert_eq!(restored.sessions.len(), 1);
        assert_eq!(
            restored.sessions[0].agent_type,
            "claude-code"
        );
    }

    #[test]
    fn empty_manifest() {
        let m = Manifest::new(vec![]);
        assert!(m.is_empty());
    }
}
