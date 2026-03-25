#[cfg(feature = "dbus-interface")]
use zbus::{interface, Connection};
use tracing::info;

/// D-Bus interface for Cosmic Workspace Applet
/// Exposes workspace information and listens for workspace changes
#[cfg(feature = "dbus-interface")]
#[interface(name = "org.Cosmic.Applet.Workspaces")]
pub struct WorkspaceAppletInterface {
    current_workspace: u32,
}

#[cfg(feature = "dbus-interface")]
#[interface]
impl WorkspaceAppletInterface {
    /// Get the current active workspace ID
    #[zbus(property)]
    pub fn current_workspace(&self) -> u32 {
        self.current_workspace
    }

    /// Signal when workspace changes
    #[zbus(signal)]
    pub async fn workspace_changed(&self, workspace_id: u32) -> zbus::Result<()>;

    /// Get list of available workspaces
    pub fn get_workspaces(&self) -> Vec<String> {
        vec!["1".into(), "2".into(), "3".into(), "4".into()]
    }

    /// Switch to a specific workspace
    pub fn switch_workspace(&mut self, workspace_id: u32) -> zbus::Result<bool> {
        info!("D-Bus: Switching to workspace {}", workspace_id);
        self.current_workspace = workspace_id;
        Ok(true)
    }
}

#[cfg(feature = "dbus-interface")]
pub async fn start_dbus_interface() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting D-Bus interface for Cosmic Workspace Applet");

    let connection = Connection::session().await?;
    
    // Request service name on D-Bus
    connection
        .request_name("org.Cosmic.Applet.Workspaces")
        .await?;

    let iface = WorkspaceAppletInterface {
        current_workspace: 0,
    };

    // Register the interface at the object path
    connection
        .object_server()
        .at("/org/Cosmic/Applet/Workspaces", iface)
        .await?;

    info!("D-Bus interface registered successfully");

    // Keep the connection alive
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}

#[cfg(not(feature = "dbus-interface"))]
pub async fn start_dbus_interface() -> Result<(), Box<dyn std::error::Error>> {
    info!("D-Bus interface not compiled (feature 'dbus-interface' not enabled)");
    Ok(())
}
