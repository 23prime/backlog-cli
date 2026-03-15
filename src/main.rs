mod api;
mod cmd;
mod config;
mod logger;
mod oauth;
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
use cmd::notification::{NotificationCountArgs, NotificationListArgs, NotificationReadArgs};
use cmd::project::category::ProjectCategoryListArgs;
use cmd::project::issue_type::ProjectIssueTypeListArgs;
use cmd::project::status::ProjectStatusListArgs;
use cmd::project::user::ProjectUserListArgs;
use cmd::project::version::ProjectVersionListArgs;
use cmd::project::{ProjectActivitiesArgs, ProjectDiskUsageArgs, ProjectListArgs, ProjectShowArgs};
use cmd::space::{SpaceActivitiesArgs, SpaceDiskUsageArgs, SpaceNotificationArgs, SpaceShowArgs};
use cmd::team::{TeamAddArgs, TeamDeleteArgs, TeamListArgs, TeamShowArgs, TeamUpdateArgs};
use cmd::user::{UserActivitiesArgs, UserListArgs, UserRecentlyViewedArgs, UserShowArgs};
use cmd::wiki::attachment::WikiAttachmentListArgs;
use cmd::wiki::{
    WikiCreateArgs, WikiDeleteArgs, WikiHistoryArgs, WikiListArgs, WikiShowArgs, WikiUpdateArgs,
};

#[derive(Parser)]
#[command(name = "bl", version, about = "Backlog CLI")]
struct Cli {
    /// Disable colored output
    #[arg(long, global = true)]
    no_color: bool,
    /// Enable verbose output (show HTTP requests and responses)
    #[arg(long, short, global = true)]
    verbose: bool,
    /// Override the active space for this command (or set BL_SPACE env var)
    #[arg(long, global = true, value_name = "SPACE_KEY")]
    space: Option<String>,
    /// Print the banner and exit
    #[arg(long)]
    banner: bool,
    #[command(subcommand)]
    command: Option<Commands>,
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
    /// Manage wiki pages
    Wiki {
        #[command(subcommand)]
        action: WikiCommands,
    },
    /// Manage users
    User {
        #[command(subcommand)]
        action: UserCommands,
    },
    /// Manage teams
    Team {
        #[command(subcommand)]
        action: TeamCommands,
    },
    /// Manage notifications
    Notification {
        #[command(subcommand)]
        action: NotificationCommands,
    },
}

#[derive(clap::ValueEnum, Clone)]
enum Order {
    Asc,
    Desc,
}

impl Order {
    fn as_str(&self) -> &'static str {
        match self {
            Order::Asc => "asc",
            Order::Desc => "desc",
        }
    }
}

