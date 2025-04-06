use leptos::*;
use leptos::prelude::*;
use log::{error, info};
use crate::utils::{get_dark_mode_preference, save_dark_mode_preference};

// Define our theme context
#[derive(Copy, Clone)]
pub struct ThemeState {
    pub dark_mode: bool,
    pub toggle_theme: Action<(), ()>,
}

pub fn provide_theme() -> ThemeState {
    // Create a signal to track dark mode state, initialized from localStorage
    let (dark_mode, set_dark_mode) = create_signal(get_dark_mode_preference());
    
    // Message for user feedback
    let (storage_message, set_storage_message) = create_signal(Option::<String>::None);
    
    // Create an action to toggle the theme
    let toggle_theme = create_action(move |_: &()| {
        set_dark_mode.update(|dark| {
            *dark = !*dark;
            
            // Handle the result of saving the preference
            match save_dark_mode_preference(*dark) {
                Ok(_) => {
                    // Clear any previous error messages
                    set_storage_message.set(None);
                },
                Err(err) => {
                    // Display the error message to the user
                    set_storage_message.set(Some(format!("Failed to save preference: {:?}", err)));
                    
                    // Log the error for debugging
                    error!("Failed to save dark mode preference: {:?}", err);
                }
            };
        });
        
        // Return unit for the action
        async {}
    });
    
    // Create the ThemeState
    let theme_state = ThemeState {
        dark_mode: dark_mode.get(),
        toggle_theme,
    };
    
    // Provide the theme state to the context
    provide_context(theme_state);
    
    // Return the theme state
    theme_state
}

// Component wrappers for common theme patterns
pub fn use_container_class() -> impl Fn() -> String {
    let theme_state = use_context::<ThemeState>().expect("ThemeState should be provided");
    let dark_mode = MaybeSignal::derive(move || theme_state.dark_mode);
    
    move || {
        if dark_mode.get() {
            "min-h-screen bg-gradient-to-b from-gray-900 to-gray-800 text-white flex flex-col items-center justify-center p-4 dark".to_string()
        } else {
            "min-h-screen bg-gradient-to-b from-blue-50 to-indigo-100 flex flex-col items-center justify-center p-4".to_string()
        }
    }
}

pub fn use_card_class() -> impl Fn() -> String {
    let theme_state = use_context::<ThemeState>().expect("ThemeState should be provided");
    let dark_mode = MaybeSignal::derive(move || theme_state.dark_mode);
    
    move || {
        if dark_mode.get() {
            "bg-gray-800 rounded-xl shadow-lg p-8 max-w-md w-full".to_string()
        } else {
            "bg-white rounded-xl shadow-lg p-8 max-w-md w-full".to_string()
        }
    }
}

pub fn use_dark_mode_toggle_button_class() -> impl Fn() -> String {
    let theme_state = use_context::<ThemeState>().expect("ThemeState should be provided");
    let dark_mode = MaybeSignal::derive(move || theme_state.dark_mode);
    
    move || {
        if dark_mode.get() {
            "ml-4 px-3 py-1 bg-purple-600 hover:bg-purple-700 text-white rounded text-sm transition-colors".to_string()
        } else {
            "ml-4 px-3 py-1 bg-indigo-500 hover:bg-indigo-600 text-white rounded text-sm transition-colors".to_string()
        }
    }
}

pub fn use_error_message_class() -> impl Fn() -> String {
    let theme_state = use_context::<ThemeState>().expect("ThemeState should be provided");
    let dark_mode = MaybeSignal::derive(move || theme_state.dark_mode);
    
    move || {
        if dark_mode.get() {
            "mt-2 p-2 bg-red-900 text-red-300 rounded-md border border-red-800".to_string()
        } else {
            "mt-2 p-2 bg-red-100 text-red-700 rounded-md border border-red-200".to_string()
        }
    }
}

pub fn use_header_class() -> impl Fn() -> String {
    let theme_state = use_context::<ThemeState>().expect("ThemeState should be provided");
    let dark_mode = MaybeSignal::derive(move || theme_state.dark_mode);
    
    move || {
        if dark_mode.get() {
            "text-3xl font-bold text-center text-purple-400 mb-6".to_string()
        } else {
            "text-3xl font-bold text-center text-indigo-600 mb-6".to_string()
        }
    }
}

pub fn use_paragraph_class() -> impl Fn() -> String {
    let theme_state = use_context::<ThemeState>().expect("ThemeState should be provided");
    let dark_mode = MaybeSignal::derive(move || theme_state.dark_mode);
    
    move || {
        if dark_mode.get() {
            "text-gray-300 text-center mb-6".to_string()
        } else {
            "text-gray-600 text-center mb-6".to_string()
        }
    }
}

