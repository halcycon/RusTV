# RusTV GUI Overview

## Architecture

The RusTV GUI application provides a professional matrix viewer interface for managing NDI video routing.

## GUI Structure

```
┌─────────────────────────────────────────────────────────────────┐
│  Menu Bar: [View] - Layout Panel ☑  Routing Panel ☑            │
│            Current Layout: 2x2 Grid                             │
├──────────────┬──────────────────────────────┬───────────────────┤
│              │                              │                   │
│  Layout      │     Matrix View Area         │   Routing Control │
│  Selection   │                              │                   │
│              │  ┌────────┬────────┐         │  🔄 Refresh       │
│  ◉ 2x2 Grid  │  │Monitor1│Monitor2│         │                   │
│  ○ 3x3 Grid  │  │ ← CAM1 │(No inp)│         │  Available Sources│
│  ○ 4x4 Grid  │  └────────┴────────┘         │  ├─ Camera 1     │
│  ○ PiP       │  ┌────────┬────────┐         │  ├─ Camera 2     │
│  ○ 1+7       │  │Monitor3│Monitor4│         │  └─ Capture Card │
│              │  │(No inp)│ ← CAM2 │         │                   │
│              │  └────────┴────────┘         │  ➡ Route Selected │
│              │                              │                   │
│              │  (Click a view slot to       │  Current Routes   │
│              │   select for routing)        │  ├─ M1 ← CAM1 ❌  │
│              │                              │  └─ M4 ← CAM2 ❌  │
│              │                              │                   │
└──────────────┴──────────────────────────────┴───────────────────┘
```

## Layout Types

### 2x2 Grid (4 Views)
```
┌─────┬─────┐
│  1  │  2  │
├─────┼─────┤
│  3  │  4  │
└─────┴─────┘
```

### 3x3 Grid (9 Views)
```
┌───┬───┬───┐
│ 1 │ 2 │ 3 │
├───┼───┼───┤
│ 4 │ 5 │ 6 │
├───┼───┼───┤
│ 7 │ 8 │ 9 │
└───┴───┴───┘
```

### 4x4 Grid (16 Views)
```
┌──┬──┬──┬──┐
│1 │2 │3 │4 │
├──┼──┼──┼──┤
│5 │6 │7 │8 │
├──┼──┼──┼──┤
│9 │10│11│12│
├──┼──┼──┼──┤
│13│14│15│16│
└──┴──┴──┴──┘
```

### Picture in Picture (PiP)
```
┌──────────────────┐
│                  │
│   Main View      │
│                  │
│           ┌────┐ │
│           │PiP │ │
│           └────┘ │
└──────────────────┘
```

### 1+7 Layout
```
┌──────────────┬─┐
│              │1│
│              ├─┤
│     Main     │2│
│     View     ├─┤
│              │3│
│              ├─┤
│              │4│
├──────────────┼─┤
│              │5│
│              ├─┤
│              │6│
│              ├─┤
│              │7│
└──────────────┴─┘
```

## Features

### Matrix View Display
- Visual representation of all output destinations
- Shows assigned input for each output
- Click to select view slots for routing
- Color-coded selection state
- Real-time updates

### Layout Selection
- Easy switching between 5 different layouts
- Instant visual feedback
- Optimized for different monitoring scenarios

### Routing Control
- **Source Discovery**: Click "🔄 Refresh Sources" to discover NDI sources
- **Source Selection**: Click on a source in the list to select it
- **View Selection**: Click on a view slot in the matrix to select it
- **Create Route**: Click "➡ Route Selected" to create the routing
- **Route Management**: View all active routes and remove them with ❌ button

### Panels
- **Layout Panel** (Left): Choose from available layouts
- **Routing Panel** (Right): Manage sources and routes
- **Menu Bar** (Top): Toggle panels and view current layout

## Workflow Example

### Basic Routing Workflow

1. **Start Application**
   ```bash
   rustv gui
   ```

2. **Discover Sources**
   - Click "🔄 Refresh Sources"
   - Wait for NDI sources to appear in the list

3. **Select Source**
   - Click on desired source (e.g., "Camera 1")

4. **Select Destination**
   - Click on a view slot in the matrix (e.g., "Monitor 1")
   - The slot will highlight

5. **Create Route**
   - Click "➡ Route Selected"
   - The route is created and displayed

6. **Change Layout** (Optional)
   - Click on different layout in Layout Panel
   - Matrix view reorganizes automatically

## Technical Details

### Components

**GUI Module Structure**:
```
src/gui/
├── mod.rs          # Module exports
├── app.rs          # Main application logic
└── layouts.rs      # Layout definitions
```

**Key Types**:
- `MatrixViewerApp`: Main application state and UI logic
- `Layout`: Enum defining available layouts
- `ViewSlot`: State for each output view

**Dependencies**:
- `egui`: Immediate mode GUI framework
- `eframe`: Application framework
- Integrates with existing NDI, matrix, and config modules

### State Management

- **MatrixRouter**: Shared via `Arc<Mutex<>>` for thread-safe routing
- **NdiDiscovery**: Shared via `Arc` for source discovery
- **View Slots**: Local state tracking output assignments

### Threading

- Main GUI runs on the main thread
- NDI discovery runs in a background tokio task
- UI updates at 10 FPS for smooth interaction

## Usage Tips

1. **Quick Start**: Just run `rustv` without arguments to launch the GUI

2. **Layout Selection**: Choose layout based on your needs:
   - 2x2: Small setups with 4 cameras
   - 3x3/4x4: Larger installations
   - PiP: Focus on one source with reference
   - 1+7: Main program output with multiple previews

3. **Efficient Routing**: 
   - Keep Routing Panel open for quick changes
   - Use keyboard navigation in source list
   - Remove old routes before creating new ones on same output

4. **Monitoring**: The GUI updates automatically as sources come and go

## Future Enhancements

Possible future additions:
- Live video preview in view slots
- Custom layout editor
- Route presets and saved configurations
- Multi-viewer mode for multiple matrix routers
- Touch screen optimization
- Keyboard shortcuts for common actions
