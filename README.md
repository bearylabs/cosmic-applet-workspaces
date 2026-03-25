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

#### ⚠️ Prerequisites for Cosmic Panel Integration

1. **Binary location**: Must be in `PATH` (e.g., `/usr/bin/` or `~/.local/bin/`)
2. **Desktop Entry**: Required in `~/.local/share/applications/`
3. **Cosmic Panel**: Must be running on Wayland
4. **libcosmic**: Applet must use `cosmic` crate for UI rendering

#### Step 1: Build the Applet with Cosmic Dependencies

```bash
# Standard release build (includes Cosmic UI framework)
cargo build --release

# The binary will be at: target/release/cosmic-applet-workspaces
```

#### Step 2: Install Binary to PATH

```bash
# For user-local installation:
mkdir -p ~/.local/bin
cp target/release/cosmic-applet-workspaces ~/.local/bin/
chmod +x ~/.local/bin/cosmic-applet-workspaces

# For system-wide installation (requires sudo):
sudo cp target/release/cosmic-applet-workspaces /usr/bin/
sudo chmod 0755 /usr/bin/cosmic-applet-workspaces
```

#### Step 3: Create Desktop Entry File (CRITICAL)

Create `~/.local/share/applications/com.system76.CosmicAppletWorkspaces.desktop`:

**⚠️ IMPORTANT: These exact fields are REQUIRED for Cosmic Panel detection:**

```ini
[Desktop Entry]
Name=Numbered Workspaces
Comment=Switch between numbered workspaces in the panel
Type=Application
Exec=cosmic-applet-workspaces
Terminal=false
Categories=COSMIC;
Icon=com.system76.CosmicAppletWorkspaces-symbolic
StartupNotify=true
NoDisplay=true
X-GNOME-UsesNotifications=false

# Cosmic Panel Applet Specific Fields (MANDATORY)
X-CosmicApplet=true
X-HostWaylandDisplay=true
X-CosmicShrinkable=true
X-OverflowPriority=5
X-OverflowMinSize=8
```

**Field Explanations:**
- `X-CosmicApplet=true` - **MANDATORY** - Identifies this as a Cosmic Panel Applet
- `NoDisplay=true` - Hides from application menu (appears only in Panel Settings)
- `X-HostWaylandDisplay=true` - Passes Wayland display to applet
- `X-OverflowPriority=5` - Panel priority when space is limited (0-10)
- `Exec=cosmic-applet-workspaces` - Must be in PATH or full path

#### Step 4: Enable Applet Discovery

```bash
# Update desktop database (if not automatic)
update-desktop-database ~/.local/share/applications/

# Verify desktop file is readable
ls -la ~/.local/share/applications/com.system76.CosmicAppletWorkspaces.desktop
```

#### Step 5: Restart Cosmic Panel

```bash
# Method 1: Kill and restart (fastest)
killall cosmic-panel
sleep 1

# Method 2: Log out and back in (most reliable)
# Or reboot the system
```

#### Step 6: Add Applet to Panel

1. Right-click on **Cosmic Panel** → **Panel Settings**
2. Click **"Add Applets"** or **"+"** button
3. Look for **"Numbered Workspaces"** in the applet list
4. Click to add to panel
5. Applet appears in panel (may take 1-2 seconds to render)

#### ✅ Verification Checklist

```bash
# 1. Check if binary is in PATH
which cosmic-applet-workspaces

# 2. Test binary directly
cosmic-applet-workspaces  # Should display without errors

# 3. Check desktop file is valid
file ~/.local/share/applications/com.system76.CosmicAppletWorkspaces.desktop

# 4. Check for Cosmic Panel detection
grep X-CosmicApplet ~/.local/share/applications/com.system76.CosmicAppletWorkspaces.desktop
# Output should show: X-CosmicApplet=true

# 5. Monitor panel logs for errors
journalctl -u cosmic-panel -f  # May not exist on all systems

# 6. Check if panel detects applet
ls ~/.config/cosmic/com.system76.CosmicPanel/  # Panel configuration
```

#### 🔴 Common Issues & Solutions

| Issue | Cause | Solution |
|-------|-------|----------|
| Applet doesn't appear in Settings | `X-CosmicApplet` missing or wrong | Use exact `X-CosmicApplet=true` |
| "Applet not found" error | Binary not in PATH | Run with full path in desktop file or add to PATH |
| Applet crashes on launch | Missing libcosmic dependency | Rebuild with: `cargo build --release` |
| Panel doesn't detect applet | Desktop file in wrong location | Must be in `~/.local/share/applications/` |
| Desktop file not scanned | Desktop database not updated | Run: `update-desktop-database ~/.local/share/applications/` |
| Workspace info not displaying | D-Bus workspace detection failing | Check if workspace manager D-Bus service is running |

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
