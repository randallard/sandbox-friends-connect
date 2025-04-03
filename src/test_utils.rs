#[cfg(test)]
pub(crate) mod test {
    use web_sys::wasm_bindgen::JsCast;
    use gloo_timers::future::TimeoutFuture;
    use std::path::Path;
    use std::fs;
    use wasm_bindgen::prelude::*;
    use web_sys::{Element, Event};
    use crate::test_setup;

    // Initialize the test environment before running any tests
    pub fn setup_test() {
        test_setup::init_test_environment();
    }

    // Improved get_by_test_id with better error handling
    pub fn get_by_test_id(test_id: &str) -> web_sys::Element {
        // Ensure test environment is set up
        setup_test();
        
        let selector = format!("[data-test-id='{}']", test_id);
        match web_sys::window() {
            Some(window) => {
                match window.document() {
                    Some(document) => {
                        match document.query_selector(&selector) {
                            Ok(Some(element)) => element,
                            Ok(None) => {
                                // Try to find all elements with data-test-id
                                let mut available_ids = String::from("Available IDs: ");
                                
                                if let Ok(elements) = document.query_selector_all("[data-test-id]") {
                                    let mut found_any = false;
                                    
                                    for i in 0..elements.length() {
                                        if let Some(el) = elements.item(i) {
                                            if let Some(el) = el.dyn_ref::<web_sys::Element>() {
                                                if let Some(id) = el.get_attribute("data-test-id") {
                                                    if found_any {
                                                        available_ids.push_str(", ");
                                                    }
                                                    available_ids.push_str(&id);
                                                    found_any = true;
                                                }
                                            }
                                        }
                                    }
                                }
                                
                                let msg = format!("Element with data-test-id='{}' not found. {}", test_id, available_ids);
                                web_sys::console::error_1(&JsValue::from_str(&msg));
                                panic!("{}", msg);
                            },
                            Err(err) => {
                                web_sys::console::error_2(
                                    &JsValue::from_str(&format!("Error finding element with selector '{}'", selector)),
                                    &err
                                );
                                panic!("Error finding element: {:?}", err);
                            }
                        }
                    },
                    None => {
                        web_sys::console::error_1(&JsValue::from_str("Document not found"));
                        panic!("Document not found");
                    }
                }
            },
            None => {
                web_sys::console::error_1(&JsValue::from_str("Window not found"));
                panic!("Window not found");
            }
        }
    }
    
    // Safer click_and_wait helper that uses simpler Event creation
    pub async fn click_and_wait(element: &web_sys::Element, timeout_ms: u32) {
        // Create a simpler click event to avoid potential issues
        if let Ok(event) = web_sys::Event::new("click") {
            match element.dispatch_event(&event) {
                Ok(_) => {
                    // First wait for DOM updates to process
                    test_setup::wait_for_dom_update().await;
                    
                    // Then add additional delay as needed
                    TimeoutFuture::new(timeout_ms).await;
                },
                Err(err) => {
                    web_sys::console::error_2(
                        &JsValue::from_str("Error dispatching click event"),
                        &err
                    );
                    // Continue anyway to avoid blocking tests
                    TimeoutFuture::new(timeout_ms).await;
                }
            }
        } else {
            web_sys::console::error_1(&JsValue::from_str("Error creating click event"));
            // Continue anyway to avoid blocking tests
            TimeoutFuture::new(timeout_ms).await;
        }
    }

    // Safer query_selector that avoids potential issues
    pub fn query_selector(selector: &str) -> Option<web_sys::Element> {
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                doc.query_selector(selector).ok().flatten()
            })
    }

    // Wait for an element to appear with improved error handling
    pub async fn wait_for_element(test_id: &str, max_attempts: u32, delay_ms: u32) -> Option<web_sys::Element> {
        let selector = format!("[data-test-id='{}']", test_id);
        
        for i in 0..max_attempts {
            if let Some(element) = query_selector(&selector) {
                return Some(element);
            }
            
            // Log waiting status
            if i > 0 && i % 5 == 0 {
                web_sys::console::log_1(&JsValue::from_str(
                    &format!("Waiting for element '{}' (attempt {}/{})", test_id, i, max_attempts)
                ));
            }
            
            // Wait between attempts
            TimeoutFuture::new(delay_ms).await;
        }
        
        None
    }

    // FS tests that are not WASM related
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