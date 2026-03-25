{
  description = "Cosmic Workspace Applet - A Rust application for monitoring workspaces in Cosmic DE";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rust-bin.stable.latest.default
            pkg-config
            cargo-watch
            cargo-edit
          ];

          shellHook = ''
            # Terminal-Modi konfigurieren
            export TERM=xterm-256color
            stty icanon
            stty echo
            stty erase ^?
            
            echo "🚀 Cosmic Applet Development Environment"
            echo "=========================================="
            echo "Rust: $(rustc --version)"
            echo "Cargo: $(cargo --version)"
            echo ""
            echo "Befehle:"
            echo "  cargo build          - Projekt bauen"
            echo "  cargo build --release - Release-Build"
            echo "  cargo run            - Applet ausführen"
            echo "  cargo watch          - Auto-Rebuild bei Änderungen"
          '';
        };
      }
    );
}
