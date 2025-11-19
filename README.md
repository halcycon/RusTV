# RusTV - NDI Matrix Viewer

A professional NDI (Network Device Interface) matrix viewer and router implemented in Rust, with support for BirdDog camera automation.

## Features

### NDI Integration
- **Automatic Source Discovery**: Continuously discover NDI sources on your network
- **Stream Viewing**: View NDI streams with support for video, audio, and metadata
- **Static Source Configuration**: Define static NDI sources in configuration

### Matrix Routing
- **Input/Output Routing**: Route any NDI input to any defined output
- **Dynamic Routing**: Change routes on-the-fly via CLI
- **Persistent Configuration**: Save and load routing configurations

### BirdDog Camera Integration
- **API Compatibility**: Full compatibility with BirdDog camera APIs
- **PTZ Control**: Pan, Tilt, Zoom control with absolute and relative positioning
- **Preset Management**: Save and recall camera presets
- **Camera Status**: Monitor camera status, temperature, and streaming state
- **Focus Control**: Manual focus control and auto-focus support

## Installation

### Prerequisites
- Rust 1.70 or later
- NDI SDK installed on your system

### Building from Source

```bash
git clone https://github.com/halcycon/RusTV.git
cd RusTV
cargo build --release
```

The compiled binary will be at `target/release/rustv`

## Usage

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
```

### Example Configuration with BirdDog Cameras

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
