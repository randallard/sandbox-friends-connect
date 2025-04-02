use leptos::*;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{self, Window, js_sys};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["localStorage"], js_name = getItem)]
    fn get_item(key: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = ["localStorage"], js_name = setItem)]
    fn set_item(key: &str, value: &str);
}

// Helper function to generate a GUID for player ID
fn generate_player_id() -> String {
    // Implementation of UUID v4 (random) generation
    let window = web_sys::window().expect("Failed to get window");
    
    // Try to use the Web Crypto API if available
    if let Some(crypto) = window.crypto() {
        // Generate random bytes for UUID v4
        let mut buffer = [0u8; 16];
        if crypto.get_random_values_with_u8_array(&mut buffer).is_ok() {
            // UUID v4 format: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
            // Set version (4) and variant bits
            buffer[6] = (buffer[6] & 0x0f) | 0x40; // Version 4
            buffer[8] = (buffer[8] & 0x3f) | 0x80; // Variant 1
            
            // Format the UUID
            format!(
                "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                buffer[0], buffer[1], buffer[2], buffer[3],
                buffer[4], buffer[5],
                buffer[6], buffer[7],
                buffer[8], buffer[9],
                buffer[10], buffer[11], buffer[12], buffer[13], buffer[14], buffer[15]
            )
        } else {
            // Fallback to a simpler approach
            fallback_uuid_generation(&window)
        }
    } else {
        // Web Crypto API not available, use fallback
        fallback_uuid_generation(&window)
    }
}

// Fallback UUID generation when Web Crypto API is not available
fn fallback_uuid_generation(window: &Window) -> String {
    // Template for UUID v4: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
    let template = "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx";
    let hex_chars = [
        '0', '1', '2', '3', '4', '5', '6', '7',
        '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'
    ];
    
    let mut result = String::with_capacity(36);
    for c in template.chars() {
        match c {
            'x' => {
                // Generate random hex digit
                let random = js_sys::Math::random() * 16.0;
                let index = random as usize;
                result.push(hex_chars[index]);
            },
            'y' => {
                // For the y position, use 8, 9, a, or b (variant 1)
                let random = js_sys::Math::random() * 4.0;
                let index = 8 + random as usize;
                result.push(hex_chars[index]);
            },
            _ => result.push(c),
        }
    }
    
    result
}

// Helper function to get player ID from localStorage
fn get_player_id() -> String {
    let stored = get_item("player_id");
    if !stored.is_null() {
        stored.as_string().unwrap_or_default()
    } else {
        // Generate a new player ID and store it
        let new_id = generate_player_id();
        set_item("player_id", &new_id);
        new_id
    }
}

#[component]
pub fn DataButton() -> impl IntoView {
    // Create a signal to track whether we're showing the button or panel
    let (show_panel, set_show_panel) = signal(false);
    
    // Get the player ID when the component initializes
    let player_id = store_value(get_player_id());
    
    // Click handler for the button to show the panel
    let show_panel_click = move |_| {
        set_show_panel.set(true);
    };
    
    // Click handler for the close button to hide the panel
    let hide_panel_click = move |_| {
        set_show_panel.set(false);
    };
    
    view! {
        <div class="mt-6">
            {move || {
                if show_panel.get() {
                    // Panel view
                    view! {
                        <div 
                            class="bg-white rounded-lg shadow-lg p-4 border border-gray-200"
                            data-test-id="data-panel"
                        >
                            <div class="flex justify-between items-center mb-4">
                                <h2 
                                    data-test-id="data-header"
                                    class="text-xl font-semibold text-indigo-700"
                                >
                                    "Locally Stored Data"
                                </h2>
                                <button
                                    data-test-id="data-close-button"
                                    class="bg-gray-200 hover:bg-gray-300 text-gray-800 p-1 rounded-lg"
                                    on:click={hide_panel_click}
                                >
                                    "×"
                                </button>
                            </div>
                            <div 
                                data-test-id="data-content"
                                class="p-4 bg-indigo-50 rounded border border-indigo-100 text-indigo-900 font-medium"
                            >
                                <p>"Your locally stored data would appear here."</p>
                                <p 
                                    data-test-id="player-id"
                                    class="mt-2 pt-2 border-t border-indigo-200 text-indigo-700"
                                >
                                    {"Player ID: "}{player_id.get_value()}
                                </p>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    // Button view
                    view! {
                        <button
                            data-test-id="data-button"
                            class="bg-indigo-500 hover:bg-indigo-600 text-white font-medium py-2 px-4 rounded-lg transition-colors"
                            on:click={show_panel_click}
                        >
                            "Locally Stored Data"
                        </button>
                    }.into_any()
                }
            }}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;
    use wasm_bindgen_test::*;
    use crate::test_utils::test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    async fn test_data_button_exists() {
        // Mount the DataButton component to the body
        mount_to_body(|| view! { <DataButton /> });
        
        // Use the helper function to get the element by test ID
        let data_button = get_by_test_id("data-button");
        
        // Verify the button exists and has the correct text
        assert!(data_button.is_object(), "Data button should exist");
        assert_eq!(data_button.text_content().unwrap(), "Locally Stored Data", 
                "Button should have the text 'Locally Stored Data'");
    }

    #[wasm_bindgen_test]
    async fn test_data_button_shows_panel_when_clicked() {
        // Mount the DataButton component to the body
        mount_to_body(|| view! { <DataButton /> });
        
        // Get the data button
        let data_button = get_by_test_id("data-button");
        
        // Click the button and wait for reactivity
        click_and_wait(&data_button, 100).await;
        
        // After clicking, the button should be replaced with a panel
        // Check for the header element
        let data_header = get_by_test_id("data-header");
        assert_eq!(
            data_header.text_content().unwrap(), 
            "Locally Stored Data", 
            "Panel should have 'Locally Stored Data' as header text"
        );
        
        // Check for the existence of a close button
        let close_button = get_by_test_id("data-close-button");
        assert!(close_button.is_object(), "Close button should exist in the panel");
        
        // Test for content visibility
        let data_content = get_by_test_id("data-content");
        assert!(data_content.is_object(), "Data content should exist");
        
        // Check that player ID element exists
        let player_id_element = get_by_test_id("player-id");
        assert!(player_id_element.is_object(), "Player ID element should exist in the data panel");
    }
    
    #[wasm_bindgen_test]
    async fn test_data_panel_shows_player_id() {
        // Mount the DataButton component to the body
        mount_to_body(|| view! { <DataButton /> });
        
        // Get the data button
        let data_button = get_by_test_id("data-button");
        
        // Click the button to show the panel and wait for reactivity
        click_and_wait(&data_button, 100).await;
        
        // After clicking, the panel should be shown with player ID
        // Get the player ID element 
        let player_id_element = get_by_test_id("player-id");
        
        // Assert that the player ID element exists and contains a value
        assert!(player_id_element.is_object(), "Player ID element should exist in the data panel");
        
        // Check that the player ID text content is not empty
        let player_id_text = player_id_element.text_content().unwrap();
        assert!(!player_id_text.is_empty(), "Player ID should not be empty");
        
        // Check that the player ID text starts with "Player ID: "
        assert!(player_id_text.starts_with("Player ID: "), 
            "Player ID should be formatted as 'Player ID: XXX'");
    }
}