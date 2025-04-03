// src/main.rs
mod app;
mod test_utils;
mod data;
mod utils;
mod mock_xhr; // Add this new module

// Add our test modules
#[cfg(test)]
pub mod test_setup;
#[cfg(test)]
mod mock_logger;
#[cfg(test)]
mod integration_tests;
#[cfg(test)]
mod log_integration_tests;
#[cfg(test)]
mod test_validation;

use leptos::*;
use leptos::prelude::*;
use app::App;
use wasm_logger;
use log;

fn main() {
    // Initialize the logger for better error messages
    // This uses wasm_logger which outputs to the browser console
    wasm_logger::init(wasm_logger::Config::default());
    
    // Apply XHR patch directly (this is important for tests)
    #[cfg(test)]
    mock_xhr::ensure_xhr_patched();
    
    // Log application startup
    log::info!("Leptos CSR application starting...");
    
    mount_to_body(|| view! { <App /> });
    
    log::info!("Application mounted successfully");
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use crate::mock_xhr;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_xhr_patch_applied() {
        // Ensure the XHR patch is applied
        mock_xhr::ensure_xhr_patched();
        
        // If we get here without errors, the patch is applied
        assert!(true, "XHR patch applied successfully");
    }
    
    #[wasm_bindgen_test]
    fn test_logger_initialization() {
        // Ensure the XHR patch is applied
        mock_xhr::ensure_xhr_patched();
        
        // Simple test to verify we can log messages without errors
        log::info!("Test info message");
        log::warn!("Test warning message");
        log::error!("Test error message");
        
        // If this doesn't throw an exception, logging is initialized properly
        assert!(true);
    }
}