# RusTV Usage Examples

This document provides practical examples for using RusTV in various scenarios.

## Getting Started

### 1. Initialize Configuration

First, create a default configuration file:

```bash
rustv init-config
```

This creates `rustv.toml` with default settings.

## NDI Discovery Examples

### Example 1: Quick Discovery Scan

Discover NDI sources on your network for 10 seconds:

```bash
rustv discover
```

### Example 2: Continuous Monitoring

Monitor NDI sources continuously (press Ctrl+C to stop):

```bash
rustv discover --continuous
```

Example output:
```
Found 3 NDI sources:
  - NDI Source: BirdDog Studio (ndi://192.168.1.101:5960)
  - NDI Source: PTZ Camera 1 (ndi://192.168.1.102:5960)
  - NDI Source: Capture Card (ndi://192.168.1.103:5960)
```

## Viewing NDI Sources

### Example 3: View a Specific Source

Connect to and view an NDI source:

```bash
rustv view "BirdDog Studio"
```

The application will continuously receive and process frames until stopped with Ctrl+C.

## Matrix Routing Examples

### Example 4: List Available Outputs

View all configured output destinations:

```bash
rustv matrix outputs
```

Example output:
```
Available outputs:
  - Monitor 1
  - Monitor 2
  - Monitor 3
  - Monitor 4
```

### Example 5: Create a Simple Route

Route an NDI source to an output:

```bash
rustv matrix route "ndi://192.168.1.101:5960" "Monitor 1"
```

### Example 6: View Current Routes

List all active routes:

```bash
rustv matrix list
```

Example output:
```
Current routes:
  ndi://192.168.1.101:5960 -> Monitor 1
  ndi://192.168.1.102:5960 -> Monitor 2
```

### Example 7: Remove a Route

Unroute an output:

```bash
rustv matrix unroute "Monitor 1"
```

## BirdDog Camera Control Examples

### Example 8: Get Camera Information

Query camera details:

```bash
rustv bird-dog 192.168.1.101 info
```

Example output:
```
Camera Information:
  Model: BirdDog P400
  Firmware: 1.2.3
  Serial: BD-P400-12345
```

### Example 9: Check Camera Status

Monitor camera operational status:

```bash
rustv bird-dog 192.168.1.101 status
```

Example output:
```
Camera Status:
  Online: true
  Recording: false
  Streaming: true
  Temperature: 42.5°C
```

### Example 10: Get Current PTZ Position

Query the current pan/tilt/zoom position:

```bash
rustv bird-dog 192.168.1.101 position
```

Example output:
```
PTZ Position:
  Pan: 0.35
  Tilt: -0.12
  Zoom: 0.68
```

### Example 11: Move Camera to Home Position

Reset camera to home position:

```bash
rustv bird-dog 192.168.1.101 home
```

### Example 12: Move Camera to Specific Position

Move camera with specific pan, tilt, and zoom values:

```bash
# Look slightly right and up with medium zoom
rustv bird-dog 192.168.1.101 move --pan 0.3 --tilt 0.2 --zoom 0.5
```

Parameter ranges:
- Pan: -1.0 (full left) to 1.0 (full right)
- Tilt: -1.0 (full down) to 1.0 (full up)
- Zoom: 0.0 (wide) to 1.0 (telephoto)

### Example 13: Recall a Camera Preset

Move camera to a saved preset position:

```bash
# Recall preset 1
rustv bird-dog 192.168.1.101 preset 1
```

## Advanced Workflows

### Workflow 1: Multi-Camera Studio Setup

1. Configure multiple cameras in `rustv.toml`:

```toml
[birddog]
cameras = [
    { name = "Camera 1", ip_address = "192.168.1.101", ndi_name = "BirdDog-CAM1" },
    { name = "Camera 2", ip_address = "192.168.1.102", ndi_name = "BirdDog-CAM2" },
    { name = "Camera 3", ip_address = "192.168.1.103", ndi_name = "BirdDog-CAM3" },
]
```

2. Discover available sources:

```bash
rustv discover
```

3. Set up routing for a multi-monitor setup:

```bash
rustv matrix route "BirdDog-CAM1" "Monitor 1"
rustv matrix route "BirdDog-CAM2" "Monitor 2"
rustv matrix route "BirdDog-CAM3" "Monitor 3"
```

