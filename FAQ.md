# ❓ FAQ - Häufig gestellte Fragen

## Installation & Setup

### F: Wie installiere ich das Applet schnell?
**A:** Siehe [QUICK_FIX.md](QUICK_FIX.md) - 5 Minuten komplett, einfache Copy-Paste Befehle.

### F: Die vollständige Anleitung?
**A:** Siehe [INSTALLATION.md](INSTALLATION.md) - Detailliert, Schritt-für-Schritt, mit Troubleshooting.

### F: Muss ich das auf den gesamten Computer anwenden oder nur auf meinen Benutzer?
**A:** Beides möglich:
- **User-lokal (empfohlen)**: `~/.local/bin/` und `~/.local/share/applications/`
- **System-weit**: `/usr/bin/` und `/usr/share/applications/` (braucht `sudo`)

### F: Ich bin auf X11 statt Wayland. Funktioniert es?
**A:** Nein. Cosmic Panel und libcosmic benötigen Wayland. 
- Logout → Bei Login "Cosmic" oder "GNOME (Wayland)" auswählen → Neu anmelden
- Verify: `echo $XDG_SESSION_TYPE` sollte `wayland` sein

### F: Ich weiß nicht, wo mein binary nach dem Build ist.
**A:** Nach `cargo build --release` ist das binary hier: `target/release/cosmic-applet-workspaces`
```bash
ls -lh target/release/cosmic-applet-workspaces  # Sollte ~5-15MB sein
```

---

## Häufige Fehler

### F: "Applet ist nicht in der 'Add Applets' Liste sichtbar"
**A:** 99% der Zeit: Desktop Entry Datei ist falsch.

Überprüfe:
```bash
# 1. Muss diese EXAKTEN Zeilen haben:
grep "^X-CosmicApplet=true" ~/.local/share/applications/com.system76.CosmicAppletWorkspaces.desktop
# Sollte anzeigen: X-CosmicApplet=true

# 2. Wenn nicht, nimm die Datei aus QUICK_FIX.md
# 3. Update database:
update-desktop-database ~/.local/share/applications/

# 4. Panel neustarten:
killall cosmic-panel
```

### F: "cosmic-applet-workspaces: command not found"
**A:** Binary ist nicht in PATH.
```bash
# 1. Wo ist die binary?
find ~ -name "cosmic-applet-workspaces" -type f 2>/dev/null

# 2. Kopiere zu PATH:
cp target/release/cosmic-applet-workspaces ~/.local/bin/
chmod +x ~/.local/bin/cosmic-applet-workspaces

# 3. Überprüfe:
which cosmic-applet-workspaces  # Sollte einen Pfad anzeigen
```

### F: "Applet erscheint einmal und verschwindet dann"
**A:** Applet crasht. 
```bash
# 1. Teste applet direkt:
cosmic-applet-workspaces 2>&1  # Sollte ohne Fehler laufen

# 2. Wenn es crasht, schau die Fehlermeldung an
# 3. Neubauen:
cargo clean && cargo build --release

# 4. Check desktop file nochmal
```

### F: "Desktop file syntax error"
**A:** Datei hat Formatierungsfehler.
```bash
# Validiere:
desktop-file-validate ~/.local/share/applications/com.system76.CosmicAppletWorkspaces.desktop

# Wenn Fehler, nutze die exakte Datei aus QUICK_FIX.md
```

---

## D-Bus & Workspace Erkennungs Fragen

### F: Wie erkenne ich verfügbare Workspaces?
**A:** Die `workspace_manager.rs` Module handhaben das. Die Default-Implementation:
```rust
// Schaut in src/workspace_manager.rs
pub fn get_workspaces() -> Result<(Vec<WorkspaceInfo>, u32)>
```

Workspaces können von verschiedenen Quellen kommen:
- Current Desktop Environment workspace manager
- D-Bus Service (falls vorhanden)
- Environment Variables

### F: Warum wird die Workspace-Info nicht angezeigt?
**A:** Workspace Manager kann keine Workspaces erkennen.
```bash
# 1. Teste applet:
cosmic-applet-workspaces

# 2. Schau nach "Available workspaces:" in Output
# 3. Falls nicht vorhanden: Environment-spezifisches Problem

# 4. Check environment:
echo "Session: $XDG_SESSION_TYPE"
echo "Desktop: $XDG_CURRENT_DESKTOP"
```

### F: Kann ich D-Bus Interface aktivieren?
**A:** Ja, mit feature flag:
```bash
cargo build --release --features dbus-interface
```

Dann D-Bus Service konfigurieren (siehe [COSMIC_INTEGRATION.md](COSMIC_INTEGRATION.md)).

---

## Entwicklung & Anpassung

### F: Wie ändere ich das UI des Applets?
**A:** Edit `src/main.rs`, Funktion `AppletState::view()`:
```rust
pub fn view(&self) -> Element<Message> {
    // Diese Funktion rendert das UI
    // Ändere hier was angezeigt wird
    column![
        text(format!("Workspace: {}", self.current_workspace))
            .size(14)
    ].into()
}
```

Nach Änderung:
```bash
cargo build --release
cp target/release/cosmic-applet-workspaces ~/.local/bin/
killall cosmic-panel  # Zum Neuladen
```

### F: Wie füge ich Funktionalität hinzu?
**A:** 3 Schritte:

1. **Definiere neue Message:**
   ```rust
   pub enum Message {
       WorkspaceChanged(u32),
       MyNewAction,  // ← Neu
   }
   ```

2. **Handle in update():**
   ```rust
   pub fn update(&mut self, message: Message) {
       match message {
           Message::MyNewAction => {
               // Deine Logik hier
           }
           ...
       }
   }
   ```

