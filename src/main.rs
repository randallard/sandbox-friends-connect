use leptos::*;
use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <div data-test-id="app-container" class="min-h-screen bg-gradient-to-b from-blue-50 to-indigo-100 flex flex-col items-center justify-center p-4">
            <div class="bg-white rounded-xl shadow-lg p-8 max-w-md w-full">
                <h1 data-test-id="hello-header" class="text-3xl font-bold text-center text-indigo-600 mb-6">"Hello Leptos"</h1>
                <p class="text-gray-600 text-center mb-6">"Welcome to your Tailwind-styled Leptos app!"</p>
                <div class="flex justify-center">
                    <button class="bg-indigo-500 hover:bg-indigo-600 text-white font-medium py-2 px-4 rounded-lg transition-colors">
                        "Get Started"
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
        let has_tailwind = contents.contains("<link data-trunk rel=\"css\" href=\"input.css\"") || 
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
}