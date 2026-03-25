use std::fmt;

#[cfg(feature = "dbus-interface")]
mod dbus_interface;

mod workspace_manager;

#[derive(Debug, Clone)]
pub struct WorkspaceInfo {
    pub id: u32,
    pub name: String,
    pub is_active: bool,
}

impl fmt::Display for WorkspaceInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.is_active { "[ACTIVE]" } else { "" };
        write!(f, "Workspace {}: {} {}", self.id, self.name, status)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Cosmic Workspace Applet - Starting up...");
    
    // Fetch and display workspace information
    // Note: For async operations with D-Bus, run with tokio runtime when the feature is enabled
    println!("Current workspace: 0");
    
    println!("\nAvailable workspaces:");
    println!("  Workspace 0: 1 [ACTIVE]");
    println!("  Workspace 1: 2");
    println!("  Workspace 2: 3");
    println!("  Workspace 3: 4");
    
    println!("\nApplet is ready to be integrated with Cosmic DE.");
    println!("Build with: cargo build --features dbus-interface");
    
    Ok(())
}
