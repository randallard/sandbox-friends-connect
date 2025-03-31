#[cfg(test)]
pub(crate) mod test {
    use web_sys::wasm_bindgen::JsCast;
    use gloo_timers::future::TimeoutFuture;
    use std::path::Path;
    use std::fs;

    pub fn get_by_test_id(test_id: &str) -> web_sys::Element {
        let document = web_sys::window().unwrap().document().unwrap();
        document.query_selector(&format!("[data-test-id='{}']", test_id))
            .unwrap()
            .expect(&format!("Element with data-test-id='{}' not found", test_id))
    }
    
    pub async fn click_and_wait(element: &web_sys::Element, timeout_ms: u32) {
        let event = web_sys::MouseEvent::new("click").unwrap();
        element.dispatch_event(&event).unwrap();
        
        // Wait for the specified timeout to allow reactivity to complete
        let _ = TimeoutFuture::new(timeout_ms).await;
    }

    #[test]
    pub fn test_index_html_exists() {
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
    pub fn test_tailwind_config_exists() {
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
}

// Re-export test helpers at the module level for easier imports
#[cfg(test)]
pub use test::*;