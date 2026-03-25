# Cosmic Applet Workspaces - Development & Testing Guide

## 🧪 Testing Without Installing to Panel

### Run Applet Standalone
```bash
# Build and run directly
cargo run --release

# With debug output
RUST_LOG=debug cargo run --release

# As it would run in panel (standalone window)
WAYLAND_DISPLAY=$WAYLAND_DISPLAY cargo run --release
```

### Monitor Panel Logs (Real-time)
```bash
# Watch panel service logs
journalctl -u cosmic-panel.service -f --no-pager

# Watch all cosmic-related services
journalctl -u cosmic-*.service -f --no-pager

# Search for applet crashes
journalctl -u cosmic-panel | grep -i "cosmic-applet-workspaces"
```

---

## 🐛 Debugging Common Issues

### Issue: Applet Not Appearing in Panel Settings

**Step 1: Verify Desktop File is Readable**
```bash
# Check file exists and has correct permissions
ls -la ~/.local/share/applications/com.system76.CosmicAppletWorkspaces.desktop

# Expected output:
# -rw-r--r-- 1 user user XXXX DATE com.system76.CosmicAppletWorkspaces.desktop
```

**Step 2: Validate Desktop File Syntax**
```bash
# Check for syntax errors
desktop-file-validate ~/.local/share/applications/com.system76.CosmicAppletWorkspaces.desktop

# If errors: Review the file
cat ~/.local/share/applications/com.system76.CosmicAppletWorkspaces.desktop

# Compare with reference (should have these exact lines):
grep "^X-Cosmic" ~/.local/share/applications/com.system76.CosmicAppletWorkspaces.desktop
# Should output:
# X-CosmicApplet=true
# X-HostWaylandDisplay=true
# X-CosmicShrinkable=true
# X-OverflowPriority=5
# X-OverflowMinSize=8
```

**Step 3: Rebuild Desktop Database**
```bash
# Clear and rebuild
rm ~/.local/share/applications/mimeapps.list
update-desktop-database ~/.local/share/applications/

# Verify it was indexed
grep -r "com.system76.CosmicAppletWorkspaces" ~/.local/share/applications/
```

**Step 4: Ensure Binary is in PATH**
```bash
# Test binary directly
which cosmic-applet-workspaces
# Should output: /home/USERNAME/.local/bin/cosmic-applet-workspaces

# Test execution
cosmic-applet-workspaces --help || cosmic-applet-workspaces
# Should run without "command not found"
```

**Step 5: Force Panel Refresh**
```bash
# Kill panel process
killall cosmic-panel

# Wait for respawn
sleep 3

# Verify panel restarted
pgrep -l cosmic-panel

# Re-check applets list in panel settings
# Right-click Panel → Panel Settings → Add Applets
```

---

### Issue: Applet Crashes When Added to Panel

**Get Full Error Output:**
```bash
# Run applet with full backtrace
RUST_BACKTRACE=full ~/.local/bin/cosmic-applet-workspaces 2>&1 | head -100

# Or with debug logging
RUST_LOG=cosmic_applet_workspaces=debug ~/.local/bin/cosmic-applet-workspaces 2>&1

# Check if it's a D-Bus issue
RUST_LOG=zbus=debug ~/.local/bin/cosmic-applet-workspaces 2>&1 | head -50
```

**Check Panel Logs for Your Applet:**
```bash
# Tail panel logs
journalctl -u cosmic-panel -n 100 --no-pager

# Look for crash/panic messages
journalctl -u cosmic-panel | grep -i "panic\|thread\|failed" | tail -20

# Search specifically for workspace applet
journalctl | grep -i "cosmic.*workspace" --color=always | tail -20
```

**Check if it's a Wayland Display Issue:**
```bash
# Verify Wayland is available to applet
echo "XDG_SESSION_TYPE: $XDG_SESSION_TYPE"
echo "WAYLAND_DISPLAY: $WAYLAND_DISPLAY"

# Test applet with explicit Wayland display
WAYLAND_DISPLAY=$WAYLAND_DISPLAY ~/.local/bin/cosmic-applet-workspaces 2>&1
```

---

### Issue: Workspace Information Not Displaying

**Test Workspace Detection:**
```bash
# Run the applet and check output
cosmic-applet-workspaces

# Look for lines containing:
# - "Current workspace: N"
# - "Available workspaces:"
# - Workspace list

# If no workspace info, check workspace manager:
grep -A 20 "fn get_workspaces" src/workspace_manager.rs
```

**Verify Workspace Manager is Running:**
```bash
# Check for workspace-related D-Bus services
dbus-send --session --print-reply --dest=org.freedesktop.DBus \
  /org/freedesktop/DBus org.freedesktop.DBus.ListNames | grep -i workspace

# Check systemd user services
systemctl --user list-units | grep -i cosmic\|workspace
```

**Check D-Bus Interface:**
```bash
# List available D-Bus services
dbus-send --session --print-reply \
  --dest=org.freedesktop.DBus \
  /org/freedesktop/DBus \
  org.freedesktop.DBus.ListNames | grep -i workspaces

# Try to call D-Bus interface directly (if implemented)
dbus-send --session --print-reply \
  --dest=org.Cosmic.Applet.Workspaces \
  /org/Cosmic/Applet/Workspaces \
  org.Cosmic.Applet.Workspaces.GetWorkspaces
```

---

## 📊 Checking Applet Status

