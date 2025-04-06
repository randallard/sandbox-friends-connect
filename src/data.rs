use leptos::*;
use leptos::prelude::*;
use crate::utils::get_player_id;
use crate::theme::{
    use_theme, 
    use_button_class, 
    use_data_panel_class, 
    use_data_header_class, 
    use_data_close_button_class, 
    use_data_content_class, 
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

    // Get the player ID when the component initializes
    let id = get_player_id();
    
    // Log the player ID to the console for debugging
    if !id.is_empty() {
        let log_msg = format!("PLAYER_ID_DATA: {}", id);
        log(&log_msg);
        info!("{}", log_msg);
    } else {
        error!("Failed to get or generate player ID");
    }
    
    let player_id = create_rw_signal(id);

    // Get the theme context
    let theme = use_theme();
    let dark_mode = theme.dark_mode;

    let button_class = use_button_class.clone();
    let data_panel_class = use_data_panel_class.clone();
    let data_header_class = use_data_header_class.clone();
    let data_close_button_class = use_data_close_button_class.clone();
    let data_content_class = use_data_content_class.clone();
    let player_id_class = use_player_id_class.clone();

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

    let show_panel_for_view = show_panel.clone();
    let show_panel_click_for_view = show_panel_click.clone();
    let hide_panel_click_for_view = hide_panel_click.clone();
    let player_id_for_view = player_id.clone();
    let dark_mode_for_view = dark_mode.clone();

    view! {
        <div class="mt-6">
            {move || {
                if show_panel_for_view.get() {
                    // Panel view
                    view! {
                        <div class={data_panel_class}>
                            <div class="flex justify-between items-center mb-4">
                                <h2 class={data_header_class}>
                                    "Locally Stored Data"
                                </h2>
                                <button
                                    class={data_close_button_class}
                                    on:click={hide_panel_click_for_view}
                                >
                                    "×"
                                </button>
                            </div>
                            <div>
                                <p>"Your locally stored data:"</p>
                                <p>{"Player ID: "}{player_id_for_view.get()}</p>
                                <p>{"Dark Mode: "}{if dark_mode_for_view { "Enabled" } else { "Disabled" }}</p>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    // Button view
                    view! {
                        <button
                            class={button_class}
                            on:click={show_panel_click_for_view}
                        >
                            "Locally Stored Data"
                        </button>
                    }.into_any()
                }
            }}
        </div>
    }
}