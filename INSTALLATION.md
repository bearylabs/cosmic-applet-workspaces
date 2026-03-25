# Cosmic Workspace Applet - Complete Installation & Troubleshooting Guide

## 🚀 Quick Start (5 Minutes)

### Prerequisites Check
```bash
# 1. Verify you're on Cosmic DE with Wayland
echo $XDG_SESSION_TYPE  # Should output: wayland
echo $XDG_CURRENT_DESKTOP  # Should contain: Cosmic

# 2. Cosmic Panel must be running
pgrep -l cosmic-panel  # Should show cosmic-panel process

# 3. Verify binary can be in PATH
mkdir -p ~/.local/bin
echo $PATH | grep -q ".local/bin" || echo "WARNING: ~/.local/bin not in PATH"
```

### Installation Steps (TL;DR)
```bash
# 1. Build the applet
cargo build --release

# 2. Install binary
cp target/release/cosmic-applet-workspaces ~/.local/bin/
chmod +x ~/.local/bin/cosmic-applet-workspaces

# 3. Ensure desktop file has CORRECT FLAGS
mkdir -p ~/.local/share/applications
cat > ~/.local/share/applications/com.system76.CosmicAppletWorkspaces.desktop << 'EOF'
[Desktop Entry]
Type=Application
Version=1.0
Name=Numbered Workspaces
Comment=Switch between numbered workspaces in the panel
Categories=COSMIC;
Exec=cosmic-applet-workspaces
Terminal=false
Icon=com.system76.CosmicAppletWorkspaces-symbolic
StartupNotify=true
NoDisplay=true

# CRITICAL: These must be present for Cosmic Panel detection!
X-CosmicApplet=true
X-HostWaylandDisplay=true
X-CosmicShrinkable=true
X-OverflowPriority=5
X-OverflowMinSize=8
EOF

# 4. Update desktop database
update-desktop-database ~/.local/share/applications/

# 5. Restart Cosmic Panel
killall cosmic-panel

# 6. Open Cosmic Panel Settings → Add Applets → "Numbered Workspaces"
```

---

## 🔧 Complete Installation Guide

### Step 1: Prerequisites

#### System Requirements:
- **OS**: Linux distribution with Cosmic DE
- **Desktop**: Cosmic 1.0+ (GNOME 46+)
- **Display Server**: Wayland (NOT X11)
- **Rust**: 1.70+ (via rustup)

#### Verify Installation:
```bash
# Check Wayland
echo "Session Type: $XDG_SESSION_TYPE"
# Expected: wayland

# Check Cosmic
echo "Desktop: $XDG_CURRENT_DESKTOP"
# Expected output contains: Cosmic

# Check Cosmic Panel is running
systemctl --user status cosmic-panel.service
# Should show: active (running)
```

#### If not Wayland:
```bash
# Switch to Wayland session
1. Log out completely
2. At login screen, click user name
3. Select "Cosmic" or "GNOME (Wayland)" session
4. Log in and verify: echo $XDG_SESSION_TYPE  # Should = wayland
```

### Step 2: Build the Applet

```bash
# Clone or navigate to repository
cd ~/dev/cosmic-applet-workspaces

# Clean previous build (if any)
cargo clean

# Build in release mode
cargo build --release

# Verify binary was created
ls -lho target/release/cosmic-applet-workspaces
# Should show executable ~5-15MB
```

**If build fails:**
```bash
# Update dependencies
cargo update

# Try build again
cargo build --release

# If still failing, check logs
cargo build --release 2>&1 | tail -50
```

### Step 3: Install Binary to PATH

#### Option A: User-Local Installation (Recommended)
```bash
# Create bin directory
mkdir -p ~/.local/bin

# Copy binary
cp target/release/cosmic-applet-workspaces ~/.local/bin/
chmod +x ~/.local/bin/cosmic-applet-workspaces

# Verify it's in PATH
which cosmic-applet-workspaces
# Should output: /home/USERNAME/.local/bin/cosmic-applet-workspaces

# Test binary directly
cosmic-applet-workspaces
# Should start without errors, output something to console
# Press Ctrl+C to stop
```

#### Option B: System-Wide Installation (Requires sudo)
```bash
sudo cp target/release/cosmic-applet-workspaces /usr/bin/
sudo chmod 0755 /usr/bin/cosmic-applet-workspaces

# Verify
which cosmic-applet-workspaces
# Should output: /usr/bin/cosmic-applet-workspaces
```

#### Option C: Ensure ~/.local/bin is in PATH
```bash
# Check if ~/.local/bin is in PATH
echo $PATH | grep -q ".local/bin" && echo "OK" || echo "NOT IN PATH"

# If NOT in PATH, add to ~/.bashrc or ~/.zshrc
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Verify
which cosmic-applet-workspaces
```

### Step 4: Create Desktop Entry File

The `.desktop` file is **CRITICAL** for Cosmic Panel detection. Without the correct flags, the panel will NOT see your applet.

