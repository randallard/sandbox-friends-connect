use leptos::*;
use leptos::prelude::*;
use crate::data::DataButton;
use crate::theme::{ThemeProvider, use_container_class, use_card_class, use_header_class, 
                  use_paragraph_class, use_button_class, use_toggle_class, use_toggle_text, use_theme};
use log::{error, info}; // Import log macros

#[component]
pub fn App() -> impl IntoView {
    // Message for user feedback
    let (storage_message, set_storage_message) = create_signal(Option::<String>::None);
    
    // Error message class
    let error_class = "mt-4 p-2 bg-red-100 text-red-700 rounded-md text-sm";
    
    view! {
        <ThemeProvider>
            <AppContent storage_message={storage_message} set_storage_message={set_storage_message} error_class={error_class} />
        </ThemeProvider>
    }
}

#[component]
fn AppContent(
    storage_message: ReadSignal<Option<String>>,
    set_storage_message: WriteSignal<Option<String>>,
    error_class: &'static str,
) -> impl IntoView {
    // Get theme helpers
    let container_class = use_container_class();
    let card_class = use_card_class();
    let header_class = use_header_class();
    let paragraph_class = use_paragraph_class();
    let button_class = use_button_class();
    let toggle_class = use_toggle_class();
    let toggle_text = use_toggle_text();
    
    // Get theme context for the toggle action
    let theme = use_theme();
    
    // Toggle function for the dark mode using the action from theme context
    let toggle_dark_mode = move |_| {
        theme.toggle_theme.dispatch(());
    };
    
    view! {
        <div
            data-test-id="app-container"
            class={container_class}
        >
            <div class={card_class}>
                <h1 data-test-id="hello-header" class={header_class}>"Hello Leptos"</h1>
                <p class={paragraph_class}>"Welcome to your Tailwind-styled Leptos app!"</p>
                <div class="flex justify-center space-x-4">
                    <button class={button_class}>
                        "Get Started"
                    </button>
                    <button
                        data-test-id="dark-mode-toggle"
                        class={toggle_class}
                        on:click={toggle_dark_mode}
                    >
                        {toggle_text}
                    </button>
                </div>
                
                // Show storage error message if any
                {move || {
                    storage_message.get().map(|msg| {
                        view! {
                            <div data-test-id="storage-error" class={error_class}>
                                {msg}
                            </div>
                        }
                    })
                }}
            </div>

            <DataButton />
        </div>
    }
}

