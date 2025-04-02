// src/main.rs
mod app;
mod test_utils;
mod data;
mod utils;

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