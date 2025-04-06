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
            // Generate a default preference (light mode) and store it
            let default_preference = false; // default to light mode
            if let Err(err) = set_storage_item("dark_mode", if default_preference { "true" } else { "false" }) {
                error!("Failed to save default dark mode preference: {:?}", err);
            }
            info!("No dark mode preference found, defaulting to light mode");
            default_preference
        }
    }
}

// Helper function to save dark mode preference to localStorage
pub fn save_dark_mode_preference(is_dark: bool) -> Result<(), StorageError> {
    set_storage_item("dark_mode", if is_dark { "true" } else { "false" })
}

// Add a new localStorage module with test-friendly helpers
pub mod localStorage {
    use super::*;
    use wasm_bindgen::JsValue;

    /// Safely accesses the local storage and performs an operation that returns a Result
    pub fn with_local_storage<F, T>(f: F) -> Result<T, JsValue>
    where
        F: FnOnce(&web_sys::Storage) -> Result<T, JsValue>,
    {
        let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window found"))?;
        let storage = window.local_storage()?.ok_or_else(|| JsValue::from_str("No localStorage found"))?;
        f(&storage)
    }

    /// Reset a localStorage item by removing it
    pub fn reset_storage_item(key: &str) -> Result<(), JsValue> {
        with_local_storage(|storage| storage.remove_item(key))
    }

    /// Set a localStorage item
    pub fn set_storage_item(key: &str, value: &str) -> Result<(), JsValue> {
        with_local_storage(|storage| storage.set_item(key, value))
    }

    /// Get a localStorage item
    pub fn get_storage_item(key: &str) -> Result<Option<String>, JsValue> {
        with_local_storage(|storage| storage.get_item(key))
    }

    /// Test helper to reset localStorage for tests
    pub fn reset_theme_storage() {
        let _ = reset_storage_item("dark_mode");
    }

