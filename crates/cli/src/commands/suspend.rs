use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum SuspendCommand {
    /// Immediately pause all sessions and suspend the machine.
    Now,
    /// Resume sessions from the last saved manifest.
    Resume,
}

pub async fn handle(cmd: SuspendCommand) -> Result<()> {
    match cmd {
        SuspendCommand::Now => {
            println!("Triggering immediate suspend...");
            // TODO: connect to daemon and call lifecycle.suspend()
        }
        SuspendCommand::Resume => {
            println!("Triggering resume from manifest...");
            // TODO: connect to daemon and call lifecycle.resume()
        }
    }
    Ok(())
}
