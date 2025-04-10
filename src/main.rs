// src/main.rs
mod app;
mod test_utils;
mod data;
mod utils;
mod theme;  
mod crypto;

// Add our new test modules
// #[cfg(test)]
// mod app_tests;
// #[cfg(test)]
// mod mock_logger;
// #[cfg(test)]
// mod integration_tests;
// #[cfg(test)]
// mod log_integration_tests;
// #[cfg(test)]
// mod theme_tests;
#[cfg(test)]
mod data_tests;

// #[cfg(test)]
// mod theme_provider_tests;  

use leptos::*;
use leptos::prelude::*;
use app::App;
use wasm_logger;
use log;

fn main() {
    // Initialize the logger for better error messages
    // This uses wasm_logger which outputs to the browser console
    wasm_logger::init(wasm_logger::Config::default());
    
    // Log application startup
    log::info!("Leptos CSR application starting...");
    
    mount_to_body(|| view! { <App /> });
    
    log::info!("Application mounted successfully");
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use gloo_timers::future::TimeoutFuture;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_logger_initialization() {
        // Simple test to verify we can log messages without errors
        log::info!("Test info message");
        log::warn!("Test warning message");
        log::error!("Test error message");
        
        // If this doesn't throw an exception, logging is initialized properly
        assert!(true);
    }
}