    /// Test helper to reset all app storage 
    pub fn reset_all_storage() {
        let _ = reset_storage_item("dark_mode");
        let _ = reset_storage_item("player_id");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use crate::test_utils::test::*;
    use std::rc::Rc;
    use leptos::prelude::*;
    use web_sys::{Element, HtmlElement};
    use wasm_bindgen::JsCast;
    use gloo_timers::future::TimeoutFuture;
    use web_sys::console;

    wasm_bindgen_test_configure!(run_in_browser);

    // Helper function to reset localStorage for tests
    async fn reset_storage() {
        if let Ok(storage) = get_storage() {
            let _ = storage.remove_item("dark_mode");
            let _ = storage.remove_item("player_id");
            // Wait a bit for storage operations to complete
            TimeoutFuture::new(50).await;
        }
    }

    // Test component to observe logging
    #[component]
    fn LogTestComponent() -> impl IntoView {
        let log_action = move |_| {
            info!("Info log message from test");
            warn!("Warning log message from test");
            error!("Error log message from test");
        };

        view! {
            <div>
                <button 
                    data-test-id="log-button"
                    on:click=log_action
                >
                    "Log Test"
                </button>
            </div>
        }
    }

    #[wasm_bindgen_test]
    async fn test_get_storage_returns_storage() {
        let storage_result = get_storage();
        assert!(storage_result.is_ok(), "Should successfully get localStorage");
    }

    #[wasm_bindgen_test]
    async fn test_get_storage_item() {
        reset_storage().await;

        // First, set a test item
        let test_key = "test_key";
        let test_value = "test_value";
        let storage = get_storage().unwrap();
        let _ = storage.set_item(test_key, test_value);

        // Then test getting it
        let result = get_storage_item(test_key);
        assert!(result.is_ok(), "Should not return an error");
        assert_eq!(result.unwrap(), Some(test_value.to_string()), 
            "Should retrieve the correct value");

        // Clean up
        let _ = storage.remove_item(test_key);
    }

    #[wasm_bindgen_test]
    async fn test_set_storage_item() {
        reset_storage().await;

        let test_key = "test_key";
        let test_value = "test_value";

        // Test setting an item
        let result = set_storage_item(test_key, test_value);
        assert!(result.is_ok(), "Should successfully set item in localStorage");

        // Verify it was set correctly
        let storage = get_storage().unwrap();
        let stored_value = storage.get_item(test_key).unwrap();
        assert_eq!(stored_value, Some(test_value.to_string()), 
            "Value should be stored correctly in localStorage");

        // Clean up
        let _ = storage.remove_item(test_key);
    }

    #[wasm_bindgen_test]
    async fn test_remove_storage_item() {
        reset_storage().await;

        let test_key = "test_key";
        let test_value = "test_value";

        // First set an item
        let storage = get_storage().unwrap();
        let _ = storage.set_item(test_key, test_value);

        // Test removing it
        let result = remove_storage_item(test_key);
        assert!(result.is_ok(), "Should successfully remove item from localStorage");

        // Verify it was removed
        let stored_value = storage.get_item(test_key).unwrap();
        assert_eq!(stored_value, None, "Item should be removed from localStorage");
    }

    #[wasm_bindgen_test]
    async fn test_generate_player_id() {
        let id1 = generate_player_id();
        let id2 = generate_player_id();

        // Test that IDs are non-empty
        assert!(!id1.is_empty(), "Generated ID should not be empty");
        
        // Test that IDs are unique
        assert_ne!(id1, id2, "Generated IDs should be unique");
        
        // Test that IDs are valid UUIDs (36 characters with 4 hyphens)
        assert_eq!(id1.len(), 36, "Generated ID should be 36 characters long");
        assert_eq!(id1.chars().filter(|&c| c == '-').count(), 4, 
            "Generated ID should contain 4 hyphens");
    }

    #[wasm_bindgen_test]
    async fn test_get_player_id() {
        reset_storage().await;

        // First call should generate a new ID
        let id1 = get_player_id();
        assert!(!id1.is_empty(), "get_player_id should return a non-empty ID");
        
        // Second call should return the same ID
        let id2 = get_player_id();
        assert_eq!(id1, id2, "get_player_id should return the same ID on subsequent calls");

        // Verify it was stored in localStorage
        let storage = get_storage().unwrap();
        let stored_id = storage.get_item("player_id").unwrap();
        assert_eq!(stored_id, Some(id1), "ID should be stored in localStorage");
    }

    #[wasm_bindgen_test]
    async fn test_get_dark_mode_preference_default() {
        reset_storage().await;

        // With no stored preference, should default to false (light mode)
        let preference = get_dark_mode_preference();
        assert_eq!(preference, false, "Default dark mode preference should be false");
    }

    #[wasm_bindgen_test]
    async fn test_get_dark_mode_preference_stored() {
        reset_storage().await;

        // Set a preference
        let storage = get_storage().unwrap();
        let _ = storage.set_item("dark_mode", "true");

        // Should retrieve the stored preference
        let preference = get_dark_mode_preference();
        assert_eq!(preference, true, "Should retrieve stored dark mode preference");
    }

    #[wasm_bindgen_test]
    async fn test_save_dark_mode_preference() {
        reset_storage().await;

        // Test saving dark mode preference
        let result = save_dark_mode_preference(true);
        assert!(result.is_ok(), "Should successfully save dark mode preference");

        // Verify it was stored correctly
        let storage = get_storage().unwrap();
        let stored_value = storage.get_item("dark_mode").unwrap();
        assert_eq!(stored_value, Some("true".to_string()), 
            "Dark mode preference should be stored correctly");

        // Test saving light mode preference
        let result = save_dark_mode_preference(false);
        assert!(result.is_ok(), "Should successfully save light mode preference");

        // Verify it was stored correctly
        let stored_value = storage.get_item("dark_mode").unwrap();
        assert_eq!(stored_value, Some("false".to_string()), 
            "Light mode preference should be stored correctly");
    }

    #[wasm_bindgen_test]
    async fn test_logging() {
        // Mount the test component
        mount_to_body(|| view! { <LogTestComponent /> });
        
        // Get the log button
        let log_button = get_by_test_id("log-button");
        
        // We can't directly assert on console logs, but we can verify the component works
        click_and_wait(&log_button, 50).await;
        
        // If we reach here without errors, the test passes
        // This is mostly to ensure the logging code doesn't throw exceptions
        assert!(true);
    }

    // Test for error handling in localStorage
    #[wasm_bindgen_test]
    async fn test_storage_error_conversion() {
        // Create a JS error
        let js_error = JsValue::from_str("Test JS error");
        
        // Convert to our StorageError
        let storage_error = StorageError::from(js_error);
        
        // Ensure error conversion worked
        match storage_error {
            StorageError::GetError(msg) => {
                assert_eq!(msg, "Test JS error", "JS error should be converted to StorageError correctly");
            },
            _ => panic!("JS error should be converted to GetError variant")
        }
    }
    
    // Tests for the new localStorage module helpers
    #[wasm_bindgen_test]
    async fn test_with_local_storage() {
        let test_key = "test_with_key";
        let test_value = "test_with_value";
        
        // Test that with_local_storage works for multiple operations
        let result = localStorage::with_local_storage(|storage| {
            let _ = storage.remove_item(test_key)?;
            let _ = storage.set_item(test_key, test_value)?;
            storage.get_item(test_key)
        });
        
        assert!(result.is_ok(), "with_local_storage should execute successfully");
        assert_eq!(result.unwrap().unwrap(), test_value, "Should retrieve the correct value");
        
        // Clean up
        let _ = localStorage::reset_storage_item(test_key);
    }
    
    #[wasm_bindgen_test]
    async fn test_localStorage_helpers() {
        // Use "dark_mode" instead of "test_localStorage_key"
        let test_key = "dark_mode"; 
        let test_value = "test_localStorage_value";
        
        // Reset the key first
        let reset_result = localStorage::reset_storage_item(test_key);
        assert!(reset_result.is_ok(), "reset_storage_item should execute successfully");
        
        // Set the value using the new helper
        let set_result = localStorage::set_storage_item(test_key, test_value);
        assert!(set_result.is_ok(), "set_storage_item should execute successfully");
        
        // Get the value using the new helper
        let get_result = localStorage::get_storage_item(test_key);
        assert!(get_result.is_ok(), "get_storage_item should execute successfully");
        assert_eq!(get_result.unwrap().unwrap(), test_value, "Should retrieve the correct value");
        
        // Reset all app storage
        localStorage::reset_all_storage();
        
        // Verify the item was removed
        let get_after_reset = localStorage::get_storage_item(test_key);
        assert_eq!(get_after_reset.unwrap(), None, "Item should be removed after reset_all_storage");
    }
}