### Quick Status Report
```bash
#!/bin/bash
echo "=== Cosmic Applet Workspaces Status Report ==="
echo ""
echo "1. Session Type:"
echo "   $XDG_SESSION_TYPE"
echo ""
echo "2. Binary Status:"
which cosmic-applet-workspaces && echo "   ✓ Found in PATH" || echo "   ✗ NOT in PATH"
test -x ~/.local/bin/cosmic-applet-workspaces && echo "   ✓ Executable" || echo "   ✗ Not executable"
echo ""
echo "3. Desktop File Status:"
test -f ~/.local/share/applications/com.system76.CosmicAppletWorkspaces.desktop && \
  echo "   ✓ Desktop file exists" || \
  echo "   ✗ Desktop file NOT found"
grep "^X-CosmicApplet=true" ~/.local/share/applications/com.system76.CosmicAppletWorkspaces.desktop >/dev/null && \
  echo "   ✓ X-CosmicApplet flag present" || \
  echo "   ✗ X-CosmicApplet flag MISSING"
echo ""
echo "4. Panel Status:"
pgrep -l cosmic-panel > /dev/null && \
  echo "   ✓ Panel running" || \
  echo "   ✗ Panel NOT running"
echo ""
echo "5. Recent Errors:"
journalctl -u cosmic-panel -n 5 --no-pager 2>/dev/null | grep -i "error\|warn" || echo "   ✓ No recent errors"
```

Save as `status-check.sh`, make executable, and run:
```bash
bash status-check.sh
```

---

## 🔧 Development Mode

### Watch Mode (Auto-rebuild on changes)
```bash
cargo watch -x "build --release"
```

### Run Tests
```bash
cargo test --release
cargo test -- --nocapture  # Show println! output
```

### Check Code Quality
```bash
cargo clippy --release

# With more lint checks
cargo clippy --release -- -W clippy::all
```

### Format Code
```bash
cargo fmt --check  # Check if formatted
cargo fmt          # Auto-format
```

---

## 📝 Making Changes

### Edit Workspace Manager
File: `src/workspace_manager.rs`
- Modify `get_workspaces()` to change detection logic
- Add `switch_workspace()` for workspace switching

### Edit UI
File: `src/main.rs`
- `AppletState::view()` - Contains the UI rendering
- `AppletState::update()` - Handles messages/events

### Edit D-Bus Interface (if enabled)
File: `src/dbus_interface.rs`
- Add methods/properties for D-Bus communication
- Remember to build with: `cargo build --release --features dbus-interface`

### After Changes:
```bash
# Rebuild
cargo build --release

# Copy new binary
cp target/release/cosmic-applet-workspaces ~/.local/bin/

# Restart panel
killall cosmic-panel
sleep 2

# Re-add applet if necessary
```

---

## 🚀 Installing to System

### For Testing (User-Local)
```bash
# Copy to user bin
cp target/release/cosmic-applet-workspaces ~/.local/bin/
chmod +x ~/.local/bin/cosmic-applet-workspaces

# Verify
which cosmic-applet-workspaces
```

### For Production (System-Wide)
```bash
# Copy to system bin
sudo cp target/release/cosmic-applet-workspaces /usr/bin/
sudo chmod 0755 /usr/bin/cosmic-applet-workspaces

# Copy desktop file
sudo cp data/cosmic-applet-workspaces.desktop /usr/share/applications/

# Update desktop database
sudo update-desktop-database /usr/share/applications/

# Copy systemd service (optional)
sudo cp data/systemd/cosmic-applet-workspaces.service \
  /usr/lib/systemd/user/

# Restart panel
killall cosmic-panel
```

---

## 📦 Packaging (for Distribution)

### Create Installation Script
```bash
#!/bin/bash
# install.sh

set -e

echo "Building Cosmic Applet Workspaces..."
cargo build --release

echo "Installing binary..."
install -Dm0755 target/release/cosmic-applet-workspaces \
  "${DESTDIR}/usr/bin/cosmic-applet-workspaces"

echo "Installing desktop file..."
install -Dm0644 data/cosmic-applet-workspaces.desktop \
  "${DESTDIR}/usr/share/applications/com.system76.CosmicAppletWorkspaces.desktop"

echo "Installing systemd service..."
install -Dm0644 data/systemd/cosmic-applet-workspaces.service \
  "${DESTDIR}/usr/lib/systemd/user/cosmic-applet-workspaces.service"

echo "Installation complete!"
```

Make executable: `chmod +x install.sh`  
Run: `./install.sh` or `DESTDIR=/tmp/package ./install.sh`

---

## 📖 Helpful Resources

- **Cosmic Panel Source**: https://github.com/pop-os/cosmic-panel
- **Cosmic Applets Examples**: https://github.com/pop-os/cosmic-applets
- **libcosmic Docs**: https://github.com/pop-os/libcosmic
- **D-Bus Documentation**: https://dbus.freedesktop.org/doc/
- **Wayland Documentation**: https://wayland.freedesktop.org/

---

## 🆘 When All Else Fails

```bash
# Nuclear option: Complete rebuild
cargo clean
rm -rf ~/.cargo/registry/src/*/cosmic*
rm -rf ~/.cargo/registry/src/*/iced*
cargo build --release

# Reset desktop database
rm -rf ~/.local/share/applications/mimeapps.list
update-desktop-database ~/.local/share/applications/

# Restart full desktop
sudo systemctl restart cosmic-panel
# Or: Log out and back in
# Or: Reboot
```

---

Last updated: March 2024  
For cosmic-applet-workspaces v1.0.0
