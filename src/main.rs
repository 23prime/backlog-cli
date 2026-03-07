mod api;
mod cmd;
mod config;
mod secret;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "bl", version, about = "Backlog CLI")]
struct Cli {
    /// Disable colored output
    #[arg(long, global = true)]
    no_color: bool,
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
    Space {
        #[command(subcommand)]
        action: Option<SpaceCommands>,
        /// Output as JSON (applies to default show action)
        #[arg(long)]
        json: bool,
    },
    /// Manage projects
    Project {
        #[command(subcommand)]
        action: ProjectCommands,
    },
}

#[derive(Subcommand)]
enum SpaceCommands {
    /// Show recent space activities
    Activities {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show disk usage of the space
    DiskUsage {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show space notification
    Notification {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum ProjectCommands {
    /// List projects
    List {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show a project
    Show {
        /// Project ID or key (e.g. TEST or 123)
        id_or_key: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum AuthCommands {
    /// Login with your API key
    Login,
    /// Show current auth status
    Status {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Logout and remove stored credentials
    Logout,
    /// Check if the system keyring is available
    Keyring,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    if cli.no_color {
        // SAFETY: called before any threads are spawned
        unsafe { std::env::set_var("NO_COLOR", "1") };
    }
    match cli.command {
        Commands::Auth { action } => match action {
            AuthCommands::Login => cmd::auth::login(),
            AuthCommands::Status { json } => cmd::auth::status(json),
            AuthCommands::Logout => cmd::auth::logout(),
            AuthCommands::Keyring => cmd::auth::check_keyring(),
        },
        Commands::Project { action } => match action {
            ProjectCommands::List { json } => cmd::project::list(json),
            ProjectCommands::Show { id_or_key, json } => cmd::project::show(&id_or_key, json),
        },
        Commands::Space { action, json } => match action {
            None => cmd::space::show(json),
            Some(SpaceCommands::Activities { json: sub_json }) => {
                cmd::space::activities(json || sub_json)
            }
            Some(SpaceCommands::DiskUsage { json: sub_json }) => {
                cmd::space::disk_usage(json || sub_json)
            }
            Some(SpaceCommands::Notification { json: sub_json }) => {
                cmd::space::notification(json || sub_json)
            }
        },
    }
}
