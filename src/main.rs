use cosmic::applet;
use cosmic::iced::wayland::event_loop::EventLoopWindowTarget;
use cosmic::iced::{window, Size};
use cosmic::prelude::*;
use std::fmt;
use tracing::{error, info};

#[cfg(feature = "dbus-interface")]
mod dbus_interface;

mod workspace_manager;

use workspace_manager::WorkspaceManager;

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

#[derive(Debug, Clone, Copy)]
pub enum Message {
    WorkspaceChanged(u32),
    RefreshWorkspaces,
    SwitchWorkspace(u32),
}

pub struct AppletState {
    current_workspace: u32,
    workspaces: Vec<WorkspaceInfo>,
}

impl AppletState {
    pub fn new() -> Self {
        Self {
            current_workspace: 0,
            workspaces: vec![],
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::WorkspaceChanged(id) => {
                info!("Workspace changed to: {}", id);
                self.current_workspace = id;
            }
            Message::RefreshWorkspaces => {
                if let Ok((workspaces, current)) = WorkspaceManager::get_workspaces() {
                    self.workspaces = workspaces;
                    self.current_workspace = current;
                }
            }
            Message::SwitchWorkspace(id) => {
                info!("Requesting switch to workspace: {}", id);
                if let Err(e) = WorkspaceManager::switch_workspace(id) {
                    error!("Failed to switch workspace: {}", e);
                }
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let current_info = self
            .workspaces
            .iter()
            .find(|w| w.is_active)
            .map(|w| w.name.clone())
            .unwrap_or_else(|| format!("{}", self.current_workspace));

        column![text(format!("Workspace: {}", current_info))
            .size(14)
            .width(70)]
        .into()
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Starting Cosmic Workspace Applet");

    let flags = applet::Flags::default();

    applet::run::<AppletState, Message, _>(
        "Cosmic Workspace Applet",
        Some(flags),
        AppletState::new(),
        |_siv: &mut AppletState| async move { Message::RefreshWorkspaces },
        |state: &AppletState| state.view(),
        |state: &mut AppletState, message: Message| {
            state.update(message);
        },
        on_window_resized,
    )?;

    Ok(())
}

fn on_window_resized(
    _siv: u32,
    _window_target: &EventLoopWindowTarget<Message>,
) -> (Size, window::Level) {
    (Size::new(80.0, 32.0), window::Level::default())
}
