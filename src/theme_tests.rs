#[cfg(test)]
mod theme_tests {
    use leptos::*;
    use leptos::prelude::*;
    use wasm_bindgen_test::*;
    use crate::test_utils::test::*;
    use crate::theme::{ThemeProvider, use_theme};
    use crate::utils::localStorage::reset_theme_storage;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[component]
    fn TestThemeComponent() -> impl IntoView {
        let theme = use_theme();
        
        // Create a function to toggle the theme
        let toggle_theme = move |_| {
            theme.toggle_theme.dispatch(());
        };
        
        view! {
            <div>
                <p data-test-id="theme-status">
                    {move || if theme.dark_mode.get() { "dark" } else { "light" }}
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
    async fn test_theme_toggle() {
        // Reset theme storage to start with a clean state
        reset_theme_storage();
        
        // Mount the test component
        mount_to_body(|| view! {
            <ThemeProvider>
                <TestThemeComponent />
            </ThemeProvider>
        });
        
        // Get status and toggle elements
        let theme_status = get_by_test_id("theme-status");
        let toggle_button = get_by_test_id("toggle-theme-button");
        
        // Check initial theme (should be light by default)
        let initial_theme = theme_status.text_content().unwrap();
        
        // Click the toggle button
        click_and_wait(&toggle_button, 200).await;
        
        // Check that the theme changed
        let new_theme = theme_status.text_content().unwrap();
        assert_ne!(initial_theme, new_theme, "Theme should change after toggling");
        
        // Toggle back and verify it changed again
        click_and_wait(&toggle_button, 200).await;
        
        let final_theme = theme_status.text_content().unwrap();
        assert_eq!(initial_theme, final_theme, "Theme should revert to initial state after toggling twice");
    }
}