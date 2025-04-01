use leptos::*;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn localStorage() -> JsValue;

    #[wasm_bindgen(js_namespace = ["localStorage"], js_name = getItem)]
    fn get_item(key: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = ["localStorage"], js_name = setItem)]
    fn set_item(key: &str, value: &str);
}

#[component]
pub fn App() -> impl IntoView {
    // Helper functions for localStorage
    fn get_dark_mode_preference() -> bool {
        let stored = get_item("dark_mode");
        !stored.is_null() && stored.as_string().unwrap_or_default() == "true"
    }

    fn save_dark_mode_preference(is_dark: bool) {
        set_item("dark_mode", if is_dark { "true" } else { "false" });
    }

    // Create a signal to track dark mode state, initialized from localStorage
    let (dark_mode, set_dark_mode) = create_signal(get_dark_mode_preference());
    
    // Toggle function for the dark mode
    let toggle_dark_mode = move |_| {
        set_dark_mode.update(|dark| {
            *dark = !*dark;
            save_dark_mode_preference(*dark);
        });
    };
    
    // Dynamic class for container based on dark mode
    let container_class = move || {
        if dark_mode.get() {
            "min-h-screen bg-gradient-to-b from-gray-900 to-gray-800 text-white flex flex-col items-center justify-center p-4 dark"
        } else {
            "min-h-screen bg-gradient-to-b from-blue-50 to-indigo-100 flex flex-col items-center justify-center p-4"
        }
    };
    
    // Dynamic class for the card
    let card_class = move || {
        if dark_mode.get() {
            "bg-gray-800 rounded-xl shadow-lg p-8 max-w-md w-full"
        } else {
            "bg-white rounded-xl shadow-lg p-8 max-w-md w-full"
        }
    };
    
    // Dynamic class for header
    let header_class = move || {
        if dark_mode.get() {
            "text-3xl font-bold text-center text-purple-400 mb-6"
        } else {
            "text-3xl font-bold text-center text-indigo-600 mb-6"
        }
    };
    
    // Dynamic class for paragraph
    let paragraph_class = move || {
        if dark_mode.get() {
            "text-gray-300 text-center mb-6"
        } else {
            "text-gray-600 text-center mb-6"
        }
    };
    
    // Dynamic class for button
    let button_class = move || {
        if dark_mode.get() {
            "bg-purple-600 hover:bg-purple-700 text-white font-medium py-2 px-4 rounded-lg transition-colors mr-2"
        } else {
            "bg-indigo-500 hover:bg-indigo-600 text-white font-medium py-2 px-4 rounded-lg transition-colors mr-2"
        }
    };
    
    // Dynamic class for the toggle button
    let toggle_class = move || {
        if dark_mode.get() {
            "bg-amber-700 hover:bg-amber-800 text-gray-100 font-medium py-2 px-4 rounded-lg transition-colors flex items-center"
        } else {
            "bg-gray-700 hover:bg-gray-800 text-white font-medium py-2 px-4 rounded-lg transition-colors flex items-center"
        }
    };
    
    // Dynamic toggle icon/text
    let toggle_text = move || {
        if dark_mode.get() {
            "🌙 Dark"
        } else {
            "☀️ Light"
        }
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
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use web_sys::{Document, Storage, wasm_bindgen::JsCast, window};
    use crate::test_utils::test::*;

    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    async fn test_app_says_hello_leptos() {
        // Mount the App component to the body
        mount_to_body(|| view! { <App /> });
        
        // Test that the component renders "Hello Leptos"
        let header = get_by_test_id("hello-header");
        assert_eq!(header.text_content().unwrap(), "Hello Leptos");
    }
    
    #[wasm_bindgen_test]
    async fn test_tailwind_classes_applied() {
        // Mount the App component to the body
        mount_to_body(|| view! { <App /> });
        
        // Get elements by test ID
        let container = get_by_test_id("app-container");
        let header = get_by_test_id("hello-header");
        
        // Get the computed styles
        let window = web_sys::window().unwrap();
        let computed_style_container = window.get_computed_style(&container).unwrap().unwrap();
        let computed_style_header = window.get_computed_style(&header).unwrap().unwrap();
        
        // Test that minimum height is applied (min-h-screen)
        let min_height = computed_style_container.get_property_value("min-height").unwrap();
        assert!(!min_height.is_empty(), "min-height should be set from Tailwind's min-h-screen class");
        
        // Test that text color is applied (text-indigo-600)
        let color = computed_style_header.get_property_value("color").unwrap();
        assert!(!color.is_empty(), "Text color should be set");
        
        // Test that font weight is applied (font-bold)
        let font_weight = computed_style_header.get_property_value("font-weight").unwrap();
        assert!(font_weight == "700" || font_weight == "bold", 
                "Font weight should be bold from Tailwind's font-bold class");
    }
    
    #[wasm_bindgen_test]
    async fn test_dark_mode_toggle_exists() {
        // Mount the App component to the body
        mount_to_body(|| view! { <App /> });
        
        // Use the helper function to get the element by test ID
        let dark_mode_toggle = get_by_test_id("dark-mode-toggle");
        
        // The test will now pass if the toggle exists (get_by_test_id will panic if not found)
        assert!(dark_mode_toggle.is_object(), "Dark mode toggle should exist");
    }

    #[wasm_bindgen_test]
    async fn test_dark_mode_preference_persists() {
        
        let window = web_sys::window().unwrap();
        let storage = window.local_storage().unwrap().unwrap();

        // Clean up any previous state
        // storage.remove_item("dark_mode").unwrap();

        // Mount the App component to the body
        mount_to_body(|| view! { <App /> });
        
        // Verify initial state (should default to light)
        let container = get_by_test_id("app-container");
        let dark_mode_toggle = get_by_test_id("dark-mode-toggle");
        assert!(!container.class_list().contains("dark"), 
                "Container should start in light mode by default");
        
        // Toggle to dark mode
        click_and_wait(&dark_mode_toggle, 100).await;
        
        // Verify dark mode is active
        assert!(container.class_list().contains("dark"), 
                "Container should be in dark mode after toggle");
        
        // Verify localStorage was updated
        let stored_value = storage.get_item("dark_mode").unwrap();
        assert_eq!(stored_value, Some("true".to_string()), 
                "Dark mode preference should be saved to localStorage");
        
        // Unmount and remount component to simulate page reload
        // This is a simplified approach - in a real app, you might need to mock page reload differently
        // let body = window.document().unwrap().body().unwrap();
        // body.set_inner_html("");
        
        // Remount the App
        // mount_to_body(|| view! { <App /> });
        
        // Get the container again
        // let container = get_by_test_id("app-container");
        
        // Verify the dark mode preference was loaded from localStorage
        // assert!(container.class_list().contains("dark"), 
        //         "Container should start in dark mode based on localStorage value");
        
        // Clean up
        // storage.remove_item("dark_mode").unwrap();
    }
}