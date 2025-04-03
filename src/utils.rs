use web_sys::Storage;
use uuid::Uuid;
use wasm_bindgen::JsValue;
use log::{error, info, warn};
use leptos::*;

#[cfg(test)]
use crate::test_setup::{mock_local_storage_get, mock_local_storage_set, mock_local_storage_remove};

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

// Helper functions for localStorage with test mocking support
pub fn get_storage() -> Result<Storage, StorageError> {
    match web_sys::window() {
        Some(win) => {
            match win.local_storage() {
                Ok(Some(storage)) => Ok(storage),
                Ok(None) => {
                    warn!("localStorage is not available");
                    Err(StorageError::StorageUnavailable)
                },
                Err(e) => {
                    error!("Error accessing localStorage: {:?}", e);
                    Err(StorageError::StorageUnavailable)
                }
            }
        },
        None => {
            warn!("Window object is not available");
            Err(StorageError::StorageUnavailable)
        }
    }
}

// Get item from localStorage with test mock support
pub fn get_storage_item(key: &str) -> Result<Option<String>, StorageError> {
    #[cfg(test)]
    {
        // In test environment, use our mock storage
        return Ok(mock_local_storage_get(key));
    }
    
    #[cfg(not(test))]
    {
        match get_storage() {
            Ok(storage) => {
                match storage.get_item(key) {
                    Ok(value) => {
                        info!("Retrieved {} from storage: {:?}", key, value);
                        Ok(value)
                    },
                    Err(e) => {
                        error!("Failed to get '{}' from localStorage: {:?}", key, e);
                        Err(StorageError::GetError(format!("Failed to get '{}': {:?}", key, e)))
                    }
                }
            },
            Err(e) => {
                warn!("Cannot get '{}' from storage: {:?}", key, e);
                Err(e)
            }
        }
    }
}

// Set item in localStorage with test mock support
pub fn set_storage_item(key: &str, value: &str) -> Result<(), StorageError> {
    #[cfg(test)]
    {
        // In test environment, use our mock storage
        mock_local_storage_set(key, value);
        return Ok(());
    }
    
    #[cfg(not(test))]
    {
        match get_storage() {
            Ok(storage) => {
                match storage.set_item(key, value) {
                    Ok(_) => {
                        info!("Successfully saved '{}' to localStorage", key);
                        Ok(())
                    },
                    Err(e) => {
                        error!("Failed to set '{}' in localStorage: {:?}", key, e);
                        Err(StorageError::SetError(format!("Failed to set '{}': {:?}", key, e)))
                    }
                }
            },
            Err(e) => {
                warn!("Cannot set '{}' in storage: {:?}", key, e);
                Err(e)
            }
        }
    }
}

// Remove item from localStorage with test mock support
pub fn remove_storage_item(key: &str) -> Result<(), StorageError> {
    #[cfg(test)]
    {
        // In test environment, use our mock storage
        mock_local_storage_remove(key);
        return Ok(());
    }
    
    #[cfg(not(test))]
    {
        match get_storage() {
            Ok(storage) => {
                match storage.remove_item(key) {
                    Ok(_) => {
                        info!("Successfully removed '{}' from localStorage", key);
                        Ok(())
                    },
                    Err(e) => {
                        error!("Failed to remove '{}' from localStorage: {:?}", key, e);
                        Err(StorageError::RemoveError(format!("Failed to remove '{}': {:?}", key, e)))
                    }
                }
            },
            Err(e) => {
                warn!("Cannot remove '{}' from storage: {:?}", key, e);
                Err(e)
            }
        }
    }
}

// Generate a new player ID
pub fn generate_player_id() -> String {
    Uuid::new_v4().to_string()
}

// Get or create player ID 
pub fn get_player_id() -> String {
    match get_storage_item("player_id") {
        Ok(Some(id)) => {
            info!("Retrieved existing player ID from storage");
            id
        },
        _ => {
            // Generate a new player ID and store it
            let new_id = generate_player_id();
            info!("Generated new player ID: {}", new_id);
            
            if let Err(err) = set_storage_item("player_id", &new_id) {
                error!("Failed to save player ID: {:?}", err);
            }
            new_id
        }
    }
}

// Get dark mode preference
pub fn get_dark_mode_preference() -> bool {
    match get_storage_item("dark_mode") {
        Ok(Some(val)) => {
            let is_dark = val == "true";
            info!("Retrieved dark mode preference: {}", is_dark);
            is_dark
        },
        _ => {
            info!("No dark mode preference found, defaulting to light mode");
            false
        }
    }
}

// Save dark mode preference
pub fn save_dark_mode_preference(is_dark: bool) -> Result<(), StorageError> {
    info!("Saving dark mode preference: {}", is_dark);
    set_storage_item("dark_mode", if is_dark { "true" } else { "false" })
}