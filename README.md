# Cosmic Applet - Workspaces

A modular Rust application that monitors and displays workspace information, designed to integrate with Cosmic DE (System76's desktop environment).

## Features

- **Workspace Monitoring**: Detects and displays the current active workspace
- **D-Bus Integration**: Optional D-Bus interface for system integration
- **Lightweight**: Written in pure Rust with minimal dependencies
- **Modular Design**: Easy to extend with additional features

## Prerequisites

- Rust 1.94+ (installed via rustup)
- Cargo (comes with Rust)
- For D-Bus integration on Linux: D-Bus development libraries

## Building

### Basic Build

```bash
cargo build --release
```

### With D-Bus Support (Linux)

```bash
cargo build --release --features dbus-interface
```

The compiled binary will be available at `target/release/cosmic-applet-workspaces`.

## Running

```bash
./target/release/cosmic-applet-workspaces
```

Output:

```
Cosmic Workspace Applet - Starting up...
Current workspace: 0

Available workspaces:
  Workspace 0: 1 [ACTIVE]
  Workspace 1: 2
  Workspace 2: 3
  Workspace 3: 4

Applet is ready to be integrated with Cosmic DE.
```

## Installation

### On NixOS with Cosmic DE

#### Step 1: Install Binary

```bash
# Create applets directory
mkdir -p ~/.local/share/cosmic/applets/

# Copy binary after building
cp target/release/cosmic-applet-workspaces ~/.local/share/cosmic/applets/
chmod +x ~/.local/share/cosmic/applets/cosmic-applet-workspaces
```

#### Step 2: Create Desktop Entry

Create `~/.local/share/applications/com.system.Workspaces.desktop`:

```ini
[Desktop Entry]
Name=Workspaces
Comment=Display current workspace number
Type=Application
Categories=Utility;

# Cosmic Applet specific
X-Cosmic-Applet=true
Exec=%h/.local/share/cosmic/applets/cosmic-applet-workspaces
Icon=view-grid

StartupNotify=false
NoDisplay=false
```

#### Step 3: Reload Cosmic Panel

```bash
# Option 1: Kill and restart the panel
killall cosmic-panel

# Option 2: Log out and log back in
```

#### Step 4: Add Applet to Panel

1. Open **Cosmic Panel Settings**
2. Navigate to "Applets" or "Add Applets"
3. Select "Workspaces" from the list
4. Click to add the applet to your panel

## Project Structure

```
cosmic-applet/
├── src/
│   ├── main.rs                 # Entry point and main UI logic
│   ├── workspace_manager.rs    # Workspace detection and management
│   └── dbus_interface.rs       # D-Bus interface (feature-gated)
├── Cargo.toml                  # Project manifest and dependencies
├── README.md                   # This file
└── .github/
    └── copilot-instructions.md # Development guidelines
```

## Development

### Adding New Workspaces

Edit `src/workspace_manager.rs` to add dynamic workspace detection via D-Bus or other methods.

### Enabling D-Bus Interface

1. Install zbus and tokio dependencies:

   ```bash
   cargo add tokio --features full
   cargo add zbus
   ```

2. Implement D-Bus proxy interface in `src/dbus_interface.rs`

3. Build with feature:
   ```bash
   cargo build --release --features dbus-interface
   ```

### Integration with Cosmic DE

To use this as a panel applet in Cosmic DE:

1. Build the release binary
2. Place the binary in `/usr/local/bin/` or another location in your PATH
3. Configure Cosmic DE to load and display the applet in the panel

## Dependencies

- **serde**: Serialization framework (optional, with `dbus-interface` feature)
- **tokio**: Async runtime (optional, with `dbus-interface` feature)
- **zbus**: D-Bus client library (optional, with `dbus-interface` feature)

## License

Licensed under the GPLv3 - see LICENSE file for details

## Contributing

Contributions are welcome! Please ensure:

- Code compiles without warnings
- Features are well-documented
- Changes are tested on Linux with Cosmic DE
