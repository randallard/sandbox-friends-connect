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

// JavaScript console logging helper
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[component]
pub fn DataButton() -> impl IntoView {
    // Create a signal to track whether we're showing the button or panel
    let (show_panel, set_show_panel) = create_signal(false);
    let (storage_error, set_storage_error) = create_signal(Option::<String>::None);

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
    
    let player_id = create_rw_signal(id);
    let dark_mode_preference = create_rw_signal(dark_mode);

    // Get the theme context
    let theme = use_theme();
    let dark_mode = theme.dark_mode;

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
        let new_mode = if dark_mode { "Disabled" } else { "Enabled" };
        let log_msg = format!("DARK_MODE_CHANGED: {}", !dark_mode);
        log(&log_msg);
        info!("{}", log_msg);
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
                                                    {"Dark Mode: "}{if dark_mode { "Enabled" } else { "Disabled" }}
                                                    <button
                                                        data-test-id="dark-mode-toggle"
                                                        class={use_dark_mode_toggle_button_class}
                                                        on:click={toggle_dark_mode}
                                                    >
                                                        {if dark_mode { "Disable" } else { "Enable" }}
                                                    </button>
                                                </p>
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