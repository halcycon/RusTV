# RusTV - NDI Matrix Viewer

A professional NDI (Network Device Interface) matrix viewer and router implemented in Rust, with support for BirdDog camera automation and a powerful GUI for matrix visualization.

## Features

### GUI Application
- **Matrix View Display**: Visual representation of all inputs and outputs
- **Customizable Layouts**: Multiple layout options including:
  - 2x2 Grid (4 views)
  - 3x3 Grid (9 views)
  - 4x4 Grid (16 views)
  - Picture in Picture (PiP)
  - 1+7 Layout (1 main + 7 small views) - main view in top-left, smaller views on right and bottom edges
  - 1+9 Layout (1 main + 9 small views) - main view in top-left, smaller views on right and bottom edges
- **Interactive Routing**: Click-to-route interface for easy source assignment
- **Real-time Source Discovery**: Automatically discover and list available NDI sources
- **Visual Feedback**: See active routes and available inputs at a glance
- **Configurable Window Size**: Set default window dimensions in configuration

### NDI Integration
- **Automatic Source Discovery**: Continuously discover NDI sources on your network
- **Stream Viewing**: View NDI streams with support for video, audio, and metadata
- **Static Source Configuration**: Define static NDI sources in configuration

### Matrix Routing
- **Input/Output Routing**: Route any NDI input to any defined output
- **Dynamic Routing**: Change routes on-the-fly via CLI or GUI
- **Persistent Configuration**: Save and load routing configurations

### BirdDog Camera Integration
- **API Compatibility**: Full compatibility with BirdDog camera APIs
- **PTZ Control**: Pan, Tilt, Zoom control with absolute and relative positioning
- **Preset Management**: Save and recall camera presets
- **Camera Status**: Monitor camera status, temperature, and streaming state
- **Focus Control**: Manual focus control and auto-focus support

### Companion Integration
- **Streamdeck Control**: Interface with Companion software for enhanced streamdeck functionality
- **Layout Control**: Change layouts via Companion buttons
- **Route Management**: Create and remove routes from streamdeck
- **Feedback**: Get current state feedback for button updates
- **Configurable**: Enable/disable and configure Companion connection settings

## Installation

### Prerequisites
- NDI SDK installed on your system

### Download Pre-built Binaries