#[derive(Subcommand)]
enum SpaceCommands {
    /// Show recent space activities
    Activities {
        /// Filter by activity type ID (repeatable)
        #[arg(long = "activity-type-id", value_name = "ID")]
        activity_type_ids: Vec<u32>,
        /// Minimum activity ID
        #[arg(long)]
        min_id: Option<u64>,
        /// Maximum activity ID
        #[arg(long)]
        max_id: Option<u64>,
        /// Number of activities to retrieve
        #[arg(long, default_value = "20")]
        count: u32,
        /// Sort order
        #[arg(long)]
        order: Option<Order>,
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
        /// Filter by activity type ID (repeatable)
        #[arg(long = "activity-type-id", value_name = "ID")]
        activity_type_ids: Vec<u32>,
        /// Minimum activity ID
        #[arg(long)]
        min_id: Option<u64>,
        /// Maximum activity ID
        #[arg(long)]
        max_id: Option<u64>,
        /// Number of activities to retrieve
        #[arg(long, default_value = "20")]
        count: u32,
        /// Sort order
        #[arg(long)]
        order: Option<Order>,
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
enum WikiCommands {
    /// List wiki pages in a project
    List {
        /// Project ID or key
        project_id_or_key: String,
        /// Search keyword
        #[arg(long)]
        keyword: Option<String>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show a wiki page
    Show {
        /// Wiki page ID
        wiki_id: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Create a wiki page
    Create {
        /// Project ID
        #[arg(long)]
        project_id: u64,
        /// Page name
        #[arg(long)]
        name: String,
        /// Page content
        #[arg(long)]
        content: String,
        /// Send mail notification
        #[arg(long)]
        mail_notify: bool,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Update a wiki page
    Update {
        /// Wiki page ID
        wiki_id: u64,
        /// New page name
        #[arg(long)]
        name: Option<String>,
        /// New page content
        #[arg(long)]
        content: Option<String>,
        /// Send mail notification
        #[arg(long)]
        mail_notify: bool,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Delete a wiki page
    Delete {
        /// Wiki page ID
        wiki_id: u64,
        /// Send mail notification
        #[arg(long)]
        mail_notify: bool,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show history of a wiki page
    History {
        /// Wiki page ID
        wiki_id: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Manage wiki attachments
    Attachment {
        #[command(subcommand)]
        action: WikiAttachmentCommands,
    },
}

#[derive(Subcommand)]
enum WikiAttachmentCommands {
    /// List attachments of a wiki page
    List {
        /// Wiki page ID
        wiki_id: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum UserCommands {
    /// List users in the space
    List {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show a user
    Show {
        /// User numeric ID
        id: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show recent activities of a user
    Activities {
        /// User numeric ID
        id: u64,
        /// Filter by activity type ID (repeatable)
        #[arg(long = "activity-type-id", value_name = "ID")]
        activity_type_ids: Vec<u32>,
        /// Minimum activity ID
        #[arg(long)]
        min_id: Option<u64>,
        /// Maximum activity ID
        #[arg(long)]
        max_id: Option<u64>,
        /// Number of activities to retrieve
        #[arg(long, default_value = "20")]
        count: u32,
        /// Sort order
        #[arg(long)]
        order: Option<Order>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show recently viewed issues (for the authenticated user)
    RecentlyViewed {
        /// Number of items to retrieve
        #[arg(long, default_value = "20")]
        count: u32,
        /// Offset for pagination
        #[arg(long, default_value = "0")]
        offset: u64,
        /// Sort order
        #[arg(long)]
        order: Option<Order>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum TeamCommands {
    /// List all teams in the space
    List {
        /// Number of teams to retrieve
        #[arg(long, default_value = "20")]
        count: u32,
        /// Offset for pagination
        #[arg(long, default_value = "0")]
        offset: u64,
        /// Sort order
        #[arg(long)]
        order: Option<Order>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show details of a specific team
    Show {
        /// Team numeric ID
        id: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Add a new team
    Add {
        /// Team name
        #[arg(long)]
        name: String,
        /// Member user IDs (repeatable)
        #[arg(long = "member-id", value_name = "ID")]
        member_ids: Vec<u64>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Update a team
    Update {
        /// Team ID
        id: u64,
        /// New team name
        #[arg(long)]
        name: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Delete a team
    Delete {
        /// Team ID
        id: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum NotificationCommands {
    /// List notifications
    List {
        /// Minimum notification ID
        #[arg(long)]
        min_id: Option<u64>,
        /// Maximum notification ID
        #[arg(long)]
        max_id: Option<u64>,
        /// Number of notifications to retrieve
        #[arg(long, default_value = "20")]
        count: u32,
        /// Sort order
        #[arg(long)]
        order: Option<Order>,
        /// Filter by sender user ID
        #[arg(long)]
        sender_id: Option<u64>,
        /// Show only unread notifications
        #[arg(long)]
        unread: bool,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Count unread notifications
    Count {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Mark a notification as read
    Read {
        /// Notification ID
        id: u64,
    },
    /// Reset the unread notification count
    ResetUnread,
}

#[derive(Subcommand)]
enum AuthCommands {
    /// Login with your API key
    Login {
        /// Skip the banner
        #[arg(long)]
        no_banner: bool,
    },
    /// Login via OAuth 2.0 (browser-based)
    LoginOauth {
        /// Skip the banner
        #[arg(long)]
        no_banner: bool,
        /// Local port for the OAuth callback server (must match the Redirect URI registered in Backlog)
        #[arg(long, default_value_t = oauth::DEFAULT_OAUTH_PORT)]
        port: u16,
    },
    /// Show current auth status
    Status {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Logout and remove stored credentials
    Logout {
        /// Space key to logout from (defaults to current space)
        space_key: Option<String>,
        /// Logout from all spaces and remove all config files
        #[arg(long, conflicts_with = "space_key")]
        all: bool,
    },
    /// Check if the system keyring is available
    Keyring,
    /// List all configured spaces
    List,
    /// Switch the current space
    Use {
        /// Space key to switch to
        space_key: String,
    },
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
    if cli.verbose {
        // SAFETY: called before any threads are spawned
        unsafe { std::env::set_var("BL_VERBOSE", "1") };
    }
    if let Some(ref space) = cli.space {
        // SAFETY: called before any threads are spawned
        unsafe { std::env::set_var("BL_SPACE", space) };
    }
    if cli.banner {
        cmd::banner::print_banner();
        return Ok(());
    }
    let Some(command) = cli.command else {
        cmd::banner::print_banner();
        use clap::CommandFactory;
        Cli::command().print_help()?;
        anstream::println!();
        return Ok(());
    };
    match command {
        Commands::Auth { action } => match action {
            AuthCommands::Login { no_banner } => cmd::auth::login(no_banner),
            AuthCommands::LoginOauth { no_banner, port } => cmd::auth::login_oauth(no_banner, port),
            AuthCommands::Status { json } => cmd::auth::status(&AuthStatusArgs::new(json)),
            AuthCommands::Logout { space_key, all } => {
                if all {
                    cmd::auth::logout_all()
                } else {
                    cmd::auth::logout(space_key.as_deref())
                }
            }
            AuthCommands::Keyring => cmd::auth::check_keyring(),
            AuthCommands::List => cmd::auth::list(),
            AuthCommands::Use { space_key } => cmd::auth::use_space(&space_key),
        },
        Commands::Project { action } => match action {
            ProjectCommands::List { json } => cmd::project::list(&ProjectListArgs::new(json)),
            ProjectCommands::Show { id_or_key, json } => {
                cmd::project::show(&ProjectShowArgs::new(id_or_key, json))
            }
            ProjectCommands::Activities {
                id_or_key,
                activity_type_ids,
                min_id,
                max_id,
                count,
                order,
                json,
            } => cmd::project::activities(&ProjectActivitiesArgs::try_new(
                id_or_key,
                json,
                activity_type_ids,
                min_id,
                max_id,
                count,
                order.map(|o| o.as_str().to_string()),
            )?),
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
        Commands::Wiki { action } => match action {
            WikiCommands::List {
                project_id_or_key,
                keyword,
                json,
            } => cmd::wiki::list(&WikiListArgs::new(project_id_or_key, keyword, json)),
            WikiCommands::Show { wiki_id, json } => {
                cmd::wiki::show(&WikiShowArgs::new(wiki_id, json))
            }
            WikiCommands::Create {
                project_id,
                name,
                content,
                mail_notify,
                json,
            } => cmd::wiki::create(&WikiCreateArgs::new(
                project_id,
                name,
                content,
                mail_notify,
                json,
            )),
            WikiCommands::Update {
                wiki_id,
                name,
                content,
                mail_notify,
                json,
            } => cmd::wiki::update(&WikiUpdateArgs::try_new(
                wiki_id,
                name,
                content,
                mail_notify,
                json,
            )?),
            WikiCommands::Delete {
                wiki_id,
                mail_notify,
                json,
            } => cmd::wiki::delete(&WikiDeleteArgs::new(wiki_id, mail_notify, json)),
            WikiCommands::History { wiki_id, json } => {
                cmd::wiki::history(&WikiHistoryArgs::new(wiki_id, json))
            }
            WikiCommands::Attachment { action } => match action {
                WikiAttachmentCommands::List { wiki_id, json } => {
                    cmd::wiki::attachment::list(&WikiAttachmentListArgs::new(wiki_id, json))
                }
            },
        },
        Commands::User { action } => match action {
            UserCommands::List { json } => cmd::user::list(&UserListArgs::new(json)),
            UserCommands::Show { id, json } => cmd::user::show(&UserShowArgs::new(id, json)),
            UserCommands::Activities {
                id,
                activity_type_ids,
                min_id,
                max_id,
                count,
                order,
                json,
            } => cmd::user::activities(&UserActivitiesArgs::try_new(
                id,
                json,
                activity_type_ids,
                min_id,
                max_id,
                count,
                order.map(|o| o.as_str().to_string()),
            )?),
            UserCommands::RecentlyViewed {
                count,
                offset,
                order,
                json,
            } => cmd::user::recently_viewed(&UserRecentlyViewedArgs::try_new(
                json,
                count,
                offset,
                order.map(|o| o.as_str().to_string()),
            )?),
        },
        Commands::Team { action } => match action {
            TeamCommands::List {
                count,
                offset,
                order,
                json,
            } => cmd::team::list(&TeamListArgs::try_new(
                json,
                count,
                order.map(|o| o.as_str().to_string()),
                offset,
            )?),
            TeamCommands::Show { id, json } => cmd::team::show(&TeamShowArgs::new(id, json)),
            TeamCommands::Add {
                name,
                member_ids,
                json,
            } => cmd::team::add(&TeamAddArgs::new(name, member_ids, json)),
            TeamCommands::Update { id, name, json } => {
                cmd::team::update(&TeamUpdateArgs::new(id, name, json))
            }
            TeamCommands::Delete { id, json } => cmd::team::delete(&TeamDeleteArgs::new(id, json)),
        },
        Commands::Notification { action } => match action {
            NotificationCommands::List {
                min_id,
                max_id,
                count,
                order,
                sender_id,
                unread,
                json,
            } => cmd::notification::list(&NotificationListArgs::try_new(
                json,
                min_id,
                max_id,
                count,
                order.map(|o| o.as_str().to_string()),
                sender_id,
                unread,
            )?),
            NotificationCommands::Count { json } => {
                cmd::notification::count(&NotificationCountArgs::new(json))
            }
            NotificationCommands::Read { id } => {
                cmd::notification::read(&NotificationReadArgs::new(id))
            }
            NotificationCommands::ResetUnread => cmd::notification::reset_unread(),
        },
        Commands::Space { action, json } => match action {
            None => cmd::space::show(&SpaceShowArgs::new(json)),
            Some(SpaceCommands::Activities {
                activity_type_ids,
                min_id,
                max_id,
                count,
                order,
                json: sub_json,
            }) => cmd::space::activities(&SpaceActivitiesArgs::try_new(
                json || sub_json,
                activity_type_ids,
                min_id,
                max_id,
                count,
                order.map(|o| o.as_str().to_string()),
            )?),
            Some(SpaceCommands::DiskUsage { json: sub_json }) => {
                cmd::space::disk_usage(&SpaceDiskUsageArgs::new(json || sub_json))
            }
            Some(SpaceCommands::Notification { json: sub_json }) => {
                cmd::space::notification(&SpaceNotificationArgs::new(json || sub_json))
            }
        },
    }
}
