use leptos::*;
use leptos::prelude::*;
use wasm_bindgen_test::*;
use crate::test_utils::test::*;
use crate::app::App;
use crate::utils::*;
use web_sys::console;
use wasm_bindgen::prelude::*;
use crate::test_setup::{mock_local_storage_clear, mock_local_storage_set, wait_for_dom_update};
use crate::mock_xhr;

wasm_bindgen_test_configure!(run_in_browser);

// Helper to safely reset the mock storage
async fn safe_reset_storage() {
    // Ensure XHR patch is applied first
    mock_xhr::ensure_xhr_patched();
    
    // Clear the mock storage (this doesn't touch the actual localStorage)
    mock_local_storage_clear();
    
    // Wait a brief moment for any pending operations
    wait_for_dom_update().await;
}

#[wasm_bindgen_test]
async fn test_dark_mode_integration() {
    // Ensure XHR patch is applied
    mock_xhr::ensure_xhr_patched();
    
    // Reset our mock storage
    safe_reset_storage().await;
    
    // Mount the App component
    mount_to_body(|| view! { <App /> });
    
    // Wait for the component to fully render
    wait_for_dom_update().await;
    
    // Get relevant elements
    let container = get_by_test_id("app-container");
    let toggle = get_by_test_id("dark-mode-toggle");
    
    // Verify initial state (light mode)
    assert!(!container.class_list().contains("dark"), 
        "Container should start in light mode");
    
    // Toggle to dark mode with our improved click helper
    click_and_wait(&toggle, 100).await;
    
    // Wait for updates to propagate
    wait_for_dom_update().await;
    
    // Verify dark mode is active
    assert!(container.class_list().contains("dark"), 
        "Container should be in dark mode after toggle");
    
    // Toggle back to light mode
    click_and_wait(&toggle, 100).await;
    
    // Wait for updates to propagate
    wait_for_dom_update().await;
    
    // Verify light mode is active
    assert!(!container.class_list().contains("dark"), 
        "Container should be back in light mode after second toggle");
}

// [rest of tests omitted for brevity - make sure to add mock_xhr::ensure_xhr_patched() to each test]