pub fn use_button_class() -> impl Fn() -> String {
    let theme_state = use_context::<ThemeState>().expect("ThemeState should be provided");
    let dark_mode = MaybeSignal::derive(move || theme_state.dark_mode);
    
    move || {
        if dark_mode.get() {
            "bg-purple-600 hover:bg-purple-700 text-white font-medium py-2 px-4 rounded-lg transition-colors mr-2".to_string()
        } else {
            "bg-indigo-500 hover:bg-indigo-600 text-white font-medium py-2 px-4 rounded-lg transition-colors mr-2".to_string()
        }
    }
}

pub fn use_toggle_class() -> impl Fn() -> String {
    let theme_state = use_context::<ThemeState>().expect("ThemeState should be provided");
    let dark_mode = MaybeSignal::derive(move || theme_state.dark_mode);
    
    move || {
        if dark_mode.get() {
            "bg-amber-700 hover:bg-amber-800 text-gray-100 font-medium py-2 px-4 rounded-lg transition-colors flex items-center".to_string()
        } else {
            "bg-gray-700 hover:bg-gray-800 text-white font-medium py-2 px-4 rounded-lg transition-colors flex items-center".to_string()
        }
    }
}

pub fn use_toggle_text() -> impl Fn() -> &'static str {
    let theme_state = use_context::<ThemeState>().expect("ThemeState should be provided");
    let dark_mode = MaybeSignal::derive(move || theme_state.dark_mode);
    
    move || {
        if dark_mode.get() {
            "🌙 Dark"
        } else {
            "☀️ Light"
        }
    }
}

pub fn use_data_panel_class() -> impl Fn() -> String {
    let theme_state = use_context::<ThemeState>().expect("ThemeState should be provided");
    let dark_mode = MaybeSignal::derive(move || theme_state.dark_mode);
    
    move || {
        if dark_mode.get() {
            "bg-gray-800 rounded-lg shadow-lg p-4 border border-gray-700".to_string()
        } else {
            "bg-white rounded-lg shadow-lg p-4 border border-gray-200".to_string()
        }
    }
}

pub fn use_data_header_class() -> impl Fn() -> String {
    let theme_state = use_context::<ThemeState>().expect("ThemeState should be provided");
    let dark_mode = MaybeSignal::derive(move || theme_state.dark_mode);
    
    move || {
        if dark_mode.get() {
            "text-xl font-semibold text-purple-400".to_string()
        } else {
            "text-xl font-semibold text-indigo-700".to_string()
        }
    }
}

pub fn use_data_content_class() -> impl Fn() -> String {
    let theme_state = use_context::<ThemeState>().expect("ThemeState should be provided");
    let dark_mode = MaybeSignal::derive(move || theme_state.dark_mode);
    
    move || {
        if dark_mode.get() {
            "p-4 bg-gray-700 rounded border border-gray-600 text-gray-200 font-medium".to_string()
        } else {
            "p-4 bg-indigo-50 rounded border border-indigo-100 text-indigo-900 font-medium".to_string()
        }
    }
}

pub fn use_data_close_button_class() -> impl Fn() -> String {
    let theme_state = use_context::<ThemeState>().expect("ThemeState should be provided");
    let dark_mode = MaybeSignal::derive(move || theme_state.dark_mode);
    
    move || {
        if dark_mode.get() {
            "bg-gray-600 hover:bg-gray-500 text-gray-200 p-1 rounded-lg".to_string()
        } else {
            "bg-gray-200 hover:bg-gray-300 text-gray-800 p-1 rounded-lg".to_string()
        }
    }
}

pub fn use_player_id_class() -> impl Fn() -> String {
    let theme_state = use_context::<ThemeState>().expect("ThemeState should be provided");
    let dark_mode = MaybeSignal::derive(move || theme_state.dark_mode);
    
    move || {
        if dark_mode.get() {
            "mt-2 pt-2 border-t border-gray-600 text-purple-400".to_string()
        } else {
            "mt-2 pt-2 border-t border-indigo-200 text-indigo-700".to_string()
        }
    }
}

#[component]
pub fn ThemeProvider(
    /// Optional children to render inside the theme provider
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    // Provide theme context to the app
    let _theme_state = provide_theme();
    
    // Return children with the provided theme
    view! {
        {children.map(|children| children())}
    }
}

// Helper to get the theme context
pub fn use_theme() -> ThemeState {
    use_context::<ThemeState>().expect("ThemeState should be provided")
}