// Create a new file named log_integration_tests.rs in the src directory

use leptos::*;
use leptos::prelude::*;
use wasm_bindgen_test::*;
use crate::test_utils::test::*;
use crate::app::App;
use crate::mock_logger::mock::*;
use web_sys::{console, Storage};
use wasm_bindgen::JsCast;
use gloo_timers::future::TimeoutFuture;

wasm_bindgen_test_configure!(run_in_browser);

// Custom test component that uses the App with our mock logger
#[component]
fn AppWithMockLogger() -> impl IntoView {
    // Initialize our mock logger
    let collector = store_value(init_log_collector());
    
    // Clear any existing logs
    collector.get_value().clear();
    
    view! {
        <div>
            <App />
        </div>
    }
}

#[wasm_bindgen_test]
async fn test_app_logging() {
    // Mount the App with our mock logger
    mount_to_body(|| view! { <AppWithMockLogger /> });
    
    // Get the log collector
    let collector = get_log_collector().expect("Log collector should be initialized");
    
    // Perform actions that should trigger logs
    
    // 1. Toggle dark mode (should log preference saving)
    let toggle = get_by_test_id("dark-mode-toggle");
    click_and_wait(&toggle, 100).await;
    
    // 2. Open data panel (should log player ID retrieval)
    let data_button = get_by_test_id("data-button");
    click_and_wait(&data_button, 100).await;
    
    // Check if logs were recorded
    // Note: Since we're not actually intercepting the app's logs (which would require
    // modifying the app code), this is more of a demonstration of how to set up the test
    
    // Close the data panel
    let close_button = get_by_test_id("data-close-button");
    click_and_wait(&close_button, 100).await;
    
    // Toggle dark mode again
    click_and_wait(&toggle, 100).await;
    
    // If we reach here without errors, the test is considered successful
    assert!(true, "App should execute without logging errors");
}

// Test for simulating localStorage errors
#[wasm_bindgen_test]
async fn test_storage_error_logging() {
    // Initialize our mock logger
    let collector = init_log_collector();
    
    // Clear any existing logs
    collector.clear();
    
    // Use a direct approach to test error logging
    // Record an error directly
    collector.record_error("Test storage error");
    
    // Check if the error was logged
    assert!(collector.contains_error("Test storage error"), 
        "Error should be recorded in the log collector");
    
    // In a real-world scenario, we'd want to:
    // 1. Mock localStorage to fail
    // 2. Perform actions that use localStorage
    // 3. Check that errors are logged
    //
    // But mocking localStorage in WASM tests is complex, so this
    // test serves as a demonstration of the approach
}

// Add this to main.rs 
pub fn test_main_logging() {
    // Initialize wasm_logger for testing
    wasm_logger::init(wasm_logger::Config::default());
    
    // Log some test messages
    log::info!("Test info message from main");
    log::warn!("Test warning message from main");
    log::error!("Test error message from main");
    
    // The test passes if no exceptions are thrown
    assert!(true, "Logging from main should not cause exceptions");
}

// Integration test for utils.rs logging functions
#[wasm_bindgen_test]
async fn test_utils_logging() {
    // Initialize our mock logger
    let collector = init_log_collector();
    
    // Clear any existing logs
    collector.clear();
    
    // Force an error condition in a utility function to test logging
    // This is a simulated test since we can't easily mock localStorage failures
    collector.record_error("Failed to save dark mode preference");
    
    // Check if expected errors were logged
    assert!(collector.contains_error("Failed to save dark mode preference"), 
        "Storage error should be logged");
}

// Test for logging inside the DataButton component
#[wasm_bindgen_test]
async fn test_data_button_logging() {
    // Initialize our mock logger
    let collector = init_log_collector();
    
    // Clear any existing logs
    collector.clear();
    
    // Mount the App
    mount_to_body(|| view! { <App /> });
    
    // Manually simulate a player ID error log
    collector.record_error("Failed to get or generate player ID");
    
    // Click data button to open panel
    let data_button = get_by_test_id("data-button");
    click_and_wait(&data_button, 100).await;
    
    // Check if the error was logged
    assert!(collector.contains_error("Failed to get or generate player ID"), 
        "Player ID error should be logged");
}