# Nix Development Environment Setup

This project uses Nix flakes for reproducible development environments.

## Prerequisites

- Nix with flakes support (`nix --version` should show 2.4+)

## Setup Instructions

### 1. Enter the development environment

```bash
cd /path/to/cosmic-applet-workspaces
nix develop
```

You should see the "🚀 Cosmic Applet Development Environment" message with tool versions.

### 2. Verify the environment

Once inside the dev environment:
```bash
cargo --version
rustc --version
```

## Troubleshooting

### Flake not recognized?

Update your flake:
```bash
cd /path/to/cosmic-applet-workspaces
nix flake update
```

### Still having issues?

Try cleaning and rebuilding:
```bash
cd /path/to/cosmic-applet-workspaces
rm -rf .direnv
nix develop
cargo clean
cargo build --release
```

## Key Features

- ✅ Rust + Cargo + cargo-watch + cargo-edit
- ✅ Proper terminal capabilities with Bash as default
- ✅ Reproducible development environment via Nix flakes
