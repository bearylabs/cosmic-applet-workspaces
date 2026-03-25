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
        rustToolchain = pkgs.rust-bin.stable.latest.default;
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "cosmic-applet-workspaces";
          version = "0.1.0";
          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = with pkgs; [
            rustToolchain
            pkg-config
          ];

          buildInputs = with pkgs; [
            glib
            cairo
            pango
            gtk3
          ];

          meta = with pkgs.lib; {
            description = "A workspace indicator applet for Cosmic DE";
            homepage = "https://github.com/yourusername/cosmic-applet";
            license = licenses.gpl3;
            maintainers = [ ];
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            rust-analyzer
            cargo-watch
            cargo-edit
            cargo-expand
            pkg-config
            glib
            cairo
            pango
            gtk3
            libxkbcommon
            libwayland
            wayland-protocols
            dbus
          ];

          shellHook = ''
            echo "Cosmic Applet Development Environment"
            echo "======================================="
            echo "Rust version: $(rustc --version)"
            echo "Cargo version: $(cargo --version)"
            echo ""
            echo "Available commands:"
            echo "  cargo build          - Build the project"
            echo "  cargo build --release - Release build"
            echo "  cargo run            - Run the applet"
            echo "  cargo test           - Run tests"
            echo "  cargo watch          - Watch for changes and rebuild"
          '';
        };

        apps.default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/cosmic-applet-workspaces";
        };
      }
    );
}