```bash
# Create directory
mkdir -p ~/.local/share/applications

# Create desktop file with EXACT flags
cat > ~/.local/share/applications/com.system76.CosmicAppletWorkspaces.desktop << 'EOF'
[Desktop Entry]
Type=Application
Version=1.0
Name=Numbered Workspaces
Comment=Switch between numbered workspaces in the panel
Categories=COSMIC;
Exec=cosmic-applet-workspaces
Terminal=false
Icon=com.system76.CosmicAppletWorkspaces-symbolic
StartupNotify=true
NoDisplay=true
Keywords=workspace;panel;applet;

# ╔════════════════════════════════════════════════════════════╗
# ║ CRITICAL COSMIC PANEL DETECTION FIELDS                    ║
# ║ If these are missing or incorrect, Panel won't see applet  ║
# ╚════════════════════════════════════════════════════════════╝
X-CosmicApplet=true
X-HostWaylandDisplay=true
X-CosmicShrinkable=true
X-OverflowPriority=5
X-OverflowMinSize=8
EOF

# Verify file was created
cat ~/.local/share/applications/com.system76.CosmicAppletWorkspaces.desktop

# CRITICAL: Check X-CosmicApplet flag exists
grep "X-CosmicApplet" ~/.local/share/applications/com.system76.CosmicAppletWorkspaces.desktop
# Output should be: X-CosmicApplet=true
```

### Step 5: Update Desktop Database

Cosmic Panel scans the desktop database for applets:

```bash
# Update desktop file associations
update-desktop-database ~/.local/share/applications/

# Verify the desktop file is indexed
desktop-file-validate ~/.local/share/applications/com.system76.CosmicAppletWorkspaces.desktop
# Should output nothing (no errors)
```

### Step 6: Restart Cosmic Panel

The panel needs to be restarted to discover your new applet:

```bash
# Method 1: Kill and restart (fastest)
killall cosmic-panel
sleep 2  # Wait for panel to restart

# Method 2: Reboot (most reliable)
sudo reboot

# Method 3: Log out and back in
# - Press Super key
# - Select logout
# - Log back in
```

### Step 7: Add Applet to Panel

Now add the applet to your panel:

1. **Right-click on the Cosmic Panel** (edge or center)
2. Select **"Panel Settings"** or **"+"**
3. Look for **"Numbered Workspaces"** in the available applets list
4. **Click it to add** to the panel
5. The applet should appear in the panel within 1-2 seconds

**If applet doesn't appear in list:**
- Re-read Step 4 (Desktop File Creation) - flags may be wrong
- Run Step 5 (Update Desktop Database) again
- Go to Step 6 (Restart Panel)

---

## 🔍 Troubleshooting

### Problem 1: Applet doesn't appear in "Add Applets" list

**Root Cause:** Cosmic Panel can't find your desktop file

**Checklist:**
```bash
# 1. Verify desktop file exists
ls -la ~/.local/share/applications/com.system76.CosmicAppletWorkspaces.desktop

# 2. Verify X-CosmicApplet flag (CRITICAL!)
grep "^X-CosmicApplet=" ~/.local/share/applications/com.system76.CosmicAppletWorkspaces.desktop
# Must output: X-CosmicApplet=true

# 3. Check desktop file syntax
desktop-file-validate ~/.local/share/applications/com.system76.CosmicAppletWorkspaces.desktop
# Should output nothing (no errors)

# 4. Verify binary is in PATH
which cosmic-applet-workspaces
# Should output a path (not "not found")

# 5. Update desktop database
update-desktop-database ~/.local/share/applications/
```

**Solution:**
- If binary not found: Run Step 3 (Install Binary)
- If desktop file syntax error: Run Step 4 again (Copy exact desktop file)
- If database not updated: Run `update-desktop-database ~/.local/share/applications/`
- Restart panel: `killall cosmic-panel; sleep 2`

### Problem 2: Applet crashes immediately after launch

**Root Cause:** Missing cosmic dependencies or workspace manager issue

**Checklist:**
```bash
# 1. Test binary directly
cosmic-applet-workspaces
# Should output debug information, not crash

# 2. Check error output
cosmic-applet-workspaces 2>&1 | head -20

# 3. Verify libcosmic compiled correctly
cargo build --release 2>&1 | tail -30

# 4. Check workspace manager can detect workspaces
echo "Workspace detection: $(cosmic-applet-workspaces 2>&1 | grep -i workspace)"
```

**Solutions:**
- Rebuild: `cargo clean && cargo build --release`
- Check systemd journal: `journalctl -u cosmic-panel -f`
- Ensure running on Wayland: `echo $XDG_SESSION_TYPE` (should = wayland)

### Problem 3: "Binary not found" error in panel

**Root Cause:** Binary is not installed to PATH

