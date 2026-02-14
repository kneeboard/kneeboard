use definition::WorkspaceConfig;
use gloo::storage::{SessionStorage, Storage};

const WORKSPACE_KEY: &str = "kneeboard_workspace_v1";

pub fn save_workspace_to_local_storage(workspace: &WorkspaceConfig) {
    if let Err(e) = SessionStorage::set(WORKSPACE_KEY, workspace) {
        gloo_console::error!("Failed to save workspace:", e.to_string());
    }
}

pub fn load_workspace_from_local_storage() -> Option<WorkspaceConfig> {
    SessionStorage::get(WORKSPACE_KEY).ok()
}

pub fn clear_workspace_from_local_storage() {
    SessionStorage::delete(WORKSPACE_KEY);
}
