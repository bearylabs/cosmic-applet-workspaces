# Cosmic Panel Integration Guide

## How the Applet Communicates with Cosmic Panel

### 1. Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│              Cosmic Desktop Environment                  │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  ┌─────────────────────────────────────────────────┐   │
│  │              Cosmic Panel (systemd)              │   │
│  │                                                  │   │
│  │  ┌──────────────────────────────────────────┐  │   │
│  │  │ Workspace Applet (IPC/D-Bus/Message)  │  │   │
│  │  │ - Displays workspace number            │  │   │
│  │  │ - Listens for workspace changes         │  │   │
│  │  │ - Handles click events                  │  │   │
│  │  └──────────────────────────────────────────┘  │   │
│  └─────────────────────────────────────────────────┘   │
│                     ▲                                   │
│                     │ D-Bus Messages                    │
│                     ▼                                   │
│  ┌─────────────────────────────────────────────────┐   │
│  │  Session D-Bus (org.freedesktop.DBus)           │   │
│  │                                                  │   │
│  │  org.Cosmic.Applet.Workspaces                   │   │
│  │  ├─ Properties:                                 │   │
│  │  │  └─ CurrentWorkspace (u32)                   │   │
│  │  │                                              │   │
│  │  ├─ Methods:                                    │   │
│  │  │  ├─ GetWorkspaces() → as (array string)      │   │
│  │  │  └─ SwitchWorkspace(u) → b (bool)            │   │
│  │  │                                              │   │
│  │  └─ Signals:                                    │   │
│  │     ├─ WorkspaceChanged(u)                      │   │
│  │     └─ WorkspacesUpdated(as)                    │   │
│  └─────────────────────────────────────────────────┘   │
│                                                          │
│              ▼ (Desktop/WM Events)                      │
│  ┌─────────────────────────────────────────────────┐   │
│  │     Workspace Manager (X11/Wayland)             │   │
│  │  - Manages 4+ workspaces                         │   │
│  │  - Emits workspace change events                 │   │
│  └─────────────────────────────────────────────────┘   │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

### 2. Communication Protocols

#### 2.1 D-Bus Interface
The applet exposes a D-Bus interface for communicating with Cosmic Panel:

**Service Name:** `org.Cosmic.Applet.Workspaces`  
**Object Path:** `/org/Cosmic/Applet/Workspaces`  
**Interface:** `org.Cosmic.Applet.Workspaces`

**Available Methods:**
- `GetWorkspaces() → as`: Returns list of workspace names
- `SwitchWorkspace(u workspace_id) → b`: Switch to workspace

**Available Properties:**
- `CurrentWorkspace: u` (read-only): Current active workspace ID

**Available Signals:**
- `WorkspaceChanged(u workspace_id)`: Emitted when workspace changes
- `WorkspacesUpdated(as workspaces)`: Emitted when workspace list updates

#### 2.2 Panel Integration Protocol

The Cosmic Panel communicates with applets via:

1. **Lifecycle Management:**
   - Panel starts applet as subprocess
   - Monitors applet process health
   - Restarts on failure (systemd Restart=on-failure)

2. **Event Handling:**
   - Panel forwards mouse clicks to applet
   - Applet responds with state updates
   - Panel renders applet's UI element

3. **Data Flow:**
   ```
   Panel → (Click Event) → Applet
   Applet → (D-Bus Message) → Window Manager
   Window Manager → (Workspace Change) → Applet
   Applet → (D-Bus Signal) → Panel
   Panel → (UI Update) → Display
   ```

### 3. Implementation Details

#### 3.1 libcosmic UI Integration

The applet uses libcosmic's built-in applet framework:

```rust
use cosmic::applet;
use cosmic::prelude::*;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    applet::run::<AppletState, Message, _>(
        "Cosmic Workspace Applet",
        Some(flags),
        AppletState::new(),
        init,           // Async initialization
        view,           // View builder
        update,         // Message handler
        on_window_resized, // Window sizing callback
    )?;
    Ok(())
}
```

Features:
- Automatic Panel integration
- Wayland/X11 support via iced runtime
- Message-driven architecture
- Built-in styling and theming

#### 3.2 D-Bus Communication

D-Bus allows external clients (Panel, other applets) to:
- Query workspace information
- Listen to workspace change signals
- Control workspace switching

