use leptos::*;
use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    // Create a signal to track dark mode state
    let (dark_mode, set_dark_mode) = create_signal(false);
    
    // Toggle function for the dark mode
    let toggle_dark_mode = move |_| {
        set_dark_mode.update(|dark| *dark = !*dark);
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

fn main() {
    mount_to_body(|| view! { <App /> });
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use std::path::Path;
    use std::fs;

    wasm_bindgen_test_configure!(run_in_browser);
    
    fn get_by_test_id(test_id: &str) -> web_sys::Element {
        let document = web_sys::window().unwrap().document().unwrap();
        document.query_selector(&format!("[data-test-id='{}']", test_id))
            .unwrap()
            .expect(&format!("Element with data-test-id='{}' not found", test_id))
    }

    #[test]
    fn test_index_html_exists() {
        let index_path = Path::new("index.html");
        assert!(index_path.exists(), "index.html file doesn't exist");
        
        let contents = fs::read_to_string(index_path).expect("Should be able to read the index.html file");
        
        // Check for required elements in index.html
        assert!(contents.contains("<link data-trunk rel=\"rust\""), 
                "index.html is missing Trunk link tag");
        
        assert!(contents.contains("<meta charset=\"utf-8\""), 
                "index.html is missing UTF-8 charset declaration");
        
        assert!(contents.contains("<meta name=\"viewport\""), 
                "index.html is missing viewport meta tag");
    
        // New check for Tailwind CSS
        let has_tailwind = contents.contains("<link data-trunk rel=\"css\" href=\"dist/tailwind.css\"") || 
                            contents.contains("https://cdn.tailwindcss.com");
        
        assert!(has_tailwind, "index.html is missing Tailwind CSS link");
    }
    
    #[test]
    fn test_tailwind_config_exists() {
        let path = Path::new("tailwind.config.js");
        
        // Skip this test if using CDN approach
        let index_contents = fs::read_to_string("index.html").unwrap_or_default();
        if index_contents.contains("https://cdn.tailwindcss.com") {
            // Using CDN, so configuration file is not required
            return;
        }
        
        assert!(path.exists(), "tailwind.config.js file doesn't exist");
        
        let contents = fs::read_to_string(path).expect("Should be able to read the tailwind.config.js file");
        
        // Check for required content in tailwind.config.js
        assert!(contents.contains("content"), 
                "tailwind.config.js is missing content configuration");
                
        assert!(contents.contains("./src/**/*.rs"), 
                "tailwind.config.js is not configured to process Rust files");
    }
    
    #[wasm_bindgen_test]
    fn test_app_says_hello_leptos() {
        // Mount the App component to the body
        mount_to_body(|| view! { <App /> });
        
        // Test that the component renders "Hello Leptos"
        let header = get_by_test_id("hello-header");
        assert_eq!(header.text_content().unwrap(), "Hello Leptos");
    }
    
    #[wasm_bindgen_test]
    fn test_tailwind_classes_applied() {
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
    fn test_dark_mode_toggle_exists() {
        // Mount the App component to the body
        mount_to_body(|| view! { <App /> });
        
        // Use the helper function to get the element by test ID
        let dark_mode_toggle = get_by_test_id("dark-mode-toggle");
        
        // The test will now pass if the toggle exists (get_by_test_id will panic if not found)
        assert!(dark_mode_toggle.is_object(), "Dark mode toggle should exist");
    }

    #[wasm_bindgen_test]
    fn test_dark_mode_toggle_changes_theme() {
        // Mount the App component to the body
        mount_to_body(|| view! { <App /> });
        
        // Get the dark mode toggle
        let dark_mode_toggle = get_by_test_id("dark-mode-toggle");
        
        // Get the container
        let container = get_by_test_id("app-container");
        
        // Check initial theme (should be light by default)
        assert!(!container.class_list().contains("dark"), 
                "Container should start in light mode");
        
        // Click the toggle
        let event = web_sys::MouseEvent::new("click").unwrap();
        dark_mode_toggle.dispatch_event(&event).unwrap();
        
        // Check that the theme has changed
        assert!(container.class_list().contains("dark"), 
                "Container should switch to dark mode after toggle click");
        
        // Click the toggle again
        dark_mode_toggle.dispatch_event(&event).unwrap();
        
        // Check that the theme has changed back
        assert!(!container.class_list().contains("dark"), 
                "Container should switch back to light mode after second toggle click");
    }
}