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

/// Import application data from a JSON string
/// Returns a Result with either a success message or an error
pub fn import_data(json_data: &str) -> Result<String, String> {
    // Parse the JSON string
    let parsed_data: Result<ExportedData, _> = serde_json::from_str(json_data);
    
    match parsed_data {
        Ok(data) => {
            // Validate version (in a real implementation, you might check compatibility)
            if data.version.is_empty() {
                return Err("Invalid data format: missing version".to_string());
            }
            
            // Extract the actual app data
            let app_data = data.data;
            
            // Store player_id
            match localStorage::set_storage_item("player_id", &app_data.player_id) {
                Ok(_) => {},
                Err(err) => {
                    error!("Failed to store player_id during import: {:?}", err);
                    return Err(format!("Storage error: {:?}", err));
                }
            }
            
            // Store dark_mode preference
            let dark_mode_value = if app_data.dark_mode { "true" } else { "false" };
            match localStorage::set_storage_item("dark_mode", dark_mode_value) {
                Ok(_) => {},
                Err(err) => {
                    error!("Failed to store dark_mode during import: {:?}", err);
                    return Err(format!("Storage error: {:?}", err));
                }
            }
            
            // Log successful import
            let log_msg = format!("DATA_IMPORT: Successfully imported data with player_id: {}", app_data.player_id);
            info!("{}", log_msg);
            log(&log_msg);
            
            Ok("Data imported successfully".to_string())
        },
        Err(err) => {
            // Handle parsing error
            let error_msg = format!("Failed to parse imported data: {:?}", err);
            error!("{}", &error_msg);
            Err(error_msg)
        }
    }
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

// Updated DataButton component with load functionality

#[component]
pub fn DataButton() -> impl IntoView {
    // Create a signal to track whether we're showing the button or panel
    let (show_panel, set_show_panel) = create_signal(false);
    let (storage_error, set_storage_error) = create_signal(Option::<String>::None);
    let (export_success, set_export_success) = create_signal(Option::<String>::None);
    let (load_success, set_load_success) = create_signal(Option::<String>::None);

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
        
        // Clear any success/error messages when panel is closed
        set_export_success.set(None);
        set_load_success.set(None);
        set_storage_error.set(None);
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
        set_load_success.set(None);
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
    

// Load button click handler
let load_button_click = move |_| {
    // Clear any previous messages
    set_export_success.set(None);
    set_load_success.set(None);
    set_storage_error.set(None);
    
    // Create a file input element
    let window = web_sys::window().expect("No window found");
    let document = window.document().expect("No document found");
    
    // Create a file input element
    let file_input = document
        .create_element("input")
        .expect("Failed to create input element");
    
    // Set attributes for the file input
    file_input
        .set_attribute("type", "file")
        .expect("Failed to set input type");
    file_input
        .set_attribute("accept", ".json")
        .expect("Failed to set accept attribute");
    file_input
        .set_attribute("style", "display: none;")
        .expect("Failed to set style attribute");
    
    // Add the input to the document body
    let body = document.body().expect("No body found");
    body.append_child(&file_input)
        .expect("Failed to append file input");
    
    // Create a reference to file_input that will be shared by the closure
    let file_input_ref = file_input.clone();
    
    // Use FnMut instead of FnOnce
    let onchange_callback = Closure::wrap(Box::new(move |_event: web_sys::Event| {
        // Create a separate clone here to avoid moving file_input_ref
        let input_elem = file_input_ref.clone();
        let file_input = input_elem
            .dyn_into::<web_sys::HtmlInputElement>()
            .expect("Failed to cast to HtmlInputElement");
        
        // Get the selected file - files is a property, not a method
        let files = file_input.files();
        if let Some(files) = files {
            if files.length() > 0 {
                if let Some(file_js) = files.get(0) {
                    let file = file_js.dyn_into::<web_sys::File>().expect("Failed to cast to File");

                    // Create a FileReader to read the file
                    let reader = web_sys::FileReader::new().expect("Failed to create FileReader");
                    let reader_clone = reader.clone();
                    
                    // Set up onload handler for the FileReader
                    let onload_closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                        // Get the file content as text
                        if let Ok(result) = reader_clone.result() {
                            if let Some(text) = result.as_string() {
                                match import_data(&text) {
                                    Ok(success_msg) => {
                                        // Update the UI with success message
                                        set_load_success.set(Some(success_msg));
                                        
                                        // Log successful import
                                        let log_msg = "DATA_IMPORT: File import successful";
                                        info!("{}", log_msg);
                                        log(log_msg);
                                        
                                        // Refresh the player ID display
                                        if let Ok(Some(id)) = localStorage::get_storage_item("player_id") {
                                            player_id.set(id);
                                        }
                                        
                                        // Refresh dark mode preference display
                                        if let Ok(Some(mode)) = localStorage::get_storage_item("dark_mode") {
                                            let is_dark = mode == "true";
                                            // Only toggle if different from current state to avoid double toggle
                                            if dark_mode.get() != is_dark {
                                                theme.toggle_theme.dispatch(());
                                            }
                                        }
                                    },
                                    Err(err) => {
                                        // Clone or copy the error string before using it
                                        let error_string = err.clone(); // If err is a String or has Clone implemented
                                        
                                        // Update the UI with error message
                                        set_storage_error.set(Some(error_string));
                                        
                                        // Log import error using the original err
                                        let error_msg = format!("DATA_IMPORT_ERROR: {}", err);
                                        error!("{}", &error_msg);
                                        log(&error_msg);
                                    }
                                }
                                } else {
                                // Handle case where result is not a string
                                let error_msg = "Failed to read file as text".to_string();
                                error!("{}", &error_msg);
                                set_storage_error.set(Some(error_msg));
                            }
                        } else {
                            // Handle case where result() returns an error
                            let error_msg = "Error getting result from FileReader".to_string();
                            error!("{}", &error_msg);
                            set_storage_error.set(Some(error_msg));
                        }
                    }) as Box<dyn FnMut(_)>);
                    
                    // Set the onload handler
                    reader.set_onload(Some(onload_closure.as_ref().unchecked_ref()));
                    onload_closure.forget(); // Prevent closure from being dropped
                    
                    // Set up error handler for the FileReader
                    let reader_error_clone = reader.clone();
                    let onerror_closure = Closure::wrap(Box::new(move |_: web_sys::Event| {
                        let error_msg = "Error reading file".to_string();
                        error!("{}", &error_msg);
                        set_storage_error.set(Some(error_msg));
                    }) as Box<dyn FnMut(_)>);
                    
                    // Set the onerror handler
                    reader.set_onerror(Some(onerror_closure.as_ref().unchecked_ref()));
                    onerror_closure.forget(); // Prevent closure from being dropped
                    
                    // Start reading the file as text
                    if let Err(err) = reader.read_as_text(&file) {
                        let error_msg = format!("Failed to read file: {:?}", err);
                        error!("{}", &error_msg);
                        set_storage_error.set(Some(error_msg));
                    }
                } else {
                    // File is None
                    let error_msg = "Could not access selected file".to_string();
                    error!("{}", &error_msg);
                    set_storage_error.set(Some(error_msg));
                }
            } else {
                // No file selected
                let error_msg = "No file selected".to_string();
                error!("{}", &error_msg);
                set_storage_error.set(Some(error_msg));
            }
        } else {
            // No files property
            let error_msg = "Failed to access file input files".to_string();
            error!("{}", &error_msg);
            set_storage_error.set(Some(error_msg));
        }
        
        // Use another clone of file_input_ref to avoid moving it
        let document_clone = window.document().expect("No document found");
        if let Some(body) = document_clone.body() {
            let input_to_remove = file_input_ref.clone();
            let _ = body.remove_child(&input_to_remove);
        }
    }) as Box<dyn FnMut(_)>);
    
    // Set the onchange handler
    file_input
        .add_event_listener_with_callback("change", onchange_callback.as_ref().unchecked_ref())
        .expect("Failed to add event listener");
    onchange_callback.forget(); // Prevent closure from being dropped
    
    // Trigger click on the file input to open file dialog
    let file_input_html = file_input
        .dyn_into::<web_sys::HtmlElement>()
        .expect("Failed to cast to HtmlElement");
    file_input_html.click();
    
    // Log load action
    let log_msg = "DATA_LOAD: File picker dialog opened";
    info!("{}", log_msg);
    log(log_msg);
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
                                                
                                                <div class="mt-4 flex space-x-2">
                                                    <button
                                                        data-test-id="export-data-button"
                                                        class={use_button_class}
                                                        on:click={export_button_click}
                                                    >
                                                        "Export Data"
                                                    </button>
                                                    
                                                    <button
                                                        data-test-id="load-data-button"
                                                        class={use_button_class}
                                                        on:click={load_button_click}
                                                    >
                                                        "Load Data"
                                                    </button>
                                                </div>
                                                
                                                <div class="mt-2">
                                                    {move || {
                                                        if let Some(success) = export_success.get() {
                                                            view! {
                                                                <p 
                                                                    data-test-id="export-success-message"
                                                                    class="text-green-600 dark:text-green-400"
                                                                >
                                                                    {success}
                                                                </p>
                                                            }.into_any()
                                                        } else if let Some(success) = load_success.get() {
                                                            view! {
                                                                <p 
                                                                    data-test-id="load-success-message"
                                                                    class="text-green-600 dark:text-green-400"
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