use clap::{Parser, Subcommand};

// CLI commands init
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// CLI commands declaration
#[derive(Subcommand)]
enum Commands {
    #[command(about = "Check health of the server API, settings, etc.")]
    Doctor,
    #[command(about = "Manage URLs")]
    Url {
        #[command(subcommand)]
        command: UrlCommands,
    },
    Tags,
    Visits,
    #[command(about = "Manage configured domains")]
    Domains {
        #[command(subcommand)]
        command: DomainCommands,
    },
    #[command(about = "Config settings")]
    Config,
}

#[derive(Subcommand)]
enum UrlCommands {
    #[command(about = "List all URLs")]
    List,
    #[command(about = "Get a single URL")]
    Get {
        #[arg(short, long)] // the slug to use
        slug: String,
    },
    #[command(about = "Create a new URL")]
    Create {
        #[arg(short, long)] // the url to shorten
        url: String,

        #[arg(short, long)] // the slug to use
        slug: Option<String>,

        #[arg(short, long)] // the tags to use
        tags: Option<Vec<String>>,
    },
    #[command(about = "Update an existing URL")]
    Update {
        #[arg(short, long)] // the slug to use
        slug: String,

        #[arg(short, long)] // the url to use
        url: Option<String>,

        #[arg(short, long)] // the tags to use
        tags: Option<Vec<String>>,
    },
    #[command(about = "Delete an existing URL")]
    Delete {
        #[arg(short, long)] // the slug to use
        slug: String,
    },
}

#[derive(Subcommand)]
enum DomainCommands {
    #[command(about = "List all domains")]
    List,
    Set {
        #[arg(short, long)]
        domain: String,
        #[arg(short, long)]
        redirect: String,
    },
}
