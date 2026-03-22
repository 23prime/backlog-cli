mod api;
mod cmd;
mod config;
mod logger;
mod oauth;
mod secret;

use anyhow::Result;
use clap::{Parser, Subcommand};

use cmd::auth::AuthStatusArgs;
use cmd::document::attachment::DocumentAttachmentGetArgs;
use cmd::document::{
    DocumentCreateArgs, DocumentDeleteArgs, DocumentListArgs, DocumentShowArgs, DocumentTreeArgs,
};
use cmd::git::{GitRepoListArgs, GitRepoShowArgs};
use cmd::issue::attachment::{
    IssueAttachmentDeleteArgs, IssueAttachmentGetArgs, IssueAttachmentListArgs,
};
use cmd::issue::comment::notification::{
    IssueCommentNotificationAddArgs, IssueCommentNotificationListArgs,
};
use cmd::issue::comment::{
    IssueCommentAddArgs, IssueCommentCountArgs, IssueCommentDeleteArgs, IssueCommentListArgs,
    IssueCommentShowArgs, IssueCommentUpdateArgs,
};
use cmd::issue::participant::IssueParticipantListArgs;
use cmd::issue::shared_file::{
    IssueSharedFileLinkArgs, IssueSharedFileListArgs, IssueSharedFileUnlinkArgs,
};
use cmd::issue::{
    IssueCountArgs, IssueCreateArgs, IssueDeleteArgs, IssueListArgs, IssueShowArgs,
    IssueUpdateArgs, ParentChild,
};
use cmd::notification::{NotificationCountArgs, NotificationListArgs, NotificationReadArgs};
use cmd::pr::attachment::{PrAttachmentDeleteArgs, PrAttachmentGetArgs, PrAttachmentListArgs};
use cmd::pr::comment::{
    PrCommentAddArgs, PrCommentCountArgs, PrCommentListArgs, PrCommentUpdateArgs,
};
use cmd::pr::{PrCountArgs, PrCreateArgs, PrListArgs, PrShowArgs, PrUpdateArgs};
use cmd::priority::PriorityListArgs;
use cmd::project::admin::{ProjectAdminAddArgs, ProjectAdminDeleteArgs, ProjectAdminListArgs};
use cmd::project::category::{
    ProjectCategoryAddArgs, ProjectCategoryDeleteArgs, ProjectCategoryListArgs,
    ProjectCategoryUpdateArgs,
};
use cmd::project::custom_field::{
    ProjectCustomFieldAddArgs, ProjectCustomFieldDeleteArgs, ProjectCustomFieldItemAddArgs,
    ProjectCustomFieldItemDeleteArgs, ProjectCustomFieldItemUpdateArgs, ProjectCustomFieldListArgs,
    ProjectCustomFieldUpdateArgs,
};
use cmd::project::issue_type::{
    ProjectIssueTypeAddArgs, ProjectIssueTypeDeleteArgs, ProjectIssueTypeListArgs,
    ProjectIssueTypeUpdateArgs,
};
use cmd::project::status::{
    ProjectStatusAddArgs, ProjectStatusDeleteArgs, ProjectStatusListArgs, ProjectStatusReorderArgs,
    ProjectStatusUpdateArgs,
};
use cmd::project::team::{ProjectTeamAddArgs, ProjectTeamDeleteArgs, ProjectTeamListArgs};
use cmd::project::user::{ProjectUserAddArgs, ProjectUserDeleteArgs, ProjectUserListArgs};
use cmd::project::version::{
    ProjectVersionAddArgs, ProjectVersionDeleteArgs, ProjectVersionListArgs,
    ProjectVersionUpdateArgs,
};
use cmd::project::webhook::{
    ProjectWebhookAddArgs, ProjectWebhookDeleteArgs, ProjectWebhookListArgs,
    ProjectWebhookShowArgs, ProjectWebhookUpdateArgs,
};
use cmd::project::{
    ProjectActivitiesArgs, ProjectCreateArgs, ProjectDeleteArgs, ProjectDiskUsageArgs,
    ProjectListArgs, ProjectShowArgs, ProjectUpdateArgs,
};
use cmd::rate_limit::RateLimitArgs;
use cmd::resolution::ResolutionListArgs;
use cmd::shared_file::{SharedFileGetArgs, SharedFileListArgs};
use cmd::space::{
    SpaceActivitiesArgs, SpaceDiskUsageArgs, SpaceImageArgs, SpaceLicenceArgs,
    SpaceNotificationArgs, SpaceShowArgs, SpaceUpdateNotificationArgs,
};
use cmd::star::{StarAddArgs, StarDeleteArgs};
use cmd::team::{
    TeamAddArgs, TeamDeleteArgs, TeamIconArgs, TeamListArgs, TeamShowArgs, TeamUpdateArgs,
};
use cmd::user::star::{UserStarCountArgs, UserStarListArgs};
use cmd::user::{
    UserActivitiesArgs, UserAddArgs, UserDeleteArgs, UserListArgs, UserRecentlyViewedArgs,
    UserRecentlyViewedProjectsArgs, UserRecentlyViewedWikisArgs, UserShowArgs, UserUpdateArgs,
};
use cmd::watch::{
    WatchAddArgs, WatchCountArgs, WatchDeleteArgs, WatchListArgs, WatchReadArgs, WatchShowArgs,
    WatchUpdateArgs,
};
use cmd::wiki::attachment::{
    WikiAttachmentAddArgs, WikiAttachmentDeleteArgs, WikiAttachmentGetArgs, WikiAttachmentListArgs,
};
use cmd::wiki::shared_file::{
    WikiSharedFileLinkArgs, WikiSharedFileListArgs, WikiSharedFileUnlinkArgs,
};
use cmd::wiki::star::WikiStarListArgs;
use cmd::wiki::tag::WikiTagListArgs;
use cmd::wiki::{
    WikiCountArgs, WikiCreateArgs, WikiDeleteArgs, WikiHistoryArgs, WikiListArgs, WikiShowArgs,
    WikiUpdateArgs,
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
    /// Manage documents
    Document {
        #[command(subcommand)]
        action: DocumentCommands,
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
    /// Manage watchings
    Watch {
        #[command(subcommand)]
        action: WatchCommands,
    },
    /// List priorities
    Priority {
        #[command(subcommand)]
        action: PriorityCommands,
    },
    /// List resolutions
    Resolution {
        #[command(subcommand)]
        action: ResolutionCommands,
    },
    /// Show API rate limit status
    RateLimit {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Manage shared files in a project
    SharedFile {
        #[command(subcommand)]
        action: SharedFileCommands,
    },
    /// Manage stars
    Star {
        #[command(subcommand)]
        action: StarCommands,
    },
    /// Manage Git repositories
    Git {
        #[command(subcommand)]
        action: GitCommands,
    },
    /// Manage pull requests
    Pr {
        #[command(subcommand)]
        action: PrCommands,
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

#[derive(clap::ValueEnum, Clone)]
enum TextFormattingRule {
    Backlog,
    Markdown,
}

impl TextFormattingRule {
    fn as_str(&self) -> &'static str {
        match self {
            TextFormattingRule::Backlog => "backlog",
            TextFormattingRule::Markdown => "markdown",
        }
    }
}

#[derive(clap::ValueEnum, Clone)]
enum RoleType {
    Administrator,
    Normal,
    Reporter,
    Viewer,
    GuestReporter,
    GuestViewer,
}

impl RoleType {
    fn as_u8(&self) -> u8 {
        match self {
            RoleType::Administrator => 1,
            RoleType::Normal => 2,
            RoleType::Reporter => 3,
            RoleType::Viewer => 4,
            RoleType::GuestReporter => 5,
            RoleType::GuestViewer => 6,
        }
    }
}

#[derive(clap::ValueEnum, Clone)]
enum WatchSort {
    Created,
    Updated,
    IssueUpdated,
}

impl WatchSort {
    fn as_str(&self) -> &'static str {
        match self {
            WatchSort::Created => "created",
            WatchSort::Updated => "updated",
            WatchSort::IssueUpdated => "issueUpdated",
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
    /// Show space licence information
    Licence {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Update the space notification message
    UpdateNotification {
        /// Notification content
        #[arg(long)]
        content: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Download the space icon image
    Image {
        /// Output file path (default: server-provided filename)
        #[arg(long, short = 'o')]
        output: Option<std::path::PathBuf>,
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
    /// Create a new project
    Create {
        /// Project name
        #[arg(long)]
        name: String,
        /// Project key (uppercase letters, numbers, underscore; 2–10 chars)
        #[arg(long)]
        key: String,
        /// Enable chart feature
        #[arg(long)]
        chart_enabled: bool,
        /// Enable subtasking
        #[arg(long)]
        subtasking_enabled: bool,
        /// Text formatting rule
        #[arg(long, default_value = "markdown")]
        text_formatting_rule: TextFormattingRule,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Update a project
    Update {
        /// Project ID or key
        id_or_key: String,
        /// New project name
        #[arg(long)]
        name: Option<String>,
        /// New project key
        #[arg(long)]
        key: Option<String>,
        /// Enable or disable chart feature
        #[arg(long)]
        chart_enabled: Option<bool>,
        /// Enable or disable subtasking
        #[arg(long)]
        subtasking_enabled: Option<bool>,
        /// Text formatting rule
        #[arg(long)]
        text_formatting_rule: Option<TextFormattingRule>,
        /// Archive or unarchive the project
        #[arg(long)]
        archived: Option<bool>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Delete a project
    Delete {
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
    /// Manage project administrators
    Admin {
        #[command(subcommand)]
        action: ProjectAdminCommands,
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
    /// Manage project teams
    Team {
        #[command(subcommand)]
        action: ProjectTeamCommands,
    },
    /// Manage project webhooks
    Webhook {
        #[command(subcommand)]
        action: ProjectWebhookCommands,
    },
    /// Manage project custom fields
    CustomField {
        #[command(subcommand)]
        action: ProjectCustomFieldCommands,
    },
}

#[derive(Subcommand)]
enum ProjectWebhookCommands {
    /// List webhooks in a project
    List {
        /// Project ID or key
        id_or_key: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show details of a webhook
    Show {
        /// Project ID or key
        id_or_key: String,
        /// Numeric webhook ID
        webhook_id: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Add a webhook to a project
    Add {
        /// Project ID or key
        id_or_key: String,
        /// Webhook name
        #[arg(long)]
        name: String,
        /// Webhook URL
        #[arg(long)]
        hook_url: String,
        /// Description
        #[arg(long)]
        description: Option<String>,
        /// Trigger on all events
        #[arg(long)]
        all_event: Option<bool>,
        /// Activity type IDs to trigger on (repeatable)
        #[arg(long = "activity-type-id")]
        activity_type_ids: Vec<u64>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Update a webhook
    Update {
        /// Project ID or key
        id_or_key: String,
        /// Numeric webhook ID
        webhook_id: u64,
        /// New webhook name
        #[arg(long)]
        name: Option<String>,
        /// New webhook URL
        #[arg(long)]
        hook_url: Option<String>,
        /// New description
        #[arg(long)]
        description: Option<String>,
        /// Trigger on all events
        #[arg(long)]
        all_event: Option<bool>,
        /// Activity type IDs to trigger on (repeatable; replaces existing list)
        #[arg(long = "activity-type-id")]
        activity_type_ids: Option<Vec<u64>>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Delete a webhook from a project
    Delete {
        /// Project ID or key
        id_or_key: String,
        /// Numeric webhook ID
        webhook_id: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum ProjectTeamCommands {
    /// List teams in a project
    List {
        /// Project ID or key
        id_or_key: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Add a team to a project
    Add {
        /// Project ID or key
        id_or_key: String,
        /// Numeric team ID to add
        #[arg(long)]
        team_id: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Remove a team from a project
    Delete {
        /// Project ID or key
        id_or_key: String,
        /// Numeric team ID to remove
        #[arg(long)]
        team_id: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
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
    /// Add a user to a project
    Add {
        /// Project ID or key
        id_or_key: String,
        /// Numeric user ID to add
        #[arg(long)]
        user_id: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Remove a user from a project
    Delete {
        /// Project ID or key
        id_or_key: String,
        /// Numeric user ID to remove
        #[arg(long)]
        user_id: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum ProjectAdminCommands {
    /// List administrators of a project
    List {
        /// Project ID or key
        id_or_key: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Add a user as project administrator
    Add {
        /// Project ID or key
        id_or_key: String,
        /// Numeric user ID to add as administrator
        #[arg(long)]
        user_id: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Remove a user from project administrators
    Delete {
        /// Project ID or key
        id_or_key: String,
        /// Numeric user ID to remove from administrators
        #[arg(long)]
        user_id: u64,
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
    /// Add a status to a project
    Add {
        /// Project ID or key
        id_or_key: String,
        /// Status name
        #[arg(long)]
        name: String,
        /// Status color (hex code, e.g. #ed8077)
        #[arg(long)]
        color: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Update a project status
    Update {
        /// Project ID or key
        id_or_key: String,
        /// Status ID to update
        #[arg(long)]
        status_id: u64,
        /// New status name
        #[arg(long)]
        name: Option<String>,
        /// New status color (hex code)
        #[arg(long)]
        color: Option<String>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Delete a project status
    Delete {
        /// Project ID or key
        id_or_key: String,
        /// Status ID to delete
        #[arg(long)]
        status_id: u64,
        /// Status ID to substitute for issues with the deleted status
        #[arg(long)]
        substitute_status_id: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Reorder project statuses
    Reorder {
        /// Project ID or key
        id_or_key: String,
        /// Status IDs in desired display order (repeatable)
        #[arg(long = "status-id", value_name = "ID")]
        status_ids: Vec<u64>,
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
    /// Add an issue type to a project
    Add {
        /// Project ID or key
        id_or_key: String,
        /// Issue type name
        #[arg(long)]
        name: String,
        /// Issue type color (hex code, e.g. #e30000)
        #[arg(long)]
        color: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Update a project issue type
    Update {
        /// Project ID or key
        id_or_key: String,
        /// Issue type ID to update
        #[arg(long)]
        issue_type_id: u64,
        /// New issue type name
        #[arg(long)]
        name: Option<String>,
        /// New issue type color (hex code)
        #[arg(long)]
        color: Option<String>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Delete a project issue type
    Delete {
        /// Project ID or key
        id_or_key: String,
        /// Issue type ID to delete
        #[arg(long)]
        issue_type_id: u64,
        /// Issue type ID to substitute for issues with the deleted type
        #[arg(long)]
        substitute_issue_type_id: u64,
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
    /// Add a category to a project
    Add {
        /// Project ID or key
        id_or_key: String,
        /// Category name
        #[arg(long)]
        name: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Update a project category
    Update {
        /// Project ID or key
        id_or_key: String,
        /// Category ID to update
        #[arg(long)]
        category_id: u64,
        /// New category name
        #[arg(long)]
        name: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Delete a project category
    Delete {
        /// Project ID or key
        id_or_key: String,
        /// Category ID to delete
        #[arg(long)]
        category_id: u64,
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
    /// Add a version to a project
    Add {
        /// Project ID or key
        id_or_key: String,
        /// Version name
        #[arg(long)]
        name: String,
        /// Description
        #[arg(long)]
        description: Option<String>,
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: Option<String>,
        /// Release due date (YYYY-MM-DD)
        #[arg(long)]
        release_due_date: Option<String>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Update a version in a project
    Update {
        /// Project ID or key
        id_or_key: String,
        /// Version ID
        #[arg(long)]
        version_id: u64,
        /// Version name
        #[arg(long)]
        name: String,
        /// Description
        #[arg(long)]
        description: Option<String>,
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        start_date: Option<String>,
        /// Release due date (YYYY-MM-DD)
        #[arg(long)]
        release_due_date: Option<String>,
        /// Mark as archived
        #[arg(long)]
        archived: Option<bool>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Delete a version from a project
    Delete {
        /// Project ID or key
        id_or_key: String,
        /// Version ID
        #[arg(long)]
        version_id: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum ProjectCustomFieldCommands {
    /// List custom fields in a project
    List {
        /// Project ID or key
        id_or_key: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Add a custom field to a project
    Add {
        /// Project ID or key
        id_or_key: String,
        /// Field type ID (1=Text, 2=Sentence, 3=Number, 4=Date, 5=SingleList, 6=MultipleList, 7=Checkbox, 8=Radio)
        #[arg(long)]
        type_id: u64,
        /// Field name
        #[arg(long)]
        name: String,
        /// Description
        #[arg(long)]
        description: Option<String>,
        /// Mark as required
        #[arg(long)]
        required: Option<bool>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Update a custom field in a project
    Update {
        /// Project ID or key
        id_or_key: String,
        /// Custom field ID
        #[arg(long)]
        custom_field_id: u64,
        /// Field name
        #[arg(long)]
        name: Option<String>,
        /// Description
        #[arg(long)]
        description: Option<String>,
        /// Mark as required
        #[arg(long)]
        required: Option<bool>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Delete a custom field from a project
    Delete {
        /// Project ID or key
        id_or_key: String,
        /// Custom field ID
        #[arg(long)]
        custom_field_id: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Manage list items of a custom field
    Item {
        #[command(subcommand)]
        action: ProjectCustomFieldItemCommands,
    },
}

#[derive(Subcommand)]
enum ProjectCustomFieldItemCommands {
    /// Add an item to a list custom field
    Add {
        /// Project ID or key
        id_or_key: String,
        /// Custom field ID
        #[arg(long)]
        custom_field_id: u64,
        /// Item name
        #[arg(long)]
        name: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Update an item in a list custom field
    Update {
        /// Project ID or key
        id_or_key: String,
        /// Custom field ID
        #[arg(long)]
        custom_field_id: u64,
        /// Item ID
        #[arg(long)]
        item_id: u64,
        /// Item name
        #[arg(long)]
        name: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Delete an item from a list custom field
    Delete {
        /// Project ID or key
        id_or_key: String,
        /// Custom field ID
        #[arg(long)]
        custom_field_id: u64,
        /// Item ID
        #[arg(long)]
        item_id: u64,
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
    /// List participants of an issue
    Participant {
        #[command(subcommand)]
        action: IssueParticipantCommands,
    },
    /// Manage shared files linked to an issue
    SharedFile {
        #[command(subcommand)]
        action: IssueSharedFileCommands,
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
    /// Count comments on an issue
    Count {
        /// Issue ID or key
        id_or_key: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show a specific comment
    Show {
        /// Issue ID or key
        id_or_key: String,
        /// Comment ID
        comment_id: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Manage comment notifications
    Notification {
        #[command(subcommand)]
        action: IssueCommentNotificationCommands,
    },
}

#[derive(Subcommand)]
enum IssueCommentNotificationCommands {
    /// List notifications for a comment
    List {
        /// Issue ID or key
        id_or_key: String,
        /// Comment ID
        comment_id: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Add notifications for a comment
    Add {
        /// Issue ID or key
        id_or_key: String,
        /// Comment ID
        comment_id: u64,
        /// User ID to notify (repeatable)
        #[arg(long = "notified-user-id", value_name = "ID")]
        notified_user_ids: Vec<u64>,
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
    /// Download an issue attachment
    Get {
        /// Issue ID or key
        id_or_key: String,
        /// Attachment ID
        attachment_id: u64,
        /// Output file path (defaults to original filename)
        #[arg(long, short)]
        output: Option<std::path::PathBuf>,
    },
    /// Delete an issue attachment
    Delete {
        /// Issue ID or key
        id_or_key: String,
        /// Attachment ID
        attachment_id: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum IssueParticipantCommands {
    /// List participants of an issue
    List {
        /// Issue ID or key
        id_or_key: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum IssueSharedFileCommands {
    /// List shared files linked to an issue
    List {
        /// Issue ID or key
        id_or_key: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Link shared files to an issue
    Link {
        /// Issue ID or key
        id_or_key: String,
        /// Shared file ID to link (repeatable)
        #[arg(long = "shared-file-id", value_name = "ID")]
        shared_file_ids: Vec<u64>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Unlink a shared file from an issue
    Unlink {
        /// Issue ID or key
        id_or_key: String,
        /// Shared file ID to unlink
        shared_file_id: u64,
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
    /// Count wiki pages in a project
    Count {
        /// Project ID or key (optional filter)
        project_id_or_key: Option<String>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Manage wiki tags
    Tag {
        #[command(subcommand)]
        action: WikiTagCommands,
    },
    /// List stars on a wiki page
    Star {
        #[command(subcommand)]
        action: WikiStarCommands,
    },
    /// Manage wiki attachments
    Attachment {
        #[command(subcommand)]
        action: WikiAttachmentCommands,
    },
    /// Manage shared files linked to a wiki page
    SharedFile {
        #[command(subcommand)]
        action: WikiSharedFileCommands,
    },
}

#[derive(Subcommand)]
enum WikiTagCommands {
    /// List tags used in wiki pages
    List {
        /// Project ID or key (optional filter)
        project_id_or_key: Option<String>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum WikiStarCommands {
    /// List stars on a wiki page
    List {
        /// Wiki page ID
        wiki_id: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
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
    /// Add attachments to a wiki page (by pre-uploaded attachment ID)
    Add {
        /// Wiki page ID
        wiki_id: u64,
        /// Attachment ID (repeatable)
        #[arg(long = "attachment-id", value_name = "ID")]
        attachment_ids: Vec<u64>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Download a wiki attachment
    Get {
        /// Wiki page ID
        wiki_id: u64,
        /// Attachment ID
        attachment_id: u64,
        /// Output file path (defaults to original filename)
        #[arg(long)]
        output: Option<std::path::PathBuf>,
    },
    /// Delete an attachment from a wiki page
    Delete {
        /// Wiki page ID
        wiki_id: u64,
        /// Attachment ID
        attachment_id: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum WikiSharedFileCommands {
    /// List shared files linked to a wiki page
    List {
        /// Wiki page ID
        wiki_id: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Link shared files to a wiki page
    Link {
        /// Wiki page ID
        wiki_id: u64,
        /// Shared file ID (repeatable)
        #[arg(long = "shared-file-id", value_name = "ID")]
        shared_file_ids: Vec<u64>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Unlink a shared file from a wiki page
    Unlink {
        /// Wiki page ID
        wiki_id: u64,
        /// Shared file ID
        shared_file_id: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum DocumentCommands {
    /// List documents
    List {
        /// Filter by project ID (repeatable)
        #[arg(long = "project-id", value_name = "ID")]
        project_ids: Vec<u64>,
        /// Search keyword
        #[arg(long)]
        keyword: Option<String>,
        /// Sort field (e.g. created, updated)
        #[arg(long)]
        sort: Option<String>,
        /// Sort order
        #[arg(long)]
        order: Option<Order>,
        /// Number of documents to retrieve
        #[arg(long)]
        count: Option<u32>,
        /// Offset for pagination
        #[arg(long)]
        offset: Option<u64>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show document tree for a project
    Tree {
        /// Project ID or key
        project_id_or_key: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show a document
    Show {
        /// Document ID
        document_id: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Create a document
    Create {
        /// Project ID
        #[arg(long)]
        project_id: u64,
        /// Document title
        #[arg(long)]
        title: Option<String>,
        /// Document content
        #[arg(long)]
        content: Option<String>,
        /// Emoji for the document icon
        #[arg(long)]
        emoji: Option<String>,
        /// Parent document ID
        #[arg(long)]
        parent_id: Option<String>,
        /// Add document at the end of the list
        #[arg(long)]
        add_last: bool,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Delete a document
    Delete {
        /// Document ID
        document_id: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Manage document attachments
    Attachment {
        #[command(subcommand)]
        action: DocumentAttachmentCommands,
    },
}

#[derive(Subcommand)]
enum DocumentAttachmentCommands {
    /// Download a document attachment
    Get {
        /// Document ID
        document_id: String,
        /// Attachment ID
        attachment_id: u64,
        /// Output file path (defaults to original filename)
        #[arg(long, short)]
        output: Option<std::path::PathBuf>,
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
    /// Add a new user
    Add {
        /// User ID (login name)
        #[arg(long)]
        user_id: String,
        /// Password
        #[arg(long)]
        password: String,
        /// Display name
        #[arg(long)]
        name: String,
        /// Email address
        #[arg(long)]
        mail_address: String,
        /// Role type
        #[arg(long)]
        role_type: RoleType,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Update a user
    Update {
        /// User numeric ID
        id: u64,
        /// Display name
        #[arg(long)]
        name: Option<String>,
        /// Password
        #[arg(long)]
        password: Option<String>,
        /// Email address
        #[arg(long)]
        mail_address: Option<String>,
        /// Role type
        #[arg(long)]
        role_type: Option<RoleType>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Delete a user
    Delete {
        /// User numeric ID
        id: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show recently viewed projects (for the authenticated user)
    RecentlyViewedProjects {
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
    /// Show recently viewed wikis (for the authenticated user)
    RecentlyViewedWikis {
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
    /// Manage user stars
    Star {
        #[command(subcommand)]
        action: UserStarCommands,
    },
}

#[derive(Subcommand)]
enum UserStarCommands {
    /// List stars of a user
    List {
        /// User numeric ID
        id: u64,
        /// Minimum star ID
        #[arg(long)]
        min_id: Option<u64>,
        /// Maximum star ID
        #[arg(long)]
        max_id: Option<u64>,
        /// Number of stars to retrieve
        #[arg(long, default_value = "20")]
        count: u32,
        /// Sort order
        #[arg(long)]
        order: Option<Order>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Count stars of a user
    Count {
        /// User numeric ID
        id: u64,
        /// Start date (YYYY-MM-DD)
        #[arg(long)]
        since: Option<String>,
        /// End date (YYYY-MM-DD)
        #[arg(long)]
        until: Option<String>,
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
    /// Create a new team
    Add {
        /// Team name
        #[arg(long)]
        name: String,
        /// Member user IDs to add (repeatable)
        #[arg(long = "member")]
        members: Vec<u64>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Update a team
    Update {
        /// Team numeric ID
        id: u64,
        /// New team name
        #[arg(long)]
        name: Option<String>,
        /// Replace member list with these user IDs (repeatable)
        #[arg(long = "member")]
        members: Vec<u64>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Delete a team
    Delete {
        /// Team numeric ID
        id: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Download team icon
    Icon {
        /// Team numeric ID
        id: u64,
        /// Output file path (default: server-provided filename)
        #[arg(long, short = 'o')]
        output: Option<std::path::PathBuf>,
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
enum WatchCommands {
    /// List watchings for a user
    List {
        /// User ID
        user_id: u64,
        /// Sort order
        #[arg(long)]
        order: Option<Order>,
        /// Sort field
        #[arg(long)]
        sort: Option<WatchSort>,
        /// Number of watchings to retrieve (1–100)
        #[arg(long, default_value = "20")]
        count: u32,
        /// Offset
        #[arg(long)]
        offset: Option<u64>,
        /// Filter by read status (true = read only, false = unread only)
        #[arg(long)]
        resource_already_read: Option<bool>,
        /// Filter by issue ID (repeatable)
        #[arg(long = "issue-id", value_name = "ID")]
        issue_ids: Vec<u64>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Count watchings for a user
    Count {
        /// User ID
        user_id: u64,
        /// Filter by resource read status
        #[arg(long)]
        resource_already_read: Option<bool>,
        /// Filter by already-read status (takes precedence over --resource-already-read)
        #[arg(long)]
        already_read: Option<bool>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show a watching
    Show {
        /// Watching ID
        id: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Add a watching
    Add {
        /// Issue ID or key to watch
        #[arg(long)]
        issue: String,
        /// Note
        #[arg(long)]
        note: Option<String>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Update a watching note
    Update {
        /// Watching ID
        id: u64,
        /// New note
        #[arg(long)]
        note: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Delete a watching
    Delete {
        /// Watching ID
        id: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Mark a watching as read
    Read {
        /// Watching ID
        id: u64,
    },
}

#[derive(Subcommand)]
enum PriorityCommands {
    /// List priorities
    List {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum ResolutionCommands {
    /// List resolutions
    List {
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum SharedFileCommands {
    /// List shared files in a project directory
    List {
        /// Project ID or key
        id_or_key: String,
        /// Directory path (default: root)
        #[arg(long)]
        path: Option<String>,
        /// Number of files to retrieve (1–100)
        #[arg(long, default_value = "20")]
        count: u32,
        /// Sort order
        #[arg(long)]
        order: Option<Order>,
        /// Offset for pagination
        #[arg(long)]
        offset: Option<u64>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Download a shared file
    Get {
        /// Project ID or key
        id_or_key: String,
        /// Shared file ID
        id: u64,
        /// Save path (default: original filename)
        #[arg(long, short)]
        output: Option<std::path::PathBuf>,
    },
}

#[derive(Subcommand)]
enum StarCommands {
    /// Add a star to an issue, comment, wiki page, pull request, or pull request comment
    Add {
        /// ID of the issue to star
        #[arg(long)]
        issue_id: Option<u64>,
        /// ID of the issue comment to star
        #[arg(long)]
        comment_id: Option<u64>,
        /// ID of the wiki page to star
        #[arg(long)]
        wiki_id: Option<u64>,
        /// ID of the pull request to star
        #[arg(long)]
        pull_request_id: Option<u64>,
        /// ID of the pull request comment to star
        #[arg(long)]
        pull_request_comment_id: Option<u64>,
    },
    /// Remove a star
    Delete {
        /// Star ID
        id: u64,
    },
}

#[derive(Subcommand)]
enum GitCommands {
    /// List Git repositories in a project
    Repo {
        #[command(subcommand)]
        action: GitRepoCommands,
    },
}

#[derive(Subcommand)]
enum GitRepoCommands {
    /// List Git repositories in a project
    List {
        /// Project ID or key
        project_id_or_key: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show a Git repository
    Show {
        /// Project ID or key
        project_id_or_key: String,
        /// Repository ID or name
        repo_id_or_name: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum PrCommands {
    /// List pull requests in a repository
    List {
        /// Project ID or key
        project_id_or_key: String,
        /// Repository ID or name
        repo_id_or_name: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Count pull requests in a repository
    Count {
        /// Project ID or key
        project_id_or_key: String,
        /// Repository ID or name
        repo_id_or_name: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Show a pull request
    Show {
        /// Project ID or key
        project_id_or_key: String,
        /// Repository ID or name
        repo_id_or_name: String,
        /// Pull request number
        number: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Create a pull request
    Create {
        /// Project ID or key
        project_id_or_key: String,
        /// Repository ID or name
        repo_id_or_name: String,
        /// Pull request summary
        #[arg(long)]
        summary: String,
        /// Pull request description
        #[arg(long)]
        description: Option<String>,
        /// Base branch (merge target)
        #[arg(long)]
        base: String,
        /// Source branch
        #[arg(long)]
        branch: String,
        /// Linked issue ID
        #[arg(long)]
        issue_id: Option<u64>,
        /// Assignee user ID
        #[arg(long)]
        assignee_id: Option<u64>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Update a pull request
    Update {
        /// Project ID or key
        project_id_or_key: String,
        /// Repository ID or name
        repo_id_or_name: String,
        /// Pull request number
        number: u64,
        /// New summary
        #[arg(long)]
        summary: Option<String>,
        /// New description
        #[arg(long)]
        description: Option<String>,
        /// New base branch
        #[arg(long)]
        base: Option<String>,
        /// Linked issue ID
        #[arg(long)]
        issue_id: Option<u64>,
        /// Assignee user ID
        #[arg(long)]
        assignee_id: Option<u64>,
        /// Comment to add when updating
        #[arg(long)]
        comment: Option<String>,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Manage pull request comments
    Comment {
        #[command(subcommand)]
        action: PrCommentCommands,
    },
    /// Manage pull request attachments
    Attachment {
        #[command(subcommand)]
        action: PrAttachmentCommands,
    },
}

#[derive(Subcommand)]
enum PrCommentCommands {
    /// List comments on a pull request
    List {
        /// Project ID or key
        project_id_or_key: String,
        /// Repository ID or name
        repo_id_or_name: String,
        /// Pull request number
        number: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Count comments on a pull request
    Count {
        /// Project ID or key
        project_id_or_key: String,
        /// Repository ID or name
        repo_id_or_name: String,
        /// Pull request number
        number: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Add a comment to a pull request
    Add {
        /// Project ID or key
        project_id_or_key: String,
        /// Repository ID or name
        repo_id_or_name: String,
        /// Pull request number
        number: u64,
        /// Comment content
        #[arg(long)]
        content: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Update a comment on a pull request
    Update {
        /// Project ID or key
        project_id_or_key: String,
        /// Repository ID or name
        repo_id_or_name: String,
        /// Pull request number
        number: u64,
        /// Comment ID
        comment_id: u64,
        /// New comment content
        #[arg(long)]
        content: String,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}

#[derive(Subcommand)]
enum PrAttachmentCommands {
    /// List attachments of a pull request
    List {
        /// Project ID or key
        project_id_or_key: String,
        /// Repository ID or name
        repo_id_or_name: String,
        /// Pull request number
        number: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
    /// Download an attachment from a pull request
    Get {
        /// Project ID or key
        project_id_or_key: String,
        /// Repository ID or name
        repo_id_or_name: String,
        /// Pull request number
        number: u64,
        /// Attachment ID
        attachment_id: u64,
        /// Output file path
        #[arg(long, short)]
        output: Option<std::path::PathBuf>,
    },
    /// Delete an attachment from a pull request
    Delete {
        /// Project ID or key
        project_id_or_key: String,
        /// Repository ID or name
        repo_id_or_name: String,
        /// Pull request number
        number: u64,
        /// Attachment ID
        attachment_id: u64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
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
            ProjectCommands::Create {
                name,
                key,
                chart_enabled,
                subtasking_enabled,
                text_formatting_rule,
                json,
            } => cmd::project::create(&ProjectCreateArgs::new(
                name,
                key,
                chart_enabled,
                subtasking_enabled,
                text_formatting_rule.as_str().to_string(),
                json,
            )),
            ProjectCommands::Update {
                id_or_key,
                name,
                key,
                chart_enabled,
                subtasking_enabled,
                text_formatting_rule,
                archived,
                json,
            } => cmd::project::update(&ProjectUpdateArgs::try_new(
                id_or_key,
                name,
                key,
                chart_enabled,
                subtasking_enabled,
                text_formatting_rule.map(|r| r.as_str().to_string()),
                archived,
                json,
            )?),
            ProjectCommands::Delete { id_or_key, json } => {
                cmd::project::delete(&ProjectDeleteArgs::new(id_or_key, json))
            }
            ProjectCommands::User { action } => match action {
                ProjectUserCommands::List { id_or_key, json } => {
                    cmd::project::user::list(&ProjectUserListArgs::new(id_or_key, json))
                }
                ProjectUserCommands::Add {
                    id_or_key,
                    user_id,
                    json,
                } => cmd::project::user::add(&ProjectUserAddArgs::new(id_or_key, user_id, json)),
                ProjectUserCommands::Delete {
                    id_or_key,
                    user_id,
                    json,
                } => cmd::project::user::delete(&ProjectUserDeleteArgs::new(
                    id_or_key, user_id, json,
                )),
            },
            ProjectCommands::Admin { action } => match action {
                ProjectAdminCommands::List { id_or_key, json } => {
                    cmd::project::admin::list(&ProjectAdminListArgs::new(id_or_key, json))
                }
                ProjectAdminCommands::Add {
                    id_or_key,
                    user_id,
                    json,
                } => cmd::project::admin::add(&ProjectAdminAddArgs::new(id_or_key, user_id, json)),
                ProjectAdminCommands::Delete {
                    id_or_key,
                    user_id,
                    json,
                } => cmd::project::admin::delete(&ProjectAdminDeleteArgs::new(
                    id_or_key, user_id, json,
                )),
            },
            ProjectCommands::Status { action } => match action {
                ProjectStatusCommands::List { id_or_key, json } => {
                    cmd::project::status::list(&ProjectStatusListArgs::new(id_or_key, json))
                }
                ProjectStatusCommands::Add {
                    id_or_key,
                    name,
                    color,
                    json,
                } => cmd::project::status::add(&ProjectStatusAddArgs::try_new(
                    id_or_key, name, color, json,
                )?),
                ProjectStatusCommands::Update {
                    id_or_key,
                    status_id,
                    name,
                    color,
                    json,
                } => cmd::project::status::update(&ProjectStatusUpdateArgs::try_new(
                    id_or_key, status_id, name, color, json,
                )?),
                ProjectStatusCommands::Delete {
                    id_or_key,
                    status_id,
                    substitute_status_id,
                    json,
                } => cmd::project::status::delete(&ProjectStatusDeleteArgs::try_new(
                    id_or_key,
                    status_id,
                    substitute_status_id,
                    json,
                )?),
                ProjectStatusCommands::Reorder {
                    id_or_key,
                    status_ids,
                    json,
                } => cmd::project::status::reorder(&ProjectStatusReorderArgs::try_new(
                    id_or_key, status_ids, json,
                )?),
            },
            ProjectCommands::IssueType { action } => match action {
                ProjectIssueTypeCommands::List { id_or_key, json } => {
                    cmd::project::issue_type::list(&ProjectIssueTypeListArgs::new(id_or_key, json))
                }
                ProjectIssueTypeCommands::Add {
                    id_or_key,
                    name,
                    color,
                    json,
                } => cmd::project::issue_type::add(&ProjectIssueTypeAddArgs::try_new(
                    id_or_key, name, color, json,
                )?),
                ProjectIssueTypeCommands::Update {
                    id_or_key,
                    issue_type_id,
                    name,
                    color,
                    json,
                } => cmd::project::issue_type::update(&ProjectIssueTypeUpdateArgs::try_new(
                    id_or_key,
                    issue_type_id,
                    name,
                    color,
                    json,
                )?),
                ProjectIssueTypeCommands::Delete {
                    id_or_key,
                    issue_type_id,
                    substitute_issue_type_id,
                    json,
                } => cmd::project::issue_type::delete(&ProjectIssueTypeDeleteArgs::try_new(
                    id_or_key,
                    issue_type_id,
                    substitute_issue_type_id,
                    json,
                )?),
            },
            ProjectCommands::Category { action } => match action {
                ProjectCategoryCommands::List { id_or_key, json } => {
                    cmd::project::category::list(&ProjectCategoryListArgs::new(id_or_key, json))
                }
                ProjectCategoryCommands::Add {
                    id_or_key,
                    name,
                    json,
                } => {
                    cmd::project::category::add(&ProjectCategoryAddArgs::new(id_or_key, name, json))
                }
                ProjectCategoryCommands::Update {
                    id_or_key,
                    category_id,
                    name,
                    json,
                } => cmd::project::category::update(&ProjectCategoryUpdateArgs::new(
                    id_or_key,
                    category_id,
                    name,
                    json,
                )),
                ProjectCategoryCommands::Delete {
                    id_or_key,
                    category_id,
                    json,
                } => cmd::project::category::delete(&ProjectCategoryDeleteArgs::new(
                    id_or_key,
                    category_id,
                    json,
                )),
            },
            ProjectCommands::Version { action } => match action {
                ProjectVersionCommands::List { id_or_key, json } => {
                    cmd::project::version::list(&ProjectVersionListArgs::new(id_or_key, json))
                }
                ProjectVersionCommands::Add {
                    id_or_key,
                    name,
                    description,
                    start_date,
                    release_due_date,
                    json,
                } => cmd::project::version::add(&ProjectVersionAddArgs::new(
                    id_or_key,
                    name,
                    description,
                    start_date,
                    release_due_date,
                    json,
                )),
                ProjectVersionCommands::Update {
                    id_or_key,
                    version_id,
                    name,
                    description,
                    start_date,
                    release_due_date,
                    archived,
                    json,
                } => cmd::project::version::update(&ProjectVersionUpdateArgs::new(
                    id_or_key,
                    version_id,
                    name,
                    description,
                    start_date,
                    release_due_date,
                    archived,
                    json,
                )),
                ProjectVersionCommands::Delete {
                    id_or_key,
                    version_id,
                    json,
                } => cmd::project::version::delete(&ProjectVersionDeleteArgs::new(
                    id_or_key, version_id, json,
                )),
            },
            ProjectCommands::Team { action } => match action {
                ProjectTeamCommands::List { id_or_key, json } => {
                    cmd::project::team::list(&ProjectTeamListArgs::new(id_or_key, json))
                }
                ProjectTeamCommands::Add {
                    id_or_key,
                    team_id,
                    json,
                } => cmd::project::team::add(&ProjectTeamAddArgs::new(id_or_key, team_id, json)),
                ProjectTeamCommands::Delete {
                    id_or_key,
                    team_id,
                    json,
                } => cmd::project::team::delete(&ProjectTeamDeleteArgs::new(
                    id_or_key, team_id, json,
                )),
            },
            ProjectCommands::Webhook { action } => match action {
                ProjectWebhookCommands::List { id_or_key, json } => {
                    cmd::project::webhook::list(&ProjectWebhookListArgs::new(id_or_key, json))
                }
                ProjectWebhookCommands::Show {
                    id_or_key,
                    webhook_id,
                    json,
                } => cmd::project::webhook::show(&ProjectWebhookShowArgs::new(
                    id_or_key, webhook_id, json,
                )),
                ProjectWebhookCommands::Add {
                    id_or_key,
                    name,
                    hook_url,
                    description,
                    all_event,
                    activity_type_ids,
                    json,
                } => cmd::project::webhook::add(&ProjectWebhookAddArgs::new(
                    id_or_key,
                    name,
                    hook_url,
                    description,
                    all_event,
                    activity_type_ids,
                    json,
                )),
                ProjectWebhookCommands::Update {
                    id_or_key,
                    webhook_id,
                    name,
                    hook_url,
                    description,
                    all_event,
                    activity_type_ids,
                    json,
                } => cmd::project::webhook::update(&ProjectWebhookUpdateArgs::try_new(
                    id_or_key,
                    webhook_id,
                    name,
                    hook_url,
                    description,
                    all_event,
                    activity_type_ids,
                    json,
                )?),
                ProjectWebhookCommands::Delete {
                    id_or_key,
                    webhook_id,
                    json,
                } => cmd::project::webhook::delete(&ProjectWebhookDeleteArgs::new(
                    id_or_key, webhook_id, json,
                )),
            },
            ProjectCommands::CustomField { action } => match action {
                ProjectCustomFieldCommands::List { id_or_key, json } => {
                    cmd::project::custom_field::list(&ProjectCustomFieldListArgs::new(
                        id_or_key, json,
                    ))
                }
                ProjectCustomFieldCommands::Add {
                    id_or_key,
                    type_id,
                    name,
                    description,
                    required,
                    json,
                } => cmd::project::custom_field::add(&ProjectCustomFieldAddArgs::new(
                    id_or_key,
                    type_id,
                    name,
                    description,
                    required,
                    json,
                )),
                ProjectCustomFieldCommands::Update {
                    id_or_key,
                    custom_field_id,
                    name,
                    description,
                    required,
                    json,
                } => cmd::project::custom_field::update(&ProjectCustomFieldUpdateArgs::try_new(
                    id_or_key,
                    custom_field_id,
                    name,
                    description,
                    required,
                    json,
                )?),
                ProjectCustomFieldCommands::Delete {
                    id_or_key,
                    custom_field_id,
                    json,
                } => cmd::project::custom_field::delete(&ProjectCustomFieldDeleteArgs::new(
                    id_or_key,
                    custom_field_id,
                    json,
                )),
                ProjectCustomFieldCommands::Item { action } => match action {
                    ProjectCustomFieldItemCommands::Add {
                        id_or_key,
                        custom_field_id,
                        name,
                        json,
                    } => cmd::project::custom_field::item_add(&ProjectCustomFieldItemAddArgs::new(
                        id_or_key,
                        custom_field_id,
                        name,
                        json,
                    )),
                    ProjectCustomFieldItemCommands::Update {
                        id_or_key,
                        custom_field_id,
                        item_id,
                        name,
                        json,
                    } => cmd::project::custom_field::item_update(
                        &ProjectCustomFieldItemUpdateArgs::new(
                            id_or_key,
                            custom_field_id,
                            item_id,
                            name,
                            json,
                        ),
                    ),
                    ProjectCustomFieldItemCommands::Delete {
                        id_or_key,
                        custom_field_id,
                        item_id,
                        json,
                    } => cmd::project::custom_field::item_delete(
                        &ProjectCustomFieldItemDeleteArgs::new(
                            id_or_key,
                            custom_field_id,
                            item_id,
                            json,
                        ),
                    ),
                },
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
                IssueCommentCommands::Count { id_or_key, json } => {
                    cmd::issue::comment::count(&IssueCommentCountArgs::new(id_or_key, json))
                }
                IssueCommentCommands::Show {
                    id_or_key,
                    comment_id,
                    json,
                } => cmd::issue::comment::show(&IssueCommentShowArgs::new(
                    id_or_key, comment_id, json,
                )),
                IssueCommentCommands::Notification { action } => match action {
                    IssueCommentNotificationCommands::List {
                        id_or_key,
                        comment_id,
                        json,
                    } => cmd::issue::comment::notification::list(
                        &IssueCommentNotificationListArgs::new(id_or_key, comment_id, json),
                    ),
                    IssueCommentNotificationCommands::Add {
                        id_or_key,
                        comment_id,
                        notified_user_ids,
                        json,
                    } => cmd::issue::comment::notification::add(
                        &IssueCommentNotificationAddArgs::try_new(
                            id_or_key,
                            comment_id,
                            notified_user_ids,
                            json,
                        )?,
                    ),
                },
            },
            IssueCommands::Attachment { action } => match action {
                IssueAttachmentCommands::List { id_or_key, json } => {
                    cmd::issue::attachment::list(&IssueAttachmentListArgs::new(id_or_key, json))
                }
                IssueAttachmentCommands::Get {
                    id_or_key,
                    attachment_id,
                    output,
                } => cmd::issue::attachment::get(&IssueAttachmentGetArgs::new(
                    id_or_key,
                    attachment_id,
                    output,
                )),
                IssueAttachmentCommands::Delete {
                    id_or_key,
                    attachment_id,
                    json,
                } => cmd::issue::attachment::delete(&IssueAttachmentDeleteArgs::new(
                    id_or_key,
                    attachment_id,
                    json,
                )),
            },
            IssueCommands::Participant { action } => match action {
                IssueParticipantCommands::List { id_or_key, json } => {
                    cmd::issue::participant::list(&IssueParticipantListArgs::new(id_or_key, json))
                }
            },
            IssueCommands::SharedFile { action } => match action {
                IssueSharedFileCommands::List { id_or_key, json } => {
                    cmd::issue::shared_file::list(&IssueSharedFileListArgs::new(id_or_key, json))
                }
                IssueSharedFileCommands::Link {
                    id_or_key,
                    shared_file_ids,
                    json,
                } => cmd::issue::shared_file::link(&IssueSharedFileLinkArgs::try_new(
                    id_or_key,
                    shared_file_ids,
                    json,
                )?),
                IssueSharedFileCommands::Unlink {
                    id_or_key,
                    shared_file_id,
                    json,
                } => cmd::issue::shared_file::unlink(&IssueSharedFileUnlinkArgs::new(
                    id_or_key,
                    shared_file_id,
                    json,
                )),
            },
        },
        Commands::Document { action } => match action {
            DocumentCommands::List {
                project_ids,
                keyword,
                sort,
                order,
                count,
                offset,
                json,
            } => cmd::document::list(&DocumentListArgs::new(
                project_ids,
                keyword,
                sort,
                order.map(|o| o.as_str().to_string()),
                count,
                offset,
                json,
            )),
            DocumentCommands::Tree {
                project_id_or_key,
                json,
            } => cmd::document::tree(&DocumentTreeArgs::new(project_id_or_key, json)),
            DocumentCommands::Show { document_id, json } => {
                cmd::document::show(&DocumentShowArgs::new(document_id, json))
            }
            DocumentCommands::Create {
                project_id,
                title,
                content,
                emoji,
                parent_id,
                add_last,
                json,
            } => cmd::document::create(&DocumentCreateArgs::new(
                project_id, title, content, emoji, parent_id, add_last, json,
            )),
            DocumentCommands::Delete { document_id, json } => {
                cmd::document::delete(&DocumentDeleteArgs::new(document_id, json))
            }
            DocumentCommands::Attachment { action } => match action {
                DocumentAttachmentCommands::Get {
                    document_id,
                    attachment_id,
                    output,
                } => cmd::document::attachment::get(&DocumentAttachmentGetArgs::new(
                    document_id,
                    attachment_id,
                    output,
                )),
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
            WikiCommands::Count {
                project_id_or_key,
                json,
            } => cmd::wiki::count(&WikiCountArgs::new(project_id_or_key, json)),
            WikiCommands::Tag { action } => match action {
                WikiTagCommands::List {
                    project_id_or_key,
                    json,
                } => cmd::wiki::tag::list(&WikiTagListArgs::new(project_id_or_key, json)),
            },
            WikiCommands::Star { action } => match action {
                WikiStarCommands::List { wiki_id, json } => {
                    cmd::wiki::star::list(&WikiStarListArgs::new(wiki_id, json))
                }
            },
            WikiCommands::Attachment { action } => match action {
                WikiAttachmentCommands::List { wiki_id, json } => {
                    cmd::wiki::attachment::list(&WikiAttachmentListArgs::new(wiki_id, json))
                }
                WikiAttachmentCommands::Add {
                    wiki_id,
                    attachment_ids,
                    json,
                } => cmd::wiki::attachment::add(&WikiAttachmentAddArgs::try_new(
                    wiki_id,
                    attachment_ids,
                    json,
                )?),
                WikiAttachmentCommands::Get {
                    wiki_id,
                    attachment_id,
                    output,
                } => cmd::wiki::attachment::get(&WikiAttachmentGetArgs::new(
                    wiki_id,
                    attachment_id,
                    output,
                )),
                WikiAttachmentCommands::Delete {
                    wiki_id,
                    attachment_id,
                    json,
                } => cmd::wiki::attachment::delete(&WikiAttachmentDeleteArgs::new(
                    wiki_id,
                    attachment_id,
                    json,
                )),
            },
            WikiCommands::SharedFile { action } => match action {
                WikiSharedFileCommands::List { wiki_id, json } => {
                    cmd::wiki::shared_file::list(&WikiSharedFileListArgs::new(wiki_id, json))
                }
                WikiSharedFileCommands::Link {
                    wiki_id,
                    shared_file_ids,
                    json,
                } => cmd::wiki::shared_file::link(&WikiSharedFileLinkArgs::try_new(
                    wiki_id,
                    shared_file_ids,
                    json,
                )?),
                WikiSharedFileCommands::Unlink {
                    wiki_id,
                    shared_file_id,
                    json,
                } => cmd::wiki::shared_file::unlink(&WikiSharedFileUnlinkArgs::new(
                    wiki_id,
                    shared_file_id,
                    json,
                )),
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
            UserCommands::Add {
                user_id,
                password,
                name,
                mail_address,
                role_type,
                json,
            } => cmd::user::add(&UserAddArgs::new(
                user_id,
                password,
                name,
                mail_address,
                role_type.as_u8(),
                json,
            )),
            UserCommands::Update {
                id,
                name,
                password,
                mail_address,
                role_type,
                json,
            } => cmd::user::update(&UserUpdateArgs::try_new(
                id,
                name,
                password,
                mail_address,
                role_type.map(|r| r.as_u8()),
                json,
            )?),
            UserCommands::Delete { id, json } => cmd::user::delete(&UserDeleteArgs::new(id, json)),
            UserCommands::RecentlyViewedProjects {
                count,
                offset,
                order,
                json,
            } => cmd::user::recently_viewed_projects(&UserRecentlyViewedProjectsArgs::try_new(
                json,
                count,
                offset,
                order.map(|o| o.as_str().to_string()),
            )?),
            UserCommands::RecentlyViewedWikis {
                count,
                offset,
                order,
                json,
            } => cmd::user::recently_viewed_wikis(&UserRecentlyViewedWikisArgs::try_new(
                json,
                count,
                offset,
                order.map(|o| o.as_str().to_string()),
            )?),
            UserCommands::Star { action } => match action {
                UserStarCommands::List {
                    id,
                    min_id,
                    max_id,
                    count,
                    order,
                    json,
                } => cmd::user::star::list(&UserStarListArgs::try_new(
                    id,
                    min_id,
                    max_id,
                    count,
                    order.map(|o| o.as_str().to_string()),
                    json,
                )?),
                UserStarCommands::Count {
                    id,
                    since,
                    until,
                    json,
                } => cmd::user::star::count(&UserStarCountArgs::new(id, since, until, json)),
            },
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
                members,
                json,
            } => cmd::team::add(&TeamAddArgs::new(name, members, json)),
            TeamCommands::Update {
                id,
                name,
                members,
                json,
            } => {
                let members = if members.is_empty() {
                    None
                } else {
                    Some(members)
                };
                cmd::team::update(&TeamUpdateArgs::try_new(id, name, members, json)?)
            }
            TeamCommands::Delete { id, json } => cmd::team::delete(&TeamDeleteArgs::new(id, json)),
            TeamCommands::Icon { id, output } => cmd::team::icon(&TeamIconArgs::new(id, output)),
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
        Commands::Watch { action } => match action {
            WatchCommands::List {
                user_id,
                order,
                sort,
                count,
                offset,
                resource_already_read,
                issue_ids,
                json,
            } => cmd::watch::list(&WatchListArgs::try_new(
                user_id,
                order.map(|o| o.as_str().to_string()),
                sort.map(|s| s.as_str().to_string()),
                count,
                offset,
                resource_already_read,
                issue_ids,
                json,
            )?),
            WatchCommands::Count {
                user_id,
                resource_already_read,
                already_read,
                json,
            } => cmd::watch::count(&WatchCountArgs::new(
                user_id,
                resource_already_read,
                already_read,
                json,
            )),
            WatchCommands::Show { id, json } => cmd::watch::show(&WatchShowArgs::new(id, json)),
            WatchCommands::Add { issue, note, json } => {
                cmd::watch::add(&WatchAddArgs::new(issue, note, json))
            }
            WatchCommands::Update { id, note, json } => {
                cmd::watch::update(&WatchUpdateArgs::new(id, note, json))
            }
            WatchCommands::Delete { id, json } => {
                cmd::watch::delete(&WatchDeleteArgs::new(id, json))
            }
            WatchCommands::Read { id } => cmd::watch::read(&WatchReadArgs::new(id)),
        },
        Commands::Priority { action } => match action {
            PriorityCommands::List { json } => cmd::priority::list(&PriorityListArgs::new(json)),
        },
        Commands::Resolution { action } => match action {
            ResolutionCommands::List { json } => {
                cmd::resolution::list(&ResolutionListArgs::new(json))
            }
        },
        Commands::RateLimit { json } => cmd::rate_limit::show(&RateLimitArgs::new(json)),
        Commands::SharedFile { action } => match action {
            SharedFileCommands::List {
                id_or_key,
                path,
                count,
                order,
                offset,
                json,
            } => cmd::shared_file::list(&SharedFileListArgs::try_new(
                id_or_key,
                path,
                count,
                order.map(|o| o.as_str().to_string()),
                offset,
                json,
            )?),
            SharedFileCommands::Get {
                id_or_key,
                id,
                output,
            } => cmd::shared_file::get(&SharedFileGetArgs::new(id_or_key, id, output)),
        },
        Commands::Star { action } => match action {
            StarCommands::Add {
                issue_id,
                comment_id,
                wiki_id,
                pull_request_id,
                pull_request_comment_id,
            } => cmd::star::add(&StarAddArgs::try_new(
                issue_id,
                comment_id,
                wiki_id,
                pull_request_id,
                pull_request_comment_id,
            )?),
            StarCommands::Delete { id } => cmd::star::delete(&StarDeleteArgs::new(id)),
        },
        Commands::Git { action } => match action {
            GitCommands::Repo { action } => match action {
                GitRepoCommands::List {
                    project_id_or_key,
                    json,
                } => cmd::git::list(&GitRepoListArgs::new(project_id_or_key, json)),
                GitRepoCommands::Show {
                    project_id_or_key,
                    repo_id_or_name,
                    json,
                } => cmd::git::show(&GitRepoShowArgs::new(
                    project_id_or_key,
                    repo_id_or_name,
                    json,
                )),
            },
        },
        Commands::Pr { action } => match action {
            PrCommands::List {
                project_id_or_key,
                repo_id_or_name,
                json,
            } => cmd::pr::list(&PrListArgs::new(project_id_or_key, repo_id_or_name, json)),
            PrCommands::Count {
                project_id_or_key,
                repo_id_or_name,
                json,
            } => cmd::pr::count(&PrCountArgs::new(project_id_or_key, repo_id_or_name, json)),
            PrCommands::Show {
                project_id_or_key,
                repo_id_or_name,
                number,
                json,
            } => cmd::pr::show(&PrShowArgs::new(
                project_id_or_key,
                repo_id_or_name,
                number,
                json,
            )),
            PrCommands::Create {
                project_id_or_key,
                repo_id_or_name,
                summary,
                description,
                base,
                branch,
                issue_id,
                assignee_id,
                json,
            } => cmd::pr::create(&PrCreateArgs::new(
                project_id_or_key,
                repo_id_or_name,
                summary,
                description,
                base,
                branch,
                issue_id,
                assignee_id,
                json,
            )),
            PrCommands::Update {
                project_id_or_key,
                repo_id_or_name,
                number,
                summary,
                description,
                base,
                issue_id,
                assignee_id,
                comment,
                json,
            } => cmd::pr::update(&PrUpdateArgs::try_new(
                project_id_or_key,
                repo_id_or_name,
                number,
                summary,
                description,
                base,
                issue_id,
                assignee_id,
                comment,
                json,
            )?),
            PrCommands::Comment { action } => match action {
                PrCommentCommands::List {
                    project_id_or_key,
                    repo_id_or_name,
                    number,
                    json,
                } => cmd::pr::comment::list(&PrCommentListArgs::new(
                    project_id_or_key,
                    repo_id_or_name,
                    number,
                    json,
                )),
                PrCommentCommands::Count {
                    project_id_or_key,
                    repo_id_or_name,
                    number,
                    json,
                } => cmd::pr::comment::count(&PrCommentCountArgs::new(
                    project_id_or_key,
                    repo_id_or_name,
                    number,
                    json,
                )),
                PrCommentCommands::Add {
                    project_id_or_key,
                    repo_id_or_name,
                    number,
                    content,
                    json,
                } => cmd::pr::comment::add(&PrCommentAddArgs::new(
                    project_id_or_key,
                    repo_id_or_name,
                    number,
                    content,
                    json,
                )),
                PrCommentCommands::Update {
                    project_id_or_key,
                    repo_id_or_name,
                    number,
                    comment_id,
                    content,
                    json,
                } => cmd::pr::comment::update(&PrCommentUpdateArgs::new(
                    project_id_or_key,
                    repo_id_or_name,
                    number,
                    comment_id,
                    content,
                    json,
                )),
            },
            PrCommands::Attachment { action } => match action {
                PrAttachmentCommands::List {
                    project_id_or_key,
                    repo_id_or_name,
                    number,
                    json,
                } => cmd::pr::attachment::list(&PrAttachmentListArgs::new(
                    project_id_or_key,
                    repo_id_or_name,
                    number,
                    json,
                )),
                PrAttachmentCommands::Get {
                    project_id_or_key,
                    repo_id_or_name,
                    number,
                    attachment_id,
                    output,
                } => cmd::pr::attachment::get(&PrAttachmentGetArgs::new(
                    project_id_or_key,
                    repo_id_or_name,
                    number,
                    attachment_id,
                    output,
                )),
                PrAttachmentCommands::Delete {
                    project_id_or_key,
                    repo_id_or_name,
                    number,
                    attachment_id,
                    json,
                } => cmd::pr::attachment::delete(&PrAttachmentDeleteArgs::new(
                    project_id_or_key,
                    repo_id_or_name,
                    number,
                    attachment_id,
                    json,
                )),
            },
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
            Some(SpaceCommands::Licence { json: sub_json }) => {
                cmd::space::licence(&SpaceLicenceArgs::new(json || sub_json))
            }
            Some(SpaceCommands::UpdateNotification {
                content,
                json: sub_json,
            }) => cmd::space::update_notification(&SpaceUpdateNotificationArgs::new(
                content,
                json || sub_json,
            )),
            Some(SpaceCommands::Image { output }) => {
                cmd::space::image(&SpaceImageArgs::new(output))
            }
        },
    }
}
