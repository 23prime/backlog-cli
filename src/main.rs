mod api;
mod cmd;
mod config;
mod secret;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "bl", version, about = "Backlog CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage authentication
    Auth {
        #[command(subcommand)]
        action: AuthCommands,
    },
    /// Show space information
    Space,
}

#[derive(Subcommand)]
enum AuthCommands {
    /// Login with your API key
    Login,
    /// Show current auth status
    Status,
    /// Logout and remove stored credentials
    Logout,
    /// Check if the system keyring is available
    Keyring,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Auth { action } => match action {
            AuthCommands::Login => cmd::auth::login(),
            AuthCommands::Status => cmd::auth::status(),
            AuthCommands::Logout => cmd::auth::logout(),
            AuthCommands::Keyring => cmd::auth::check_keyring(),
        },
        Commands::Space => cmd::space::show(),
    }
}
