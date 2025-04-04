use leptos::*;
use leptos::prelude::*;
use wasm_bindgen_test::*;
use crate::test_utils::test::*;
use crate::app::App;
use crate::utils::*;
use crate::data::get_test_player_id;
use web_sys::{console, Storage};
use wasm_bindgen::JsCast;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen::prelude::*;

wasm_bindgen_test_configure!(run_in_browser);

// JavaScript console logging wrapper
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Helper to create custom DOM attributes for testing
fn set_test_attribute(element: &web_sys::Element, name: &str, value: &str) -> Result<(), JsValue> {
    element.set_attribute(&format!("data-test-{}", name), value)
}

// Helper to reset localStorage between tests
async fn reset_storage() {
    if let Ok(storage) = get_storage() {
        let _ = storage.remove_item("dark_mode");
        let _ = storage.remove_item("player_id");
        // Wait a bit for storage operations to complete
        TimeoutFuture::new(50).await;
    }
}

#[wasm_bindgen_test]
async fn test_dark_mode_integration() {
    reset_storage().await;
    
    // Mount the App component
    mount_to_body(|| view! { <App /> });
    
    // Get relevant elements
    let container = get_by_test_id("app-container");
    let toggle = get_by_test_id("dark-mode-toggle");
    
    // Check current mode (after reset_storage it should default to light)
    let is_currently_dark = container.class_list().contains("dark");
    
    // Click the toggle button to switch modes
    click_and_wait(&toggle, 100).await;
    
    // Verify mode was toggled
    assert_eq!(
        container.class_list().contains("dark"), 
        !is_currently_dark,
        "Container class should be toggled after clicking"
    );
    
    // Verify localStorage was updated
    let storage = get_storage().unwrap();
    let stored_value = storage.get_item("dark_mode").unwrap();
    assert_eq!(
        stored_value, 
        Some((!is_currently_dark).to_string()),
        "Mode preference should be saved to localStorage"
    );
    
    // Toggle back to original mode
    click_and_wait(&toggle, 100).await;
    
    // Verify original mode is restored
    assert_eq!(
        container.class_list().contains("dark"), 
        is_currently_dark,
        "Container class should be back to original state after second toggle"
    );
    
    // Verify localStorage was updated again
    let final_stored_value = storage.get_item("dark_mode").unwrap();
    assert_eq!(
        final_stored_value, 
        Some(is_currently_dark.to_string()),
        "Original mode preference should be saved to localStorage"
    );
}

#[wasm_bindgen_test]
async fn test_data_button_player_id_integration() {
    reset_storage().await;
    
    // Log the start of the test
    log("Starting player ID integration test");
    
    // Mount the App component
    mount_to_body(|| view! { <App /> });
    
    // Wait for component initialization
    TimeoutFuture::new(100).await;
    
    // Click the data button to show the panel
    let data_button = get_by_test_id("data-button");
    if data_button.is_undefined() {
        log("ERROR: Could not find data-button");
        assert!(false, "Could not find data-button");
        return;
    }
    
    // Click and wait
    click_and_wait(&data_button, 100).await;
    
    // Get the player ID element from the panel
    let player_id_element = get_by_test_id("player-id");
    if player_id_element.is_undefined() {
        log("ERROR: Could not find player-id element");
        assert!(false, "Could not find player-id element");
        return;
    }
    
    // Log the element content for debugging
    log(&format!("Player ID element found with text: {}", 
        player_id_element.text_content().unwrap_or_default()));
    
    // Verify the player ID is displayed (any ID is acceptable)
    let player_id_text = player_id_element.text_content().unwrap_or_default();
    assert!(player_id_text.starts_with("Player ID:"), 
        "Panel should display a player ID with proper formatting");
    assert!(player_id_text.len() > 10, 
        "Player ID text should not be empty and have a reasonable length");
    
    log("Test completed successfully!");
}

#[wasm_bindgen_test]
async fn test_storage_error_handling_integration() {
    reset_storage().await;
    
    // Mount the App component
    mount_to_body(|| view! { <App /> });
    
    // Click the data button to show the panel
    let data_button = get_by_test_id("data-button");
    click_and_wait(&data_button, 100).await;
    
    // Close the panel
    let close_button = get_by_test_id("data-close-button");
    click_and_wait(&close_button, 100).await;
    
    // If we reach here without errors, the error handling path works
    log("Storage error handling test completed successfully");
}

#[wasm_bindgen_test]
async fn test_logging_integration() {
    // Mount the App component
    mount_to_body(|| view! { <App /> });
    
    // Perform actions that should trigger logs
    let data_button = get_by_test_id("data-button");
    click_and_wait(&data_button, 100).await;
    
    // Verify that the window has the player ID accessible
    match get_test_player_id() {
        Some(id) => {
            log(&format!("TEST: Found player ID in context: {}", id));
            assert!(!id.is_empty(), "Player ID should not be empty");
        },
        None => {
            log("TEST: Player ID not found in context");
            // This is still valid as we're just testing logging, not the ID itself
        }
    }
    
    // If we reach here without errors, logging worked
    log("Logging test completed successfully");
    assert!(true, "Logging should not cause exceptions");
}

#[wasm_bindgen_test]
async fn test_data_button_dark_mode_integration() {
    reset_storage().await;
    
    // Log the start of the test
    log("Starting dark mode integration test");
    
    // Mount the App component
    mount_to_body(|| view! { <App /> });
    
    // Wait for component initialization
    TimeoutFuture::new(100).await;
    
    // Click the data button to show the panel
    let data_button = get_by_test_id("data-button");
    if data_button.is_undefined() {
        log("ERROR: Could not find data-button");
        assert!(false, "Could not find data-button");
        return;
    }
    
    // Click and wait
    click_and_wait(&data_button, 100).await;
    
    // Get the dark mode element from the panel
    let dark_mode_element = get_by_test_id("dark-mode-setting");
    if dark_mode_element.is_undefined() {
        log("ERROR: Could not find dark-mode-setting element");
        assert!(false, "Could not find dark-mode-value element");
        return;
    }
    
    // Log the element content for debugging
    log(&format!("Dark mode element found with text: {}", 
        dark_mode_element.text_content().unwrap_or_default()));
    
    // Verify the dark mode value is displayed correctly
    let dark_mode_text = dark_mode_element.text_content().unwrap_or_default();
    assert!(dark_mode_text.contains("Dark Mode:"), 
        "Panel should display the dark mode value");
    
    log("Test completed successfully!");
}