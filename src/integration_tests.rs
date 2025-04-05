use leptos::*;
use leptos::prelude::*;
use wasm_bindgen_test::*;
use crate::test_utils::test::*;
use crate::app::App;
use crate::utils::*;
use crate::utils::localStorage::reset_all_storage;
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
    // Use our new enhanced helper
    reset_all_storage();
    // Wait a bit for storage operations to complete
    TimeoutFuture::new(50).await;
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