Download the latest release for your platform from the [Releases page](https://github.com/halcycon/RusTV/releases):

- **Linux x86_64**: `rustv-linux-x86_64.tar.gz`
- **Linux ARM64**: `rustv-linux-aarch64.tar.gz`
- **macOS Intel**: `rustv-macos-x86_64.tar.gz`
- **macOS Apple Silicon**: `rustv-macos-aarch64.tar.gz`
- **Windows x86_64**: `rustv-windows-x86_64.exe.zip`

Each release includes SHA256 checksums for verification.

**Example (Linux x86_64)**:
```bash
# Download and extract
wget https://github.com/halcycon/RusTV/releases/download/v1.0.0/rustv-linux-x86_64.tar.gz
tar xzf rustv-linux-x86_64.tar.gz

# Verify checksum (optional)
wget https://github.com/halcycon/RusTV/releases/download/v1.0.0/rustv-linux-x86_64.tar.gz.sha256
sha256sum -c rustv-linux-x86_64.tar.gz.sha256

# Make executable and move to PATH
chmod +x rustv
sudo mv rustv /usr/local/bin/
```

### Building from Source

Requirements: Rust 1.70 or later

```bash
git clone https://github.com/halcycon/RusTV.git
cd RusTV
cargo build --release
```

The compiled binary will be at `target/release/rustv`

## Usage

### GUI Application

Start the graphical interface (default when no command is specified):

```bash
rustv gui
# or simply
rustv
```

The GUI provides:
- **Matrix View**: Visual grid showing all outputs and their assigned inputs
- **Layout Selection**: Choose from different view layouts (2x2, 3x3, 4x4, PiP, 1+7, 1+9)
- **Routing Control**: 
  1. Click "🔄 Refresh Sources" to discover available NDI sources
  2. Select a source from the list
  3. Click on a view slot in the matrix
  4. Click "➡ Route Selected" to create the route
- **Route Management**: View and remove active routes

### Initialize Configuration

Generate a default configuration file:

```bash
rustv init-config
```

This creates `rustv.toml` with default settings.

### NDI Discovery

Start NDI source discovery:

```bash
# One-time discovery
rustv discover

# Continuous discovery mode
rustv discover --continuous
```

### View NDI Source

View a specific NDI source:

```bash
rustv view "SOURCE_NAME"
```

### Matrix Routing

#### List Outputs
```bash
rustv matrix outputs
```

#### List Current Routes
```bash
rustv matrix list
```

#### Create a Route
```bash
rustv matrix route "ndi://camera1" "Monitor 1"
```

#### Remove a Route
```bash
rustv matrix unroute "Monitor 1"
```

### BirdDog Camera Control

#### Get Camera Information
```bash
rustv bird-dog 192.168.1.100 info
```

#### Get Camera Status
```bash
rustv bird-dog 192.168.1.100 status
```

#### Get PTZ Position
```bash
rustv bird-dog 192.168.1.100 position
```

#### Move Camera to Home
```bash
rustv bird-dog 192.168.1.100 home
```

#### Move Camera to Position
```bash
rustv bird-dog 192.168.1.100 move --pan 0.5 --tilt 0.3 --zoom 0.8
```

Pan and tilt values range from -1.0 to 1.0, zoom from 0.0 to 1.0.

#### Recall Preset
```bash
rustv bird-dog 192.168.1.100 preset 1
```

### Companion Integration

Control RusTV via Companion software:

#### Test Connection
```bash
rustv companion test
```

#### Change Layout
```bash
rustv companion set-layout "1+7 Layout"
```

#### Create Route
```bash
rustv companion route "Camera 1" "Monitor 1"
```

#### Remove Route
```bash
rustv companion unroute "Monitor 1"
```

#### Get Feedback
```bash
rustv companion feedback
```

## Configuration

The `rustv.toml` configuration file supports the following options:

```toml
[ndi]
# Enable automatic NDI source discovery
auto_discovery = true
# Discovery interval in seconds
discovery_interval = 5
# Static NDI sources (optional)
static_sources = []

[matrix]
# Define output destinations
outputs = [
    "Monitor 1",
    "Monitor 2",
    "Monitor 3",
    "Monitor 4",
]
# Saved routes
routes = []

[birddog]
# BirdDog camera configurations
cameras = []

[gui]
# Default layout to use on startup
# Options: "Grid2x2", "Grid3x3", "Grid4x4", "PiP", "OneAndSeven", "OneAndNine"
default_layout = "Grid2x2"
# Window dimensions
window_width = 1280.0
window_height = 720.0

[companion]
# Enable Companion integration for streamdeck control
enabled = false
host = "localhost"
port = 8888
```

### Example Configuration with BirdDog Cameras and Companion

```toml
[ndi]
auto_discovery = true
discovery_interval = 5
static_sources = []

[matrix]
outputs = [
    "Monitor 1",
    "Monitor 2",
    "Preview",
    "Program",
]
routes = []

[birddog]
cameras = [
    { name = "Camera 1", ip_address = "192.168.1.101", ndi_name = "BirdDog-CAM1" },
    { name = "Camera 2", ip_address = "192.168.1.102", ndi_name = "BirdDog-CAM2" },
]

[gui]
default_layout = "OneAndSeven"
window_width = 1920.0
window_height = 1080.0

[companion]
enabled = true
host = "localhost"
port = 8888
```

## Architecture

The application is structured into several modules:

- **ndi**: NDI source discovery, receiver, and stream handling
- **matrix**: Matrix routing logic for input/output management
- **birddog**: BirdDog camera API client and PTZ control
- **config**: Configuration management

## Development

### Running Tests

```bash
cargo test
```

### Running with Debug Logging

```bash
RUST_LOG=debug rustv discover
```

Available log levels: `error`, `warn`, `info`, `debug`, `trace`

## API Integration

### BirdDog Camera API

The application implements the BirdDog camera REST API:

- `/api/camera/info` - Camera information
- `/api/camera/status` - Current camera status
- `/api/ptz/position` - Current PTZ position
- `/api/ptz/control` - PTZ control commands

Commands supported:
- Absolute positioning (pan, tilt, zoom)
- Relative movement
- Stop movement
- Home position
- Preset save/recall
- Focus control
- Auto-focus

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- NDI® is a registered trademark of Vizrt Group
- BirdDog camera integration based on BirdDog API specifications
