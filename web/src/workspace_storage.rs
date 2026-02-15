use definition::ProfileConfig;
use gloo::storage::{SessionStorage, Storage};

const PROFILE_KEY: &str = "kneeboard_workspace_v1";

pub fn save_profile_to_local_storage(profile: &ProfileConfig) {
    if let Err(e) = SessionStorage::set(PROFILE_KEY, profile) {
        gloo_console::error!("Failed to save profile:", e.to_string());
    }
}

pub fn load_profile_from_local_storage() -> Option<ProfileConfig> {
    SessionStorage::get(PROFILE_KEY).ok()
}

pub fn clear_profile_from_local_storage() {
    SessionStorage::delete(PROFILE_KEY);
}
