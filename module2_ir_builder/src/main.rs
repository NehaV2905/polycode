mod api;
mod graph;
mod grpc_client;
mod ir;

use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::{error, info};
use tracing_subscriber;

use api::GraphQuery;
use grpc_client::IREventClient;

#[derive(Parser)]
#[command(name = "ir-builder")]
#[command(about = "Module 2: IR Definition & Builder", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Connect to Module 1 and build the IR graph
    Connect {
        /// Module 1 gRPC server address
        #[arg(short, long, default_value = "http://127.0.0.1:50051")]
        server: String,

        /// File to monitor
        #[arg(short, long)]
        file: String,

        /// Programming language
        #[arg(short, long, default_value = "python")]
        language: String,
    },

    /// Query the graph (requires pre-built graph)
    Query {
        /// Type of query
        #[command(subcommand)]
        query_type: QueryType,
    },

    /// Export graph to JSON
    Export {
        /// Output file path
        #[arg(short, long)]
        output: String,
    },
}

#[derive(Subcommand)]
enum QueryType {
    /// Find all callers of a function
    Callers {
        #[arg(short, long)]
        function: String,

        #[arg(short = 'p', long)]
        file_path: String,
    },

    /// Find all callees of a function
    Callees {
        #[arg(short, long)]
        function: String,

        #[arg(short = 'p', long)]
        file_path: String,
    },

    /// Find dependencies of a file
    Dependencies {
        #[arg(short = 'p', long)]
        file_path: String,
    },

    /// Find unused functions in a file
    Unused {
        #[arg(short = 'p', long)]
        file_path: String,
    },

    /// Get graph statistics
    Stats,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Connect {
            server,
            file,
            language,
        } => {
            info!("Starting IR Builder - connecting to Module 1");
            connect_and_build(&server, &file, &language).await?;
        }

        Commands::Query { query_type } => {
            error!("Query command not yet implemented");
            error!("You need to first build a graph using the 'connect' command");
            // TODO: Implement persistent graph storage and querying
        }

        Commands::Export { output } => {
            error!("Export command not yet implemented");
            error!("You need to first build a graph using the 'connect' command");
            // TODO: Implement graph export
        }
    }

    Ok(())
}

async fn connect_and_build(server: &str, file: &str, language: &str) -> Result<()> {
    info!("Connecting to Module 1 at {}", server);

    // Connect to Module 1
    let mut client = IREventClient::connect(server.to_string()).await?;

    // Monitor file and build graph
    client.monitor_file(file.to_string(), language.to_string()).await?;

    // Get the built graph
    let graph = client.into_graph();

    // Display statistics
    let stats = graph.stats();
    info!("\n=== Graph Build Complete ===");
    info!("Total nodes: {}", stats.node_count);
    info!("Total edges: {}", stats.edge_count);
    info!("Total files: {}", stats.file_count);

    // Example queries
    info!("\n=== Example Queries ===");
    let query = GraphQuery::new(&graph);

    // List all functions
    let functions = query.get_functions(file);
    info!("Functions found: {}", functions.len());
    for func in functions.iter().take(10) {
        info!("  - {} (line {})", func.name, func.line_number);
    }

    // Find unused functions
    let unused = query.find_unused_functions(file);
    if !unused.is_empty() {
        info!("\nUnused functions: {}", unused.len());
        for func in unused.iter().take(5) {
            info!("  - {} (line {})", func.name, func.line_number);
        }
    }

    // Find dependencies
    let deps = query.find_dependencies(file);
    if !deps.is_empty() {
        info!("\nDependencies: {}", deps.len());
        for dep in deps.iter().take(5) {
            info!("  - {} (imports: {:?})", dep.module_path, dep.imported_names);
        }
    }

    Ok(())
}
