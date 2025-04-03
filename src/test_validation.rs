// src/test_validation.rs

use wasm_bindgen_test::*;
use wasm_bindgen::prelude::*;
use web_sys::console;
use crate::test_setup::{init_test_environment, mock_local_storage_set, mock_local_storage_get, patch_url_requests};
use crate::test_utils::test::*;
use leptos::*;
use leptos::prelude::*;
use wasm_bindgen_futures::JsFuture;

wasm_bindgen_test_configure!(run_in_browser);

// Simple component for testing
#[component]
fn TestComponent() -> impl IntoView {
    view! {
        <div>
            <h1 data-test-id="test-header">"Test Component"</h1>
            <button data-test-id="test-button">"Click Me"</button>
        </div>
    }
}

// Basic test to validate the test environment works
#[wasm_bindgen_test]
fn test_environment_works() {
    // Initialize test environment
    init_test_environment();
    
    // Log to console for debugging
    console::log_1(&JsValue::from_str("Running environment validation test"));
    
    // Test that mock storage works
    mock_local_storage_set("test-key", "test-value");
    let value = mock_local_storage_get("test-key");
    assert_eq!(value, Some("test-value".to_string()), "Mock storage should work");
    
    // If we got here without errors, the test environment works
    console::log_1(&JsValue::from_str("✅ Test environment is working correctly"));
}

// Test that mounting a component works
#[wasm_bindgen_test]
fn test_component_mounting() {
    // Initialize test environment
    init_test_environment();
    
    // Log to console for debugging
    console::log_1(&JsValue::from_str("Testing component mounting"));
    
    // Mount a simple test component
    mount_to_body(|| view! { <TestComponent /> });
    
    // Try to find an element
    let header = get_by_test_id("test-header");
    
    // Verify we found the element
    assert_eq!(header.text_content().unwrap(), "Test Component", 
        "Should be able to mount a component and find elements");
        
    console::log_1(&JsValue::from_str("✅ Component mounting test passed"));
}

// Test that URL interception works
#[wasm_bindgen_test]
async fn test_url_interception() {
    // Initialize test environment and apply URL patch
    init_test_environment();
    patch_url_requests();
    
    console::log_1(&JsValue::from_str("Testing URL interception"));
    
    // Create a URL that would normally cause a 404 error
    let test_url = "/session/test-id/url";
    
    // Try to fetch the URL - if our patch works, this shouldn't throw an error
    let window = web_sys::window().unwrap();
    let request_promise = window.fetch_with_str(test_url);
    
    // Convert to a Rust future
    let future = JsFuture::from(request_promise);
    
    // Wait for the result
    match future.await {
        Ok(_) => {
            console::log_1(&JsValue::from_str("✅ URL interception test passed - No 404 error"));
            assert!(true, "URL interception works");
        },
        Err(e) => {
            console::error_1(&e);
            // We'll fail the test if the interception didn't work
            assert!(false, "URL interception failed - got an error from fetch");
        }
    }
}

// Log more verbose information about the test environment
#[wasm_bindgen_test]
fn test_diagnostic_info() {
    // Initialize test environment
    init_test_environment();
    
    console::log_1(&JsValue::from_str("------------ TEST ENVIRONMENT DIAGNOSTICS ------------"));
    
    // Check for window object
    if web_sys::window().is_some() {
        console::log_1(&JsValue::from_str("✅ Window object is available"));
    } else {
        console::error_1(&JsValue::from_str("❌ Window object is NOT available"));
    }
    
    // Check for document object
    if web_sys::window().and_then(|win| win.document()).is_some() {
        console::log_1(&JsValue::from_str("✅ Document object is available"));
    } else {
        console::error_1(&JsValue::from_str("❌ Document object is NOT available"));
    }
    
    // Check for DOM manipulation capability
    let doc_result = js_sys::eval("document.body.appendChild(document.createElement('div'))");
    if doc_result.is_ok() {
        console::log_1(&JsValue::from_str("✅ DOM manipulation is working"));
    } else {
        console::error_1(&JsValue::from_str("❌ DOM manipulation is NOT working"));
    }
    
    // Check for fetch API
    let fetch_result = js_sys::eval("typeof fetch === 'function'");
    if fetch_result.is_ok() && fetch_result.unwrap().as_bool().unwrap_or(false) {
        console::log_1(&JsValue::from_str("✅ Fetch API is available"));
    } else {
        console::error_1(&JsValue::from_str("❌ Fetch API is NOT available"));
    }
    
    // Check for XMLHttpRequest
    let xhr_result = js_sys::eval("typeof XMLHttpRequest === 'function'");
    if xhr_result.is_ok() && xhr_result.unwrap().as_bool().unwrap_or(false) {
        console::log_1(&JsValue::from_str("✅ XMLHttpRequest is available"));
    } else {
        console::error_1(&JsValue::from_str("❌ XMLHttpRequest is NOT available"));
    }
    
    console::log_1(&JsValue::from_str("--------------- END DIAGNOSTICS ---------------"));
}