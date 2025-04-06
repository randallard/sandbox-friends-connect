#[cfg(test)]
mod theme_tests {
    use leptos::*;
    use leptos::prelude::*;
    use wasm_bindgen_test::*;
    use crate::test_utils::test::*;
    use crate::theme::{ThemeProvider, use_theme};
    use crate::utils::get_storage_item;
    use crate::utils::localStorage::{reset_theme_storage, set_storage_item};
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[component]
    fn TestThemeComponent() -> impl IntoView {
        let theme = use_theme();
        
        // Create a function to toggle the theme
        let toggle_theme = move |_| {
            theme.toggle_theme.dispatch(());
        };
        
        view! {
            <div class=format!("theme-test-container {}", if theme.dark_mode.get() { "dark" } else { "light" })>
                <p data-test-id="theme-status">
                    if theme.dark_mode {
                        "Dark Mode"
                    } else {
                        "Light Mode"
                    }
                </p>
                <button 
                    data-test-id="toggle-theme-button"
                    on:click=toggle_theme
                >
                    "Toggle Theme"
                </button>
            </div>
        }
    }
    
    #[wasm_bindgen_test]
    async fn test_theme_provider_initial_state() {
        // Reset localStorage for the test using our enhanced helper
        reset_theme_storage();
        
        // Mount the component
        mount_to_body(|| view! {
            <ThemeProvider>
                <TestThemeComponent />
            </ThemeProvider>
        });
        
        // Check the initial state (should be light mode by default)
        let theme_status = get_by_test_id("theme-status");
        assert_eq!(theme_status.text_content().unwrap(), "Light Mode", 
            "Theme should start in light mode by default");
        
        // Check the container class
        let container = document().query_selector(".theme-test-container").unwrap().unwrap();
        assert!(container.class_list().contains("light"), 
            "Container should have 'light' class initially");
    }
    
    #[wasm_bindgen_test]
    async fn test_theme_toggle_functionality() {
        // Reset localStorage for the test
        reset_theme_storage();
        
        // Mount the component
        mount_to_body(|| view! {
            <ThemeProvider>
                <TestThemeComponent />
            </ThemeProvider>
        });
        
        // Get the toggle button and theme status
        let toggle_button = get_by_test_id("toggle-theme-button");
        let theme_status = get_by_test_id("theme-status");
        
        // Initial state check
        assert_eq!(theme_status.text_content().unwrap(), "Light Mode", 
            "Theme should start in light mode");
        
        // Click the toggle button
        click_and_wait(&toggle_button, 100).await;
        
        // Check that the theme status has changed
        assert_eq!(theme_status.text_content().unwrap(), "Dark Mode", 
            "Theme should switch to dark mode after toggle");
        
        // Check the container class
        let container = document().query_selector(".theme-test-container").unwrap().unwrap();
        assert!(container.class_list().contains("dark"), 
            "Container should have 'dark' class after toggle");
        
        // Check localStorage was updated using the original helper for verification
        let stored_value = get_storage_item("dark_mode").unwrap();
        assert_eq!(stored_value, Some("true".to_string()), 
            "Dark mode preference should be saved to localStorage");
    }
    
    #[wasm_bindgen_test]
    async fn test_theme_respects_stored_preference() {
        // Set a dark mode preference in localStorage using our enhanced helper
        let _ = set_storage_item("dark_mode", "true");
        
        // Mount the component
        mount_to_body(|| view! {
            <ThemeProvider>
                <TestThemeComponent />
            </ThemeProvider>
        });
        
        // Check that the theme status respects the stored preference
        let theme_status = get_by_test_id("theme-status");
        assert_eq!(theme_status.text_content().unwrap(), "Dark Mode", 
            "Theme should initialize to dark mode based on stored preference");
        
        // Check the container class
        let container = document().query_selector(".theme-test-container").unwrap().unwrap();
        assert!(container.class_list().contains("dark"), 
            "Container should have 'dark' class based on stored preference");
    }
    
    #[wasm_bindgen_test]
    async fn test_theme_context_stays_in_sync() {
        // Reset localStorage
        reset_theme_storage();
        
        // A more complex test component that has multiple components using the theme
        #[component]
        fn MultiThemeComponent() -> impl IntoView {
            view! {
                <ThemeProvider>
                    <div class="main-container">
                        <TestThemeComponent />
                        <SecondThemeComponent />
                    </div>
                </ThemeProvider>
            }
        }
        
        #[component]
        fn SecondThemeComponent() -> impl IntoView {
            let theme = use_theme();
            
            view! {
                <div class="second-component">
                    <p data-test-id="second-theme-status">
                        if theme.dark_mode {
                            "Second Component: Dark"
                        } else {
                            "Second Component: Light"
                        }
                    </p>
                </div>
            }
        }
        
        // Mount the component
        mount_to_body(|| view! { <MultiThemeComponent /> });
        
        // Get elements from both components
        let first_status = get_by_test_id("theme-status");
        let second_status = get_by_test_id("second-theme-status");
        let toggle_button = get_by_test_id("toggle-theme-button");
        
        // Initial state check - both components should be in light mode
        assert_eq!(first_status.text_content().unwrap(), "Light Mode", 
            "First component should start in light mode");
        assert_eq!(second_status.text_content().unwrap(), "Second Component: Light", 
            "Second component should start in light mode");
        
        // Click the toggle button in the first component
        click_and_wait(&toggle_button, 100).await;
        
        // Both components should now be in dark mode
        assert_eq!(first_status.text_content().unwrap(), "Dark Mode", 
            "First component should switch to dark mode after toggle");
        assert_eq!(second_status.text_content().unwrap(), "Second Component: Dark", 
            "Second component should also switch to dark mode (context is shared)");
    }
}