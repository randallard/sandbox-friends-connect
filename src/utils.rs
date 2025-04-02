use web_sys::Storage;
use uuid::Uuid;
use wasm_bindgen::JsValue;
use log::{error, info, warn};  // Import log macros
use leptos::*;

// Error type for localStorage operations
#[derive(Debug, Clone)]
pub enum StorageError {
    StorageUnavailable,
    GetError(String),
    SetError(String),
    RemoveError(String),
}

impl From<JsValue> for StorageError {
    fn from(js_value: JsValue) -> Self {
        let error_msg = js_value.as_string().unwrap_or_else(|| "Unknown JS error".to_string());
        StorageError::GetError(error_msg)
    }
}

// Helper functions for localStorage
pub fn get_storage() -> Result<Storage, StorageError> {
    web_sys::window()
        .and_then(|win| win.local_storage().ok())
        .flatten()
        .ok_or(StorageError::StorageUnavailable)
}

// Helper function to get an item from localStorage with error handling
pub fn get_storage_item(key: &str) -> Result<Option<String>, StorageError> {
    let storage = get_storage()?;
    storage.get_item(key).map_err(|e| StorageError::GetError(format!("Failed to get '{}': {:?}", key, e)))
}

// Helper function to set an item in localStorage with error handling
pub fn set_storage_item(key: &str, value: &str) -> Result<(), StorageError> {
    let storage = get_storage()?;
    storage.set_item(key, value).map_err(|e| StorageError::SetError(format!("Failed to set '{}': {:?}", key, e)))
}

// Helper function to remove an item from localStorage with error handling
pub fn remove_storage_item(key: &str) -> Result<(), StorageError> {
    let storage = get_storage()?;
    storage.remove_item(key).map_err(|e| StorageError::RemoveError(format!("Failed to remove '{}': {:?}", key, e)))
}

// Uses the uuid crate to generate a player ID
pub fn generate_player_id() -> String {
    Uuid::new_v4().to_string()
}

// Helper function to get or create player ID from localStorage
pub fn get_player_id() -> String {
    match get_storage_item("player_id") {
        Ok(Some(id)) => id,
        _ => {
            // Generate a new player ID and store it
            let new_id = generate_player_id();
            if let Err(err) = set_storage_item("player_id", &new_id) {
                error!("Failed to save player ID: {:?}", err);
            }
            new_id
        }
    }
}

// Helper function to get dark mode preference from localStorage
pub fn get_dark_mode_preference() -> bool {
    match get_storage_item("dark_mode") {
        Ok(Some(val)) => val == "true",
        _ => {
            info!("Could not retrieve dark mode preference, defaulting to light mode");
            false
        }
    }
}

// Helper function to save dark mode preference to localStorage
pub fn save_dark_mode_preference(is_dark: bool) -> Result<(), StorageError> {
    set_storage_item("dark_mode", if is_dark { "true" } else { "false" })
}