#[cfg(test)]
mod app_wasm_tests {
    use leptos::*;
    use leptos::prelude::*;
    use crate::data::DataButton;
    use crate::theme::{ThemeProvider, use_container_class, use_card_class, use_header_class, 
                      use_paragraph_class, use_button_class, use_toggle_class, use_toggle_text, use_theme};
    use log::{error, info}; // Import log macros
    
    use wasm_bindgen_test::*;
    use web_sys::{Document, wasm_bindgen::JsCast, window};
    use crate::test_utils::test::*;
    use crate::app::*;

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
    }

    #[wasm_bindgen_test]
    async fn test_data_button_integration() {
        // Mount the App component to the body
        mount_to_body(|| view! { <App /> });
        
        // Verify the data button exists when integrated into the App
        let data_button = get_by_test_id("data-button");
        assert!(data_button.is_object(), "Data button should exist when integrated into App");
    }
    
    // New test for storage error handling
    #[wasm_bindgen_test]
    async fn test_storage_error_handling() {
        // This test would simulate a localStorage failure
        // Since it's hard to mock localStorage failures directly,
        // we can check that the error element exists in the DOM structure
        
        // Mount the App component to the body
        mount_to_body(|| view! { <App /> });
        
        // Check that error message element doesn't exist initially
        let document = web_sys::window().unwrap().document().unwrap();
        let error_elements = document.query_selector_all("[data-test-id='storage-error']").unwrap();
        assert_eq!(error_elements.length(), 0, "Error message should not be visible initially");
        
        // For a complete test, we'd need to mock localStorage to fail
        // This is complex in WASM and would require additional test infrastructure
    }
}