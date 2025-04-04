use leptos::*;
use leptos::prelude::*;
use crate::utils::{get_player_id, get_dark_mode_preference, save_dark_mode_preference, StorageError};
use log::{error, warn, info};
use wasm_bindgen::prelude::*;

// JavaScript console logging helper
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Create a global ID for testing
#[derive(Clone)]
struct PlayerIdState(String);

// Create a global dark mode state for testing
#[derive(Clone)]
struct DarkModeState(bool);

#[component]
pub fn DataButton() -> impl IntoView {
    // Create a signal to track whether we're showing the button or panel
    let (show_panel, set_show_panel) = create_signal(false);
    
    // Create a signal for potential storage errors
    let (storage_error, set_storage_error) = create_signal(Option::<String>::None);
    
    // Get the player ID when the component initializes
    let id = get_player_id();
    
    // Get the dark mode preference
    let dark_mode = get_dark_mode_preference();
    
    // Store player ID and dark mode in global state for testing
    provide_context(PlayerIdState(id.clone()));
    provide_context(DarkModeState(dark_mode));
    
    if id.is_empty() {
        let err_msg = "Failed to get or generate player ID".to_string();
        error!("{}", err_msg);
        set_storage_error.set(Some(err_msg));
    } else {
        // Log the player ID to the console for debugging and testing
        let log_msg = format!("PLAYER_ID_DATA: {}", id);
        log(&log_msg);
        info!("{}", log_msg);
    }
    
    // Create signals for the player ID and dark mode to use in reactive contexts
    let player_id = create_rw_signal(id);
    let (dark_mode_preference, set_dark_mode_preference) = create_signal(dark_mode);
    
    // Click handler for the button to show the panel
    let show_panel_click = move |_| {
        set_show_panel.set(true);
        
        // Log the player ID again when the panel is shown
        let current_id = player_id.get();
        if !current_id.is_empty() {
            let log_msg = format!("PLAYER_ID_PANEL_OPENED: {}", current_id);
            log(&log_msg);
            info!("{}", log_msg);
        }
    };
    
    // Click handler for the close button to hide the panel
    let hide_panel_click = move |_| {
        set_show_panel.set(false);
    };
    
    // Click handler for toggling dark mode
    let toggle_dark_mode = move |_| {
        let new_preference = !dark_mode_preference.get();
        set_dark_mode_preference.set(new_preference);
        
        // Save the new preference
        if let Err(err) = save_dark_mode_preference(new_preference) {
            let err_msg = format!("Failed to save dark mode preference: {:?}", err);
            error!("{}", err_msg);
            set_storage_error.set(Some(err_msg));
        } else {
            // Log the dark mode change
            let log_msg = format!("DARK_MODE_CHANGED: {}", new_preference);
            log(&log_msg);
            info!("{}", log_msg);
        }
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
                                
                                {move || {
                                    if let Some(error) = storage_error.get() {
                                        view! {
                                            <p 
                                                data-test-id="storage-error"
                                                class="mt-2 p-2 bg-red-100 text-red-700 rounded-md"
                                            >
                                                {"Error: "}{error}
                                            </p>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div>
                                                <p 
                                                    data-test-id="player-id"
                                                    class="mt-2 pt-2 border-t border-indigo-200 text-indigo-700"
                                                >
                                                    {"Player ID: "}{player_id.get()}
                                                </p>
                                                <div 
                                                    data-test-id="dark-mode-setting"
                                                    class="mt-2 pt-2 border-t border-indigo-200 text-indigo-700 flex justify-between items-center"
                                                >
                                                    <span>{"Dark Mode: "}{if dark_mode_preference.get() { "Enabled" } else { "Disabled" }}</span>
                                                    <button
                                                        data-test-id="dark-mode-toggle"
                                                        class="ml-4 px-3 py-1 bg-indigo-500 hover:bg-indigo-600 text-white rounded text-sm"
                                                        on:click={toggle_dark_mode}
                                                    >
                                                        {if dark_mode_preference.get() { "Disable" } else { "Enable" }}
                                                    </button>
                                                </div>
                                            </div>
                                        }.into_any()
                                    }
                                }}
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

// Helper function to get the player ID in tests
pub fn get_test_player_id() -> Option<String> {
    use_context::<PlayerIdState>().map(|state| state.0)
}

// Helper function to get the dark mode preference in tests
pub fn get_test_dark_mode() -> Option<bool> {
    use_context::<DarkModeState>().map(|state| state.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;
    use wasm_bindgen_test::*;
    use crate::test_utils::test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    // Simplified console spy
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console, js_name = log)]
        fn console_log(s: &str);
    }
    
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

        // Check that dark mode setting element exists
        let dark_mode_element = get_by_test_id("dark-mode-setting");
        assert!(dark_mode_element.is_object(), "Dark mode setting element should exist in the data panel");
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
            
        // Print something to console for debugging
        console_log("Test completed successfully!");
    }
    
    // New test for storage error display
    #[wasm_bindgen_test]
    async fn test_storage_error_display() {
        // This test would check that storage errors are displayed properly
        // Note: For a full test we would need to mock localStorage failures
        
        // Since mocking is complex, we're just checking the component structure
        mount_to_body(|| view! { <DataButton /> });
        
        // Get the data button and click it
        let data_button = get_by_test_id("data-button");
        click_and_wait(&data_button, 100).await;
        
        // Check that either player-id or storage-error exists
        let document = web_sys::window().unwrap().document().unwrap();
        let has_player_id = document.query_selector("[data-test-id='player-id']").unwrap().is_some();
        let has_error = document.query_selector("[data-test-id='storage-error']").unwrap().is_some();
        
        // Either the player ID or an error message should exist
        assert!(has_player_id || has_error, 
            "Either player ID or storage error element should exist in the panel");
    }
}