Example D-Bus client communication:
```bash
# Get current workspace
busctl get-property org.Cosmic.Applet.Workspaces \
  /org/Cosmic/Applet/Workspaces \
  org.Cosmic.Applet.Workspaces \
  CurrentWorkspace

# Listen for workspace changes
busctl monitor org.Cosmic.Applet.Workspaces

# Switch workspace
busctl call org.Cosmic.Applet.Workspaces \
  /org/Cosmic/Applet/Workspaces \
  org.Cosmic.Applet.Workspaces \
  SwitchWorkspace u 1
```

#### 3.3 Systemd Service Integration

The applet's systemd service file ensures:
- Automatic startup in graphical session
- Proper lifecycle management
- D-Bus name registration
- Logging and monitoring

```ini
[Unit]
After=graphical-session-init.target
PartOf=graphical-session.target

[Service]
Type=simple
Restart=on-failure
RestartSec=5

[Install]
WantedBy=graphical-session.target
```

### 4. Message Flow Examples

#### 4.1 Initial Startup
```sequence
Panel              Applet             D-Bus            WM
  │                  │                 │               │
  ├─ Launch ─────────→│                 │               │
  │                  ├─ Register ──────→│               │
  │                  │← ACK ────────────┤               │
  │                  │                  ├─ Query ──────→│
  │                  │                  │← Workspaces ──┤
  │                  ├─ Emit Signal ────→│               │
  │←─ Display ────────┤                  │               │
```

#### 4.2 User Switches Workspace
```sequence
User               Panel              Applet           WM
  │                  │                 │               │
  ├─ Click ──────────→│                 │               │
  │                  ├─ Event ─────────→│               │
  │                  │                  ├─ Command ────→│
  │                  │                  │←─ ACK ────────┤
  │                  │←─ Signal ────────┤               │
  │                  ├─ Re-render ─────→│               │
  │←─ Display ────────┤                 │               │
```

#### 4.3 Workspace Changes (External)
```sequence
WM                 Applet           D-Bus            Panel
  │                  │               │                │
  ├─ Change ────────→│               │                │
  │                  ├─ Emit Signal ─→│                │
  │                  │                ├─ Broadcast ───→│
  │                  │                │                ├─ Update UI
  │                  │                │                ├─ Display
```

### 5. Error Handling

The applet implements robust error handling:

1. **D-Bus Connection Failures:**
   - Automatic reconnection with exponential backoff
   - Graceful degradation to fallback mode
   - User notification via UI

2. **Workspace Manager Errors:**
   - Fallback to static workspace list
   - Retry logic for transient failures
   - Logging for debugging

3. **Panel Integration Issues:**
   - Systemd service restart policy
   - Health check via D-Bus ping
   - Watchdog timeout protection

### 6. Installation and Deployment

#### Installation for Users
```bash
# Build
cargo build --release

# Install binary
sudo install -Dm755 target/release/cosmic-applet-workspaces \
  /usr/bin/cosmic-applet-workspaces

# Install desktop file
sudo install -Dm644 data/cosmic-applet-workspaces.desktop \
  /usr/share/applications/cosmic-applet-workspaces.desktop

# Install systemd service
sudo install -Dm644 data/systemd/cosmic-applet-workspaces.service \
  /usr/lib/systemd/user/cosmic-applet-workspaces.service

# Install D-Bus service file
sudo install -Dm644 data/systemd/org.Cosmic.Applet.Workspaces.service \
  /usr/share/dbus-1/services/org.Cosmic.Applet.Workspaces.service

# Install D-Bus interface definition
sudo install -Dm644 data/dbus-1/org.Cosmic.Applet.Workspaces.xml \
  /usr/share/dbus-1/interfaces/org.Cosmic.Applet.Workspaces.xml

# Enable service
systemctl --user enable cosmic-applet-workspaces.service
systemctl --user start cosmic-applet-workspaces.service
```

#### Verification
```bash
# Check D-Bus registration
busctl list | grep Cosmic.Applet

# Check systemd service status
systemctl --user status cosmic-applet-workspaces.service

# View logs
journalctl --user -u cosmic-applet-workspaces -f
```

### 7. Advanced Features (Future)

- Workspace labels/naming
- Workspace preview on hover
- Keyboard shortcuts for switching
- Configuration file support (TOML/YAML)
- Workspace pinning/protection
- Performance monitoring