### Workflow 2: Automated Camera Movements

Create a script to automate camera movements:

```bash
#!/bin/bash
CAMERA_IP="192.168.1.101"

# Position 1: Wide shot
rustv bird-dog $CAMERA_IP move --pan 0 --tilt 0 --zoom 0.2
sleep 5

# Position 2: Close-up right
rustv bird-dog $CAMERA_IP move --pan 0.5 --tilt 0.1 --zoom 0.8
sleep 5

# Position 3: Close-up left
rustv bird-dog $CAMERA_IP move --pan -0.5 --tilt 0.1 --zoom 0.8
sleep 5

# Return to home
rustv bird-dog $CAMERA_IP home
```

### Workflow 3: Monitoring Multiple Cameras

Monitor the status of multiple cameras:

```bash
#!/bin/bash
CAMERAS=("192.168.1.101" "192.168.1.102" "192.168.1.103")

for camera in "${CAMERAS[@]}"; do
    echo "=== Camera $camera ==="
    rustv bird-dog $camera status
    echo ""
done
```

## Configuration Examples

### Example Configuration 1: Basic Setup

```toml
[ndi]
auto_discovery = true
discovery_interval = 5
static_sources = []

[matrix]
outputs = ["Monitor 1", "Monitor 2"]
routes = []

[birddog]
cameras = []
```

### Example Configuration 2: Production Setup

```toml
[ndi]
auto_discovery = true
discovery_interval = 10
static_sources = [
    { name = "Main Camera", url = "ndi://192.168.1.100:5960" },
    { name = "Backup Camera", url = "ndi://192.168.1.101:5960" },
]

[matrix]
outputs = [
    "Program Monitor",
    "Preview Monitor",
    "Multiview 1",
    "Multiview 2",
    "Multiview 3",
    "Multiview 4",
    "Recording Output",
    "Streaming Output",
]
routes = []

[birddog]
cameras = [
    { name = "Main PTZ", ip_address = "192.168.1.100", ndi_name = "BirdDog-Main" },
    { name = "Stage Left", ip_address = "192.168.1.101", ndi_name = "BirdDog-Left" },
    { name = "Stage Right", ip_address = "192.168.1.102", ndi_name = "BirdDog-Right" },
    { name = "Audience", ip_address = "192.168.1.103", ndi_name = "BirdDog-Audience" },
]
```

## Troubleshooting Examples

### Enable Debug Logging

For troubleshooting, enable debug logging:

```bash
RUST_LOG=debug rustv discover
```

Available log levels:
- `error`: Only errors
- `warn`: Warnings and errors
- `info`: General information (default)
- `debug`: Detailed debug information
- `trace`: Very detailed trace information

### Example: Debugging NDI Discovery

```bash
RUST_LOG=debug rustv discover --continuous
```

### Example: Debugging BirdDog Communication

```bash
RUST_LOG=debug rustv bird-dog 192.168.1.101 status
```

## Integration Examples

### Example: Using with Shell Scripts

Create a script to automate common tasks:

```bash
#!/bin/bash
# auto-route.sh - Automatically route discovered sources

# Discover sources
echo "Discovering NDI sources..."
rustv discover

# Set up default routing
echo "Setting up default routes..."
rustv matrix route "ndi://camera1" "Monitor 1"
rustv matrix route "ndi://camera2" "Monitor 2"

# Position cameras
echo "Positioning cameras..."
rustv bird-dog 192.168.1.101 home
rustv bird-dog 192.168.1.102 preset 1

echo "Setup complete!"
```

### Example: Cron Job for Monitoring

Monitor camera status every hour:

```bash
# Add to crontab
0 * * * * /usr/local/bin/rustv bird-dog 192.168.1.101 status >> /var/log/camera-status.log 2>&1
```

## Performance Tips

1. **Discovery Interval**: For production, increase discovery interval in config:
   ```toml
   discovery_interval = 30  # Check every 30 seconds instead of 5
   ```

2. **Static Sources**: For known sources, use static configuration to avoid constant discovery:
   ```toml
   static_sources = [
       { name = "Camera 1", url = "ndi://192.168.1.100:5960" },
   ]
   ```

3. **Logging**: In production, use `RUST_LOG=info` or `RUST_LOG=warn` to reduce log output
