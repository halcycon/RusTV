mod birddog;
mod config;
mod matrix;
mod ndi;

use anyhow::Result;
use birddog::{BirdDogClient, PtzPosition};
use clap::{Parser, Subcommand};
use config::Config;
use log::{error, info};
use matrix::MatrixRouter;
use ndi::{NdiDiscovery, NdiReceiver, NdiSource};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "rustv")]
#[command(about = "NDI Matrix Viewer with BirdDog camera integration", long_about = None)]
struct Cli {
    /// Configuration file path
    #[arg(short, long, default_value = "rustv.toml")]
    config: PathBuf,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the NDI discovery service
    Discover {
        /// Run in continuous mode
        #[arg(short, long)]
        continuous: bool,
    },
    /// View an NDI source
    View {
        /// Source name or URL
        source: String,
    },
    /// Matrix routing commands
    Matrix {
        #[command(subcommand)]
        action: MatrixAction,
    },
    /// BirdDog camera control
    BirdDog {
        /// Camera IP address
        camera_ip: String,
        #[command(subcommand)]
        action: BirdDogAction,
    },
    /// Generate default configuration file
    InitConfig,
}

#[derive(Subcommand)]
enum MatrixAction {
    /// List all routes
    List,
    /// Create a route
    Route {
        /// Input source
        input: String,
        /// Output destination
        output: String,
    },
    /// Remove a route
    Unroute {
        /// Output destination
        output: String,
    },
    /// List all inputs
    Inputs,
    /// List all outputs
    Outputs,
}

#[derive(Subcommand)]
enum BirdDogAction {
    /// Get camera information
    Info,
    /// Get camera status
    Status,
    /// Get current PTZ position
    Position,
    /// Move to home position
    Home,
    /// Move camera (pan, tilt, zoom)
    Move {
        #[arg(long)]
        pan: f64,
        #[arg(long)]
        tilt: f64,
        #[arg(long)]
        zoom: f64,
    },
    /// Recall a preset
    Preset {
        /// Preset number (1-255)
        id: u8,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let cli = Cli::parse();
    
    // Load or create configuration
    let config = Config::ensure_default_config(&cli.config)?;
    info!("Configuration loaded from: {:?}", cli.config);

    match cli.command {
        Some(Commands::Discover { continuous }) => {
            cmd_discover(continuous).await?;
        }
        Some(Commands::View { source }) => {
            cmd_view(&source).await?;
        }
        Some(Commands::Matrix { action }) => {
            cmd_matrix(action, &config).await?;
        }
        Some(Commands::BirdDog { camera_ip, action }) => {
            cmd_birddog(&camera_ip, action).await?;
        }
        Some(Commands::InitConfig) => {
            config.to_file(&cli.config)?;
            info!("Configuration file created at: {:?}", cli.config);
        }
        None => {
            // Default: start interactive mode
            info!("RusTV - NDI Matrix Viewer");
            info!("Use --help for available commands");
            cmd_discover(false).await?;
        }
    }

    Ok(())
}

async fn cmd_discover(continuous: bool) -> Result<()> {
    info!("Starting NDI source discovery...");
    let discovery = NdiDiscovery::new();
    discovery.start().await?;

    if continuous {
        info!("Running in continuous mode. Press Ctrl+C to stop.");
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            let sources = discovery.get_sources();
            info!("Found {} NDI sources:", sources.len());
            for source in sources {
                println!("  - {}", source);
            }
        }
    } else {
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        let sources = discovery.get_sources();
        info!("Found {} NDI sources:", sources.len());
        for source in sources {
            println!("  - {}", source);
        }
        discovery.stop();
    }

    Ok(())
}

async fn cmd_view(source_name: &str) -> Result<()> {
    info!("Viewing NDI source: {}", source_name);
    
    let mut receiver = NdiReceiver::new();
    let source = NdiSource::new(source_name.to_string(), format!("ndi://{}", source_name));
    
    receiver.connect(source)?;
    info!("Connected to source. Press Ctrl+C to stop.");
    
    // Simulate receiving frames
    loop {
        tokio::time::sleep(tokio::time::Duration::from_millis(33)).await; // ~30fps
        if let Err(e) = receiver.receive_video_frame() {
            error!("Error receiving frame: {}", e);
            break;
        }
    }
    
    receiver.disconnect();
    Ok(())
}

async fn cmd_matrix(action: MatrixAction, config: &Config) -> Result<()> {
    let mut router = MatrixRouter::new();
    
    // Initialize with config
    for output in &config.matrix.outputs {
        router.add_output(output.clone());
    }
    
    match action {
        MatrixAction::List => {
            let routes = router.get_all_routes();
            info!("Current routes:");
            for route in routes {
                println!("  {} -> {}", route.input, route.output);
            }
        }
        MatrixAction::Route { input, output } => {
            router.route(&input, &output)?;
            info!("Route created: {} -> {}", input, output);
        }
        MatrixAction::Unroute { output } => {
            if let Some(input) = router.unroute(&output) {
                info!("Route removed: {} -> {}", input, output);
            } else {
                info!("No route found for output: {}", output);
            }
        }
        MatrixAction::Inputs => {
            let inputs = router.get_inputs();
            info!("Available inputs:");
            for input in inputs {
                println!("  - {}", input);
            }
        }
        MatrixAction::Outputs => {
            let outputs = router.get_outputs();
            info!("Available outputs:");
            for output in outputs {
                println!("  - {}", output);
            }
        }
    }
    
    Ok(())
}

async fn cmd_birddog(camera_ip: &str, action: BirdDogAction) -> Result<()> {
    let client = BirdDogClient::new(camera_ip);
    
    match action {
        BirdDogAction::Info => {
            let info = client.get_info().await?;
            println!("Camera Information:");
            println!("  Model: {}", info.model);
            println!("  Firmware: {}", info.firmware_version);
            println!("  Serial: {}", info.serial_number);
        }
        BirdDogAction::Status => {
            let status = client.get_status().await?;
            println!("Camera Status:");
            println!("  Online: {}", status.online);
            println!("  Recording: {}", status.recording);
            println!("  Streaming: {}", status.streaming);
            println!("  Temperature: {}°C", status.temperature);
        }
        BirdDogAction::Position => {
            let position = client.get_ptz_position().await?;
            println!("PTZ Position:");
            println!("  Pan: {}", position.pan);
            println!("  Tilt: {}", position.tilt);
            println!("  Zoom: {}", position.zoom);
        }
        BirdDogAction::Home => {
            client.home().await?;
            info!("Camera moved to home position");
        }
        BirdDogAction::Move { pan, tilt, zoom } => {
            let position = PtzPosition::new(pan, tilt, zoom);
            client.move_absolute(position).await?;
            info!("Camera moved to position: pan={}, tilt={}, zoom={}", pan, tilt, zoom);
        }
        BirdDogAction::Preset { id } => {
            client.recall_preset(id).await?;
            info!("Recalled preset {}", id);
        }
    }
    
    Ok(())
}

