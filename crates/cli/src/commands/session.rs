use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum SessionCommand {
    /// List all tracked sessions.
    List,
    /// Add a session to be managed (by PID or command).
    Add {
        #[arg(long)]
        pid: Option<u32>,
        #[arg(long)]
        label: Option<String>,
    },
    /// Remove a session from management.
    Remove {
        /// Session ID or label.
        id: String,
    },
}

pub async fn handle(cmd: SessionCommand) -> Result<()> {
    match cmd {
        SessionCommand::List => {
            println!("Tracked sessions: (none — daemon not connected)");
        }
        SessionCommand::Add { pid, label } => {
            println!(
                "Adding session: pid={:?} label={:?}",
                pid, label
            );
        }
        SessionCommand::Remove { id } => {
            println!("Removing session: {id}");
        }
    }
    Ok(())
}
