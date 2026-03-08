mod api;
mod cmd;
mod config;
mod secret;

use anyhow::Result;
use clap::{Parser, Subcommand};

use cmd::auth::AuthStatusArgs;
use cmd::issue::attachment::IssueAttachmentListArgs;
use cmd::issue::comment::{
    IssueCommentAddArgs, IssueCommentDeleteArgs, IssueCommentListArgs, IssueCommentUpdateArgs,
};
use cmd::issue::{
    IssueCountArgs, IssueCreateArgs, IssueDeleteArgs, IssueListArgs, IssueShowArgs,
    IssueUpdateArgs, ParentChild,
};
use cmd::project::category::ProjectCategoryListArgs;
use cmd::project::issue_type::ProjectIssueTypeListArgs;
use cmd::project::status::ProjectStatusListArgs;
use cmd::project::user::ProjectUserListArgs;
use cmd::project::version::ProjectVersionListArgs;
use cmd::project::{ProjectActivitiesArgs, ProjectDiskUsageArgs, ProjectListArgs, ProjectShowArgs};
use cmd::space::{SpaceActivitiesArgs, SpaceDiskUsageArgs, SpaceNotificationArgs, SpaceShowArgs};

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
    /// Manage issues
    Issue {
        #[command(subcommand)]
        action: IssueCommands,
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
    /// Show recent activities for a project
    Activities {
        /// Project ID or key
        id_or_key: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show disk usage for a project
    DiskUsage {
        /// Project ID or key
        id_or_key: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Manage project users
    User {
        #[command(subcommand)]
        action: ProjectUserCommands,
    },
    /// Manage project statuses
    Status {
        #[command(subcommand)]
        action: ProjectStatusCommands,
    },
    /// Manage project issue types
    IssueType {
        #[command(subcommand)]
        action: ProjectIssueTypeCommands,
    },
    /// Manage project categories
    Category {
        #[command(subcommand)]
        action: ProjectCategoryCommands,
    },
    /// Manage project versions
    Version {
        #[command(subcommand)]
        action: ProjectVersionCommands,
    },
}

#[derive(Subcommand)]
enum ProjectUserCommands {
    /// List users in a project
    List {
        /// Project ID or key
        id_or_key: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum ProjectStatusCommands {
    /// List statuses in a project
    List {
        /// Project ID or key
        id_or_key: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum ProjectIssueTypeCommands {
    /// List issue types in a project
    List {
        /// Project ID or key
        id_or_key: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum ProjectCategoryCommands {
    /// List categories in a project
    List {
        /// Project ID or key
        id_or_key: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum ProjectVersionCommands {
    /// List versions in a project
    List {
        /// Project ID or key
        id_or_key: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum IssueCommands {
    /// List issues
    List {
        /// Filter by project ID (repeatable)
        #[arg(long = "project-id", value_name = "ID")]
        project_ids: Vec<u64>,
        /// Filter by status ID (repeatable)
        #[arg(long = "status-id", value_name = "ID")]
        status_ids: Vec<u64>,
        /// Filter by assignee ID (repeatable)
        #[arg(long = "assignee-id", value_name = "ID")]
        assignee_ids: Vec<u64>,
        /// Filter by issue type ID (repeatable)
        #[arg(long = "issue-type-id", value_name = "ID")]
        issue_type_ids: Vec<u64>,
        /// Filter by category ID (repeatable)
        #[arg(long = "category-id", value_name = "ID")]
        category_ids: Vec<u64>,
        /// Filter by milestone ID (repeatable)
        #[arg(long = "milestone-id", value_name = "ID")]
        milestone_ids: Vec<u64>,
        /// Filter by parent-child relation
        #[arg(long)]
        parent_child: Option<ParentChild>,
        /// Search keyword
        #[arg(long)]
        keyword: Option<String>,
        /// Number of issues to retrieve (max 100)
        #[arg(long, default_value = "20")]
        count: u32,
        /// Offset for pagination
        #[arg(long, default_value = "0")]
        offset: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Count issues
    Count {
        /// Filter by project ID (repeatable)
        #[arg(long = "project-id", value_name = "ID")]
        project_ids: Vec<u64>,
        /// Filter by status ID (repeatable)
        #[arg(long = "status-id", value_name = "ID")]
        status_ids: Vec<u64>,
        /// Filter by assignee ID (repeatable)
        #[arg(long = "assignee-id", value_name = "ID")]
        assignee_ids: Vec<u64>,
        /// Filter by issue type ID (repeatable)
        #[arg(long = "issue-type-id", value_name = "ID")]
        issue_type_ids: Vec<u64>,
        /// Filter by category ID (repeatable)
        #[arg(long = "category-id", value_name = "ID")]
        category_ids: Vec<u64>,
        /// Filter by milestone ID (repeatable)
        #[arg(long = "milestone-id", value_name = "ID")]
        milestone_ids: Vec<u64>,
        /// Filter by parent-child relation
        #[arg(long)]
        parent_child: Option<ParentChild>,
        /// Search keyword
        #[arg(long)]
        keyword: Option<String>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show an issue
    Show {
        /// Issue ID or key (e.g. TEST-1 or 123)
        id_or_key: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Create an issue
    Create {
        /// Project ID
        #[arg(long)]
        project_id: u64,
        /// Issue summary
        #[arg(long)]
        summary: String,
        /// Issue type ID
        #[arg(long)]
        issue_type_id: u64,
        /// Priority ID (1=High, 2=Normal, 3=Low)
        #[arg(long)]
        priority_id: u64,
        /// Description
        #[arg(long)]
        description: Option<String>,
        /// Assignee user ID
        #[arg(long)]
        assignee_id: Option<u64>,
        /// Due date (YYYY-MM-DD)
        #[arg(long)]
        due_date: Option<String>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Update an issue
    Update {
        /// Issue ID or key
        id_or_key: String,
        /// New summary
        #[arg(long)]
        summary: Option<String>,
        /// New description
        #[arg(long)]
        description: Option<String>,
        /// New status ID
        #[arg(long)]
        status_id: Option<u64>,
        /// New priority ID
        #[arg(long)]
        priority_id: Option<u64>,
        /// New assignee user ID
        #[arg(long)]
        assignee_id: Option<u64>,
        /// New due date (YYYY-MM-DD)
        #[arg(long)]
        due_date: Option<String>,
        /// Comment to add with update
        #[arg(long)]
        comment: Option<String>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Delete an issue
    Delete {
        /// Issue ID or key
        id_or_key: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Manage issue comments
    Comment {
        #[command(subcommand)]
        action: IssueCommentCommands,
    },
    /// Manage issue attachments
    Attachment {
        #[command(subcommand)]
        action: IssueAttachmentCommands,
    },
}

#[derive(Subcommand)]
enum IssueCommentCommands {
    /// List comments on an issue
    List {
        /// Issue ID or key
        id_or_key: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Add a comment to an issue
    Add {
        /// Issue ID or key
        id_or_key: String,
        /// Comment content
        #[arg(long)]
        content: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Update a comment
    Update {
        /// Issue ID or key
        id_or_key: String,
        /// Comment ID
        comment_id: u64,
        /// New content
        #[arg(long)]
        content: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Delete a comment
    Delete {
        /// Issue ID or key
        id_or_key: String,
        /// Comment ID
        comment_id: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum IssueAttachmentCommands {
    /// List attachments on an issue
    List {
        /// Issue ID or key
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

fn main() -> std::process::ExitCode {
    if let Err(e) = run() {
        use owo_colors::OwoColorize;
        anstream::eprintln!("{}: {e:#}", "ERROR".red().bold());
        return std::process::ExitCode::FAILURE;
    }
    std::process::ExitCode::SUCCESS
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    if cli.no_color {
        // SAFETY: called before any threads are spawned
        unsafe { std::env::set_var("NO_COLOR", "1") };
    }
    match cli.command {
        Commands::Auth { action } => match action {
            AuthCommands::Login => cmd::auth::login(),
            AuthCommands::Status { json } => cmd::auth::status(&AuthStatusArgs::new(json)),
            AuthCommands::Logout => cmd::auth::logout(),
            AuthCommands::Keyring => cmd::auth::check_keyring(),
        },
        Commands::Project { action } => match action {
            ProjectCommands::List { json } => cmd::project::list(&ProjectListArgs::new(json)),
            ProjectCommands::Show { id_or_key, json } => {
                cmd::project::show(&ProjectShowArgs::new(id_or_key, json))
            }
            ProjectCommands::Activities { id_or_key, json } => {
                cmd::project::activities(&ProjectActivitiesArgs::new(id_or_key, json))
            }
            ProjectCommands::DiskUsage { id_or_key, json } => {
                cmd::project::disk_usage(&ProjectDiskUsageArgs::new(id_or_key, json))
            }
            ProjectCommands::User { action } => match action {
                ProjectUserCommands::List { id_or_key, json } => {
                    cmd::project::user::list(&ProjectUserListArgs::new(id_or_key, json))
                }
            },
            ProjectCommands::Status { action } => match action {
                ProjectStatusCommands::List { id_or_key, json } => {
                    cmd::project::status::list(&ProjectStatusListArgs::new(id_or_key, json))
                }
            },
            ProjectCommands::IssueType { action } => match action {
                ProjectIssueTypeCommands::List { id_or_key, json } => {
                    cmd::project::issue_type::list(&ProjectIssueTypeListArgs::new(id_or_key, json))
                }
            },
            ProjectCommands::Category { action } => match action {
                ProjectCategoryCommands::List { id_or_key, json } => {
                    cmd::project::category::list(&ProjectCategoryListArgs::new(id_or_key, json))
                }
            },
            ProjectCommands::Version { action } => match action {
                ProjectVersionCommands::List { id_or_key, json } => {
                    cmd::project::version::list(&ProjectVersionListArgs::new(id_or_key, json))
                }
            },
        },
        Commands::Issue { action } => match action {
            IssueCommands::List {
                project_ids,
                status_ids,
                assignee_ids,
                issue_type_ids,
                category_ids,
                milestone_ids,
                parent_child,
                keyword,
                count,
                offset,
                json,
            } => cmd::issue::list(&IssueListArgs::try_new(
                project_ids,
                status_ids,
                assignee_ids,
                issue_type_ids,
                category_ids,
                milestone_ids,
                parent_child,
                keyword,
                count,
                offset,
                json,
            )?),
            IssueCommands::Count {
                project_ids,
                status_ids,
                assignee_ids,
                issue_type_ids,
                category_ids,
                milestone_ids,
                parent_child,
                keyword,
                json,
            } => cmd::issue::count(&IssueCountArgs::new(
                project_ids,
                status_ids,
                assignee_ids,
                issue_type_ids,
                category_ids,
                milestone_ids,
                parent_child,
                keyword,
                json,
            )),
            IssueCommands::Show { id_or_key, json } => {
                cmd::issue::show(&IssueShowArgs::new(id_or_key, json))
            }
            IssueCommands::Create {
                project_id,
                summary,
                issue_type_id,
                priority_id,
                description,
                assignee_id,
                due_date,
                json,
            } => cmd::issue::create(&IssueCreateArgs::new(
                project_id,
                summary,
                issue_type_id,
                priority_id,
                description,
                assignee_id,
                due_date,
                json,
            )),
            IssueCommands::Update {
                id_or_key,
                summary,
                description,
                status_id,
                priority_id,
                assignee_id,
                due_date,
                comment,
                json,
            } => cmd::issue::update(&IssueUpdateArgs::try_new(
                id_or_key,
                summary,
                description,
                status_id,
                priority_id,
                assignee_id,
                due_date,
                comment,
                json,
            )?),
            IssueCommands::Delete { id_or_key, json } => {
                cmd::issue::delete(&IssueDeleteArgs::new(id_or_key, json))
            }
            IssueCommands::Comment { action } => match action {
                IssueCommentCommands::List { id_or_key, json } => {
                    cmd::issue::comment::list(&IssueCommentListArgs::new(id_or_key, json))
                }
                IssueCommentCommands::Add {
                    id_or_key,
                    content,
                    json,
                } => cmd::issue::comment::add(&IssueCommentAddArgs::new(id_or_key, content, json)),
                IssueCommentCommands::Update {
                    id_or_key,
                    comment_id,
                    content,
                    json,
                } => cmd::issue::comment::update(&IssueCommentUpdateArgs::new(
                    id_or_key, comment_id, content, json,
                )),
                IssueCommentCommands::Delete {
                    id_or_key,
                    comment_id,
                    json,
                } => cmd::issue::comment::delete(&IssueCommentDeleteArgs::new(
                    id_or_key, comment_id, json,
                )),
            },
            IssueCommands::Attachment { action } => match action {
                IssueAttachmentCommands::List { id_or_key, json } => {
                    cmd::issue::attachment::list(&IssueAttachmentListArgs::new(id_or_key, json))
                }
            },
        },
        Commands::Space { action, json } => match action {
            None => cmd::space::show(&SpaceShowArgs::new(json)),
            Some(SpaceCommands::Activities { json: sub_json }) => {
                cmd::space::activities(&SpaceActivitiesArgs::new(json || sub_json))
            }
            Some(SpaceCommands::DiskUsage { json: sub_json }) => {
                cmd::space::disk_usage(&SpaceDiskUsageArgs::new(json || sub_json))
            }
            Some(SpaceCommands::Notification { json: sub_json }) => {
                cmd::space::notification(&SpaceNotificationArgs::new(json || sub_json))
            }
        },
    }
}