**Checklist:**
```bash
# 1. Verify binary exists
file ~/.local/bin/cosmic-applet-workspaces

# 2. Verify it's in PATH
echo $PATH | grep -q ".local/bin" && echo "In PATH" || echo "NOT in PATH"

# 3. Verify it's executable
ls -la ~/.local/bin/cosmic-applet-workspaces | grep '^-rwx'

# 4. Test binary execution
~/.local/bin/cosmic-applet-workspaces --version 2>&1 || echo "Binary error"
```

**Solutions:**
- Copy to PATH: `cp target/release/cosmic-applet-workspaces ~/.local/bin/`
- Make executable: `chmod +x ~/.local/bin/cosmic-applet-workspaces`
- Add ~/.local/bin to PATH (see Step 3-Option C)
- Use full path in desktop file: `Exec=/home/USERNAME/.local/bin/cosmic-applet-workspaces`

### Problem 4: Applet shows but doesn't display workspace info

**Root Cause:** Workspace manager not detecting workspaces (D-Bus issue)

**Diagnostics:**
```bash
# 1. Check if workspace manager service is running
systemctl --user status cosmic-workspaces
# Or check what workspace service exists
dbus-send --session --print-reply --dest=org.freedesktop.DBus /org/freedesktop/DBus \
  org.freedesktop.DBus.ListNames

# 2. Run the applet and check output
cosmic-applet-workspaces 2>&1

# 3. Check workspace manager module
grep -r "WorkspaceManager" src/

# 4. Check D-Bus interface XML
cat data/dbus-1/org.Cosmic.Applet.Workspaces.xml
```

**Solutions:**
- Ensure workspace manager is running properly
- Check if using X11 instead of Wayland: `echo $XDG_SESSION_TYPE` should = wayland
- File a bug if D-Bus service not available

### Problem 5: Panel restarts when applet is added

**Root Cause:** Applet crashes, causing panel to restart

**Diagnostics:**
```bash
# 1. Check system journal for crashes
journalctl -u cosmic-panel --no-pager -n 50

# 2. Test applet standalone
WAYLAND_DISPLAY=$WAYLAND_DISPLAY cosmic-applet-workspaces

# 3. Check for panic messages
RUST_BACKTRACE=1 cosmic-applet-workspaces 2>&1
```

**Solutions:**
- Rebuild without old artifacts: `cargo clean && cargo build --release`
- Check if all dependencies compiled: `cargo tree`
- Review error messages in applet output

---

## ✅ Verification Checklist

After installation, verify everything is working:

```bash
# 1. Binary is in PATH and executable
which cosmic-applet-workspaces && echo "✓ Binary found" || echo "✗ Binary NOT in PATH"
ls -la ~/.local/bin/cosmic-applet-workspaces | grep -q rwx && echo "✓ Executable" || echo "✗ Not executable"

# 2. Desktop file has correct flags
grep "^X-CosmicApplet=true" ~/.local/share/applications/com.system76.CosmicAppletWorkspaces.desktop \
  && echo "✓ X-CosmicApplet flag" || echo "✗ Missing X-CosmicApplet flag"

# 3. Desktop file syntax is valid
desktop-file-validate ~/.local/share/applications/com.system76.CosmicAppletWorkspaces.desktop \
  && echo "✓ Desktop file valid" || echo "✗ Desktop file has errors"

# 4. Can run binary directly
cosmic-applet-workspaces >/dev/null 2>&1 && echo "✓ Binary runs" || echo "✗ Binary crashes"

# 5. Panel can find applet
gsettings get com.system76.CosmicPanel applets | grep -i workspace && echo "✓ Panel configured" || echo "? Check panel settings"
```

---

## 📞 Getting Help

If you're still having issues:

1. **Check the README.md** - Installation section
2. **Review COSMIC_INTEGRATION.md** - Architecture details
3. **Check workspace_manager.rs** - How workspaces are detected
4. **Inspect dbus-1/ and systemd/** folders - Service configuration
5. **Open issue on GitHub** with:
   - Output of `echo $XDG_SESSION_TYPE`
   - Output of `which cosmic-applet-workspaces`
   - Output of `cosmic-applet-workspaces 2>&1 | head -20`
   - Output of `ls -la ~/.local/share/applications/com.system76.CosmicAppletWorkspaces.desktop`

---

## 🎯 Summary of Critical Points

| Item | Critical? | Why |
|------|-----------|-----|
| `X-CosmicApplet=true` in desktop file | **YES** | Panel uses this to identify applets |
| Binary in PATH | **YES** | Panel can't find/execute binary otherwise |
| Desktop file location | **YES** | Must be `~/.local/share/applications/` |
| Wayland session | **YES** | Cosmic Panel + Wayland layer shell required |
| `NoDisplay=true` | **YES** | Prevents app from appearing in normal menus |
| `Exec=cosmic-applet-workspaces` | **YES** | Must be exact binary name or full path |
| `X-HostWaylandDisplay=true` | **NO** | Optional, but recommended for best compatibility |

---

Generated: March 2024  
For cosmic-applet-workspaces v1.0.0
