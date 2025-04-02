// Create a new file named mock_logger.rs in the src directory

#[cfg(test)]
pub mod mock {
    use leptos::*;
    use leptos::prelude::*;
    use web_sys::{console, Element};
    use wasm_bindgen::JsValue;
    use std::cell::RefCell;
    use std::rc::Rc;
    
    // A component that stores log messages for testing
    #[derive(Clone)]
    pub struct LogCollector {
        info_logs: Rc<RefCell<Vec<String>>>,
        warn_logs: Rc<RefCell<Vec<String>>>,
        error_logs: Rc<RefCell<Vec<String>>>,
    }
    
    impl LogCollector {
        pub fn new() -> Self {
            Self {
                info_logs: Rc::new(RefCell::new(Vec::new())),
                warn_logs: Rc::new(RefCell::new(Vec::new())),
                error_logs: Rc::new(RefCell::new(Vec::new())),
            }
        }
        
        pub fn record_info(&self, message: &str) {
            self.info_logs.borrow_mut().push(message.to_string());
            // Also log to console for debugging
            console::log_1(&JsValue::from_str(&format!("INFO: {}", message)));
        }
        
        pub fn record_warn(&self, message: &str) {
            self.warn_logs.borrow_mut().push(message.to_string());
            // Also log to console for debugging
            console::warn_1(&JsValue::from_str(&format!("WARN: {}", message)));
        }
        
        pub fn record_error(&self, message: &str) {
            self.error_logs.borrow_mut().push(message.to_string());
            // Also log to console for debugging
            console::error_1(&JsValue::from_str(&format!("ERROR: {}", message)));
        }
        
        pub fn contains_info(&self, pattern: &str) -> bool {
            self.info_logs.borrow().iter().any(|log| log.contains(pattern))
        }
        
        pub fn contains_warn(&self, pattern: &str) -> bool {
            self.warn_logs.borrow().iter().any(|log| log.contains(pattern))
        }
        
        pub fn contains_error(&self, pattern: &str) -> bool {
            self.error_logs.borrow().iter().any(|log| log.contains(pattern))
        }
        
        pub fn info_count(&self) -> usize {
            self.info_logs.borrow().len()
        }
        
        pub fn warn_count(&self) -> usize {
            self.warn_logs.borrow().len()
        }
        
        pub fn error_count(&self) -> usize {
            self.error_logs.borrow().len()
        }
        
        pub fn clear(&self) {
            self.info_logs.borrow_mut().clear();
            self.warn_logs.borrow_mut().clear();
            self.error_logs.borrow_mut().clear();
        }
    }
    
    // Create a global log collector that can be accessed from tests
    thread_local! {
        static GLOBAL_LOG_COLLECTOR: RefCell<Option<LogCollector>> = RefCell::new(None);
    }
    
    pub fn init_log_collector() -> LogCollector {
        let collector = LogCollector::new();
        GLOBAL_LOG_COLLECTOR.with(|global| {
            *global.borrow_mut() = Some(collector.clone());
        });
        collector
    }
    
    pub fn get_log_collector() -> Option<LogCollector> {
        GLOBAL_LOG_COLLECTOR.with(|global| global.borrow().clone())
    }
    
    // Component that logs messages when certain actions occur
    #[component]
    pub fn LogTestApp() -> impl IntoView {
        // Use a signal instead of store_value for reactivity and to avoid thread safety issues
        let collector = init_log_collector();
        let collector_clone1 = collector.clone();
        let collector_clone2 = collector.clone();
        let collector_clone3 = collector.clone();
        let collector_clone4 = collector.clone();
        
        let log_info = move |_| {
            collector_clone1.record_info("Test info message");
        };
        
        let log_warn = move |_| {
            collector_clone2.record_warn("Test warning message");
        };
        
        let log_error = move |_| {
            collector_clone3.record_error("Test error message");
        };
        
        let log_all = move |_| {
            collector_clone4.record_info("All levels info");
            collector_clone4.record_warn("All levels warning");
            collector_clone4.record_error("All levels error");
        };
        
        view! {
            <div class="p-4">
                <h1 data-test-id="log-test-header" class="text-xl mb-4">"Log Test App"</h1>
                <div class="space-y-2">
                    <button 
                        data-test-id="log-info-button"
                        class="bg-blue-500 text-white px-4 py-2 rounded"
                        on:click=log_info
                    >
                        "Log Info"
                    </button>
                    
                    <button 
                        data-test-id="log-warn-button"
                        class="bg-yellow-500 text-white px-4 py-2 rounded"
                        on:click=log_warn
                    >
                        "Log Warning"
                    </button>
                    
                    <button 
                        data-test-id="log-error-button"
                        class="bg-red-500 text-white px-4 py-2 rounded"
                        on:click=log_error
                    >
                        "Log Error"
                    </button>
                    
                    <button 
                        data-test-id="log-all-button"
                        class="bg-purple-500 text-white px-4 py-2 rounded"
                        on:click=log_all
                    >
                        "Log All Levels"
                    </button>
                </div>
            </div>
        }
    }
}

// Tests for the mock logger
#[cfg(test)]
mod tests {
    use super::mock::*;
    use wasm_bindgen_test::*;
    use leptos::*;
    use leptos::prelude::*;
    use crate::test_utils::test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    async fn test_log_collector() {
        // Initialize a new log collector
        let collector = LogCollector::new();
        
        // Record some log messages
        collector.record_info("Test info");
        collector.record_warn("Test warning");
        collector.record_error("Test error");
        
        // Check that the logs were recorded
        assert!(collector.contains_info("Test info"), "Info log should be recorded");
        assert!(collector.contains_warn("Test warning"), "Warning log should be recorded");
        assert!(collector.contains_error("Test error"), "Error log should be recorded");
        
        // Check log counts
        assert_eq!(collector.info_count(), 1, "Should have 1 info log");
        assert_eq!(collector.warn_count(), 1, "Should have 1 warning log");
        assert_eq!(collector.error_count(), 1, "Should have 1 error log");
        
        // Clear logs
        collector.clear();
        
        // Check that logs were cleared
        assert_eq!(collector.info_count(), 0, "Info logs should be cleared");
        assert_eq!(collector.warn_count(), 0, "Warning logs should be cleared");
        assert_eq!(collector.error_count(), 0, "Error logs should be cleared");
    }
    
    #[wasm_bindgen_test]
    async fn test_log_test_app() {
        // Mount the LogTestApp
        mount_to_body(|| view! { <LogTestApp /> });
        
        // Get the log buttons
        let info_button = get_by_test_id("log-info-button");
        let warn_button = get_by_test_id("log-warn-button");
        let error_button = get_by_test_id("log-error-button");
        let all_button = get_by_test_id("log-all-button");
        
        // Get the global log collector
        let collector = get_log_collector().expect("Log collector should be initialized");
        
        // Click the info button
        click_and_wait(&info_button, 50).await;
        assert!(collector.contains_info("Test info message"), "Info log should be recorded");
        
        // Click the warn button
        click_and_wait(&warn_button, 50).await;
        assert!(collector.contains_warn("Test warning message"), "Warning log should be recorded");
        
        // Click the error button
        click_and_wait(&error_button, 50).await;
        assert!(collector.contains_error("Test error message"), "Error log should be recorded");
        
        // Clear logs
        collector.clear();
        
        // Click the all button
        click_and_wait(&all_button, 50).await;
        assert!(collector.contains_info("All levels info"), "Info log should be recorded");
        assert!(collector.contains_warn("All levels warning"), "Warning log should be recorded");
        assert!(collector.contains_error("All levels error"), "Error log should be recorded");
    }
}