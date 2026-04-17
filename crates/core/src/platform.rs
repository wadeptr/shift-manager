use crate::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PlatformMode {
    /// Daemon runs on the same machine as agents.
    Local,
    /// Daemon runs on a remote coordinator; manages host via SSH + WoL.
    Ssh,
}

/// Identifies the machine being managed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub mode: PlatformMode,
    /// SSH: "user@host". Local: ignored.
    pub host: Option<String>,
    /// SSH private key path.
    pub ssh_key: Option<std::path::PathBuf>,
    /// MAC address for Wake-on-LAN.
    pub mac_address: Option<String>,
    /// SSH port (default 22).
    pub ssh_port: Option<u16>,
}

impl Target {
    pub fn local() -> Self {
        Self {
            mode: PlatformMode::Local,
            host: None,
            ssh_key: None,
            mac_address: None,
            ssh_port: None,
        }
    }

    pub fn ssh(host: impl Into<String>, ssh_key: std::path::PathBuf, mac_address: impl Into<String>) -> Self {
        Self {
            mode: PlatformMode::Ssh,
            host: Some(host.into()),
            ssh_key: Some(ssh_key),
            mac_address: Some(mac_address.into()),
            ssh_port: Some(22),
        }
    }
}

#[async_trait]
pub trait Platform: Send + Sync {
    fn mode(&self) -> PlatformMode;

    /// Suspend (sleep/hibernate) the target machine.
    async fn suspend(&self, target: &Target) -> Result<()>;

    /// Wake the target machine (RTC alarm or WoL).
    async fn wake(&self, target: &Target) -> Result<()>;

    /// Returns true if the target machine is reachable.
    async fn is_alive(&self, target: &Target) -> Result<bool>;
}

#[cfg(any(test, feature = "test-utils"))]
pub mod tests {
    use super::*;
    use mockall::mock;

    mock! {
        pub Platform {}

        #[async_trait]
        impl Platform for Platform {
            fn mode(&self) -> PlatformMode;
            async fn suspend(&self, target: &Target) -> Result<()>;
            async fn wake(&self, target: &Target) -> Result<()>;
            async fn is_alive(&self, target: &Target) -> Result<bool>;
        }
    }

    #[test]
    fn local_target_has_no_host() {
        let t = Target::local();
        assert_eq!(t.mode, PlatformMode::Local);
        assert!(t.host.is_none());
    }

    #[test]
    fn ssh_target_defaults_to_port_22() {
        let t = Target::ssh("user@host", std::path::PathBuf::from("/key"), "aa:bb:cc");
        assert_eq!(t.ssh_port, Some(22));
    }
}
