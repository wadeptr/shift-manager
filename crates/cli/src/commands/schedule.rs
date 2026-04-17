use anyhow::Result;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum ScheduleCommand {
    /// Show current schedule.
    Show,
    /// Set the suspend and wake times.
    Set {
        /// Time to suspend, 24h format (e.g. 23:00).
        #[arg(long)]
        suspend_at: String,
        /// Time to wake, 24h format (e.g. 08:00).
        #[arg(long)]
        wake_at: String,
    },
    /// Clear all scheduled triggers.
    Clear,
}

pub async fn handle(cmd: ScheduleCommand) -> Result<()> {
    match cmd {
        ScheduleCommand::Show => {
            println!("No schedule configured.");
        }
        ScheduleCommand::Set { suspend_at, wake_at } => {
            println!("Schedule set: suspend={suspend_at}  wake={wake_at}");
            // TODO: persist to config file
        }
        ScheduleCommand::Clear => {
            println!("Schedule cleared.");
        }
    }
    Ok(())
}
