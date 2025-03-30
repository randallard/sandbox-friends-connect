use leptos::*;
use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <div data-test-id="app-container">
            <h1 data-test-id="hello-header">"Hello Leptos"</h1>
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

    #[wasm_bindgen_test]
    fn test_app_says_hello_leptos() {
        // Mount the App component to the body
        mount_to_body(|| view! { <App /> });
        
        // Test that the component renders "Hello Leptos"
        let header = get_by_test_id("hello-header");
        assert_eq!(header.text_content().unwrap(), "Hello Leptos");
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
    }
}