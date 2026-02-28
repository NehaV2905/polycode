pub mod event_processor;

use anyhow::{Context, Result};
use tokio_stream::StreamExt;
use tonic::Request;
use tracing::{debug, error, info};

use crate::graph::GraphBuilder;

// Include the generated protobuf code
pub mod ir_events {
    tonic::include_proto!("ir");
}

use ir_events::{ir_event_stream_client::IrEventStreamClient, MonitorFileRequest};

/// Client for receiving IR events from Module 1
pub struct IREventClient {
    /// gRPC client
    client: IrEventStreamClient<tonic::transport::Channel>,

    /// Graph builder to process events
    builder: GraphBuilder,
}

impl IREventClient {
    /// Connect to Module 1's gRPC server
    pub async fn connect(server_addr: String) -> Result<Self> {
        info!("Connecting to Module 1 at {}", server_addr);

        let client = IrEventStreamClient::connect(server_addr)
            .await
            .context("Failed to connect to Module 1 gRPC server")?;

        info!("Connected to Module 1 successfully");

        Ok(Self {
            client,
            builder: GraphBuilder::new(),
        })
    }

    /// Stream events for a specific file and build the graph
    pub async fn monitor_file(&mut self, file_path: String, language: String) -> Result<()> {
        info!("Monitoring file: {} ({})", file_path, language);

        // Clear existing nodes first (incremental update), then set file context
        // so the fresh module node survives into event processing.
        self.builder.set_current_file(file_path.clone(), language.clone());
        self.builder.clear_current_file()?;
        self.builder.set_current_file(file_path.clone(), language.clone());

        // Create request
        let request = Request::new(MonitorFileRequest {
            file_path: file_path.clone(),
            language: language.clone(),
        });

        // Get event stream
        let mut stream = self
            .client
            .stream_events(request)
            .await
            .context("Failed to start event stream")?
            .into_inner();

        // Process events
        let mut event_count = 0;
        while let Some(event_result) = stream.next().await {
            match event_result {
                Ok(event) => {
                    event_count += 1;
                    if let Err(e) = event_processor::process_event(&mut self.builder, event) {
                        error!("Failed to process event: {}", e);
                    }
                }
                Err(e) => {
                    error!("Error receiving event: {}", e);
                    break;
                }
            }
        }

        // Resolve pending function calls
        self.builder.resolve_pending_calls()?;

        info!(
            "Processed {} events for file: {}",
            event_count, file_path
        );

        // Print graph stats
        let stats = self.builder.graph().stats();
        info!(
            "Graph stats: {} nodes, {} edges, {} files",
            stats.node_count, stats.edge_count, stats.file_count
        );

        Ok(())
    }

    /// Get a reference to the graph builder
    pub fn builder(&self) -> &GraphBuilder {
        &self.builder
    }

    /// Get a mutable reference to the graph builder
    pub fn builder_mut(&mut self) -> &mut GraphBuilder {
        &mut self.builder
    }

    /// Consume the client and return the built graph
    pub fn into_graph(self) -> crate::graph::IRGraph {
        self.builder.into_graph()
    }
}
