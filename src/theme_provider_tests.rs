#[cfg(test)]
mod theme_provider_tests {
    use leptos::*;
    use leptos::prelude::*;
    use wasm_bindgen_test::*;
    use crate::test_utils::test::*;
    use crate::theme::{ThemeProvider, use_theme};
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    async fn test_theme_provider_with_no_children() {
        // This test ensures ThemeProvider works even without children
        mount_to_body(|| view! {
            <ThemeProvider />
        });
        
        // If this doesn't throw an exception, the component works without children
        assert!(true);
    }
    
    #[wasm_bindgen_test]
    async fn test_theme_provider_with_some_children() {
        // Component that uses theme context
        #[component]
        fn ChildComponent() -> impl IntoView {
            // Try to access theme context
            let theme = use_theme();
            
            view! {
                <div data-test-id="theme-child">
                    {if theme.dark_mode { "Dark" } else { "Light" }}
                </div>
            }
        }
        
        // Mount ThemeProvider with children
        mount_to_body(|| view! {
            <ThemeProvider>
                <ChildComponent />
            </ThemeProvider>
        });
        
        // If we can get the child element, the children were rendered
        let child = get_by_test_id("theme-child");
        assert!(child.is_object(), "Child component should be rendered");
        
        // Check that theme context was provided to the child
        assert!(child.text_content().unwrap() == "Light" || 
                child.text_content().unwrap() == "Dark", 
                "Child should have access to theme context");
    }
}