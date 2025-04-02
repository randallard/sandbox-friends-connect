// src/main.rs
mod app;
mod test_utils;
mod data;
mod utils; // Add the new utils module

use leptos::*;
use leptos::prelude::*;

use app::App;

fn main() {
    mount_to_body(|| view! { <App /> });
}