3. **Verwende in view():**
   ```rust
   pub fn view(&self) -> Element<Message> {
       button("Click me")
           .on_press(Message::MyNewAction)
           ...
   }
   ```

### F: Wo finde ich noch Beispiele?
**A:** Official cosmic-applets:
- GitHub: https://github.com/pop-os/cosmic-applets
- Beispiele: cosmic-applet-audio, cosmic-applet-network, etc.

---

## Build & Kompilierungs Fragen

### F: Build schlägt fehl mit "git checkout error"
**A:** Git Repository Problem.
```bash
# 1. Update flake:
nix flake update

# 2. Oder wenn nicht Nix:
cargo update

# 3. Clean und retry:
cargo clean
cargo build --release
```

### F: "binary is not in PATH" beim Build
**A:** Rust nicht in PATH während Build.
```bash
# 1. Lade rust environment:
source $HOME/.cargo/env

# 2. Oder nutze rustup:
rustup toolchain install stable
rustup default stable

# 3. Retry:
cargo build --release
```

### F: Build hat Compile-Fehler
**A:** Wahrscheinlich API-Inkompatibilität.
```bash
# 1. Update dependencies:
cargo update

# 2. Check rust version:
rustc --version  # Sollte 1.70+

# 3. Clean und retry:
cargo clean
cargo build --release 2>&1 | tail -50  # Zeige letzten 50 Zeilen
```

---

## Panel & Desktop Integration

### F: Funktioniert das nur mit Cosmic Panel?
**A:** Ja, dieses Applet ist spezifisch für Cosmic Panel. 
- Funktioniert mit anderen Panels/Desktop Environments nicht

Für andere Environments könntet ihr:
- Ein separates Applet für GNOME Shell, KDE Plasma, etc. schreiben
- Module in applet-agnostische Lib extrahieren
- Aber das ist separate Arbeit

### F: Was ist der unterschied zu normalen Desktop Apps?
**A:** Cosmic Applets sind:
- Subprocess von Panel (nicht unabhängig)
- Nutzen libcosmic UI Framework (nicht GTK/Qt)
- Kommunizieren über D-Bus mit System
- Rendern im Panel Layer (nicht als Fenster)
- Haben spezielle Lifecycle Management

### F: Kann ich das Applet als standalone App auch starten?
**A:** Ja, zum Testen:
```bash
cosmic-applet-workspaces  # Läuft als unabhängige Wayland-App

# Mit Debug Output:
RUST_LOG=debug cosmic-applet-workspaces
```

Die wird als eigenes Fenster angezeigt, nicht im Panel.

---

## Systemd & Services

### F: Muss ich systemd Service installieren?
**A:** Nein, nicht erforderlich für base functionality.
- Optional wenn du willst, dass Panel es automatisch managed
- Siehe [INSTALLATION.md](INSTALLATION.md) für systemd Setup

### F: Wie starte ich den Service manuell?
**A:** 
```bash
# Wenn installiert:
systemctl --user start cosmic-applet-workspaces.service

# Check status:
systemctl --user status cosmic-applet-workspaces.service

# View logs:
journalctl -u cosmic-applet-workspaces.service -f
```

---

## Problembehebung Strategie

### Wenn nichts funktioniert:

1. **Versuche QUICK_FIX.md** (5 Min) - löst 99% der Probleme
2. **Versuche INSTALLATION.md** (30 Min) - Schritt-für-Schritt
3. **Prüfe die Desktop-Datei** - 90% der Fehler sind hier
   ```bash
   grep "X-Cosmic" ~/.local/share/applications/com.system76.CosmicAppletWorkspaces.desktop
   # MUSS anzeigen:
   # X-CosmicApplet=true
   # X-HostWaylandDisplay=true
   # X-CosmicShrinkable=true
   # X-OverflowPriority=5
   # X-OverflowMinSize=8
   ```
4. **Starte Panel neu** - kostet nix
5. **Nutze DEVELOPMENT.md** - Debug Tools
6. **Schau in die Logs** - `journalctl -u cosmic-panel -f`

---

## Support & Debugging

### F: Ich bin völlig stecken. Wo bekomme ich Hilfe?
**A:** Sammle diese Infos:
```bash
# 1. Session Type
echo "Session: $XDG_SESSION_TYPE"

# 2. Binary Status
which cosmic-applet-workspaces
ls -lh ~/.local/bin/cosmic-applet-workspaces

# 3. Desktop File Check
grep "X-Cosmic" ~/.local/share/applications/com.system76.CosmicAppletWorkspaces.desktop

# 4. Desktop File Syntax
desktop-file-validate ~/.local/share/applications/com.system76.CosmicAppletWorkspaces.desktop

# 5. Panel Running
pgrep -l cosmic-panel

# 6. Applet Output
cosmic-applet-workspaces 2>&1 | head -20

# 7. Panel Logs
journalctl -u cosmic-panel -n 20 --no-pager
```

Dann:
- Öffne Issue auf GitHub mit diesen Outputs
- Oder vergleiche mit [INSTALLATION.md](INSTALLATION.md) Checklisten

---

## Terminologie Klarifications

### Was ist...

**Cosmic Panel**: Die Task Bar/Panel in Cosmic DE. Hier werden Applets angezeigt.

**Cosmic Applet**: GUI-Komponente die im Panel läuft (wie z.B. Clock, Volume).

**libcosmic**: GUI Framework für Cosmic Applets (auf Basis von iced).

**D-Bus**: Inter-Process Communication System für Linux.

**Wayland**: Modern Display Server (nicht X11).

**Desktop Entry (.desktop)**: Datei die Desktop Apps/Applets beschreibt.

**Desktop Database**: Index von .desktop Dateien (updated mit `update-desktop-database`).

---

Last updated: März 2024  
Für: cosmic-applet-workspaces v1.0.0
