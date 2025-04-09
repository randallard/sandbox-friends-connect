use leptos::ev::play;
use leptos::*;
use leptos::prelude::*;
use crate::utils::get_player_id;
use crate::theme::{
    use_theme,
    use_dark_mode_toggle_button_class, 
    use_button_class, 
    use_data_panel_class, 
    use_data_header_class, 
    use_data_close_button_class, 
    use_data_content_class,
    use_error_message_class, 
    use_player_id_class
};
use log::{error, info};
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use web_sys::{Blob, BlobPropertyBag, Url, HtmlAnchorElement, Document};
use js_sys;
use crate::utils::localStorage;

// Data export type
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExportedData {
    pub version: String,
    pub timestamp: String,
    pub data: ExportedAppData,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ExportedAppData {
    pub player_id: String,
    pub dark_mode: bool,
}

// JavaScript console logging helper
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// Creates a download for the user with the given content and filename
pub fn trigger_download(content: &str, filename: &str) -> Result<(), JsValue> {
    // Create a Blob from the content string
    let mut blob_properties = BlobPropertyBag::new();
    blob_properties.type_("application/json");
    
    let blob_parts = js_sys::Array::new();
    blob_parts.push(&JsValue::from_str(content));
    
    let blob = Blob::new_with_str_sequence_and_options(
        &blob_parts,
        &blob_properties,
    )?;
    
    // Create a URL for the blob
    let url = Url::create_object_url_with_blob(&blob)?;
    
    // Create and click an anchor element to trigger the download
    let window = web_sys::window().ok_or_else(|| JsValue::from_str("No window found"))?;
    let document = window.document().ok_or_else(|| JsValue::from_str("No document found"))?;
    let a = document.create_element("a")?
        .dyn_into::<HtmlAnchorElement>()?;
    
    a.set_href(&url);
    a.set_download(filename);
    a.set_attribute("style", "display: none;")?;
    
    let body = document.body().ok_or_else(|| JsValue::from_str("No body found"))?;
    body.append_child(&a)?;
    a.click();
    body.remove_child(&a)?;
    
    // Release the URL object
    Url::revoke_object_url(&url)?;
    
    Ok(())
}

/// Export all application data to a JSON string for backup purposes
/// Returns a Result with either the JSON string or an error message
pub fn export_data() -> Result<String, String> {
    // Get player_id from storage
    let player_id = match localStorage::get_storage_item("player_id") {
        Ok(Some(id)) => id,
        Ok(None) => {
            // No ID exists in storage - throw an error
            error!("No player ID found in storage during export");
            return Err("Missing player ID required for export".to_string());
        },
        Err(err) => {
            // Error accessing storage
            error!("Failed to access player ID during export: {:?}", err);
            return Err(format!("Storage error: {:?}", err));
        }
    };

    // Get dark mode preference
    let dark_mode = match localStorage::get_storage_item("dark_mode") {
        Ok(Some(value)) => value == "true",
        _ => false // Default to light mode
    };
    
    // Create the export data structure
    let export_data = ExportedData {
        version: "0.1.0".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        data: ExportedAppData {
            player_id,
            dark_mode,
        },
    };
    
    // Serialize to JSON
    match serde_json::to_string(&export_data) {
        Ok(json_string) => {
            info!("Data successfully exported");
            Ok(json_string)
        },
        Err(err) => {
            error!("Failed to serialize export data: {:?}", err);
            Err(format!("Serialization error: {:?}", err))
        }
    }
}

#[component]
pub fn DataButton() -> impl IntoView {
    // Create a signal to track whether we're showing the button or panel
    let (show_panel, set_show_panel) = create_signal(false);
    let (storage_error, set_storage_error) = create_signal(Option::<String>::None);
    let (export_success, set_export_success) = create_signal(Option::<String>::None);

    // Get the player ID when the component initializes
    let id = get_player_id();
    
    // Log the player ID to the console for debugging
    if !id.is_empty() {
        let log_msg = format!("PLAYER_ID_DATA: {}", id);
        log(&log_msg);
        info!("{}", log_msg);
    } else {
        let err_msg = "Failed to get or generate player ID".to_string();
        error!("{}", err_msg);
        set_storage_error.set(Some(err_msg));
    }
    
    let theme = use_theme();
    let dark_mode = theme.dark_mode;
    let player_id = create_rw_signal(id);
    let dark_mode_preference = create_rw_signal(dark_mode);
    let dark_mode_signal = create_memo(move |_| theme.dark_mode);
    create_effect(move |_| {
        // Update our local reactive signal to match the global state
        let current_theme_value = dark_mode_signal.get();
        if dark_mode_preference.get() != current_theme_value {
            dark_mode_preference.set(current_theme_value);
        }
    });

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

    let toggle_dark_mode = move |_| {
        theme.toggle_theme.dispatch(());
        
        // Log the dark mode change
        let new_preference = !dark_mode.get(); // Predict new value
        let log_msg = format!("DARK_MODE_CHANGED: {}", new_preference);
        log(&log_msg);
        info!("{}", log_msg);
    };

    // Export button click handler
    let export_button_click = move |_| {
        // Clear any previous messages
        set_export_success.set(None);
        set_storage_error.set(None);
        
        // Get the data to export
        match export_data() {
            Ok(export_json) => {
                // Generate a filename with timestamp for uniqueness
                let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
                let filename = format!("game_data_export_{}.json", timestamp);
                
                // Trigger the download
                match trigger_download(&export_json, &filename) {
                    Ok(_) => {
                        // Set success message
                        set_export_success.set(Some("Data exported successfully".to_string()));
                        
                        // Log export action
                        let log_msg = format!("DATA_EXPORT: Export initiated: {}", filename);
                        info!("{}", log_msg);
                        log(&log_msg);
                    },
                    Err(err) => {
                        // Handle download error
                        let error_msg = format!("Failed to download data: {:?}", err);
                        error!("{}", &error_msg);
                        set_storage_error.set(Some(error_msg));
                    }
                }
            },
            Err(err) => {
                // Handle export error
                set_storage_error.set(Some(err));
            }
        }
    };

    view! {
        <div class="mt-6">
            {move || {
                if show_panel.get() {
                    // Panel view
                    view! {
                        <div class={use_data_panel_class}
                            data-test-id="data-panel">
                            <div class="flex justify-between items-center mb-4">
                                <h2 
                                    data-test-id="data-header"
                                    class={use_data_header_class}
                                >
                                    "Locally Stored Data"
                                </h2>
                                <button
                                    data-test-id="data-close-button"
                                    class={use_data_close_button_class}
                                    on:click={hide_panel_click}
                                >
                                    "×"
                                </button>
                            </div>
                            <div 
                                data-test-id="data-content"
                                class={use_data_content_class}
                            >
                                <p>"Your locally stored data:"</p>
                                {move || {
                                    if let Some(error) = storage_error.get() {
                                        view! {
                                            <p 
                                                data-test-id="storage-error"
                                                class={use_error_message_class}
                                            >
                                                {"Error: "}{error}
                                            </p>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div>
                                                <p 
                                                    data-test-id="player-id"
                                                    class={use_player_id_class}
                                                >
                                                    {"Player ID: "}{player_id.get()}
                                                </p>
                                                <p>
                                                    <span>{"Dark Mode: "}{if dark_mode.get() { "Enabled" } else { "Disabled" }}</span>
                                                    <button
                                                        data-test-id="dark-mode-toggle"
                                                        class={use_dark_mode_toggle_button_class}
                                                        on:click={toggle_dark_mode}
                                                    >
                                                        {if dark_mode.get() { "Disable" } else { "Enable" }}
                                                    </button>
                                                </p>
                                                
                                                <div class="mt-4">
                                                    <button
                                                        data-test-id="export-data-button"
                                                        class={use_button_class}
                                                        on:click={export_button_click}
                                                    >
                                                        "Export Data"
                                                    </button>
                                                    
                                                    {move || {
                                                        if let Some(success) = export_success.get() {
                                                            view! {
                                                                <p 
                                                                    data-test-id="export-success-message"
                                                                    class="mt-2 text-green-600 dark:text-green-400"
                                                                >
                                                                    {success}
                                                                </p>
                                                            }.into_any()
                                                        } else {
                                                            view! {}.into_any()
                                                        }
                                                    }}
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
                            class={use_button_class}
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