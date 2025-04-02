use web_sys::Storage;
use uuid::Uuid;

// Helper functions for localStorage
pub fn get_storage() -> Option<Storage> {
    web_sys::window()
        .and_then(|win| win.local_storage().ok())
        .flatten()
}

// Uses the uuid crate to generate a player ID
pub fn generate_player_id() -> String {
    Uuid::new_v4().to_string()
}

// Helper function to get or create player ID from localStorage
pub fn get_player_id() -> String {
    let stored = get_storage()
        .and_then(|storage| storage.get_item("player_id").ok())
        .flatten();
    
    match stored {
        Some(id) => id,
        None => {
            // Generate a new player ID and store it
            let new_id = generate_player_id();
            if let Some(storage) = get_storage() {
                let _ = storage.set_item("player_id", &new_id);
            }
            new_id
        }
    }
}

// Helper function to get dark mode preference from localStorage
pub fn get_dark_mode_preference() -> bool {
    get_storage()
        .and_then(|storage| storage.get_item("dark_mode").ok())
        .flatten()
        .map(|val| val == "true")
        .unwrap_or(false)
}

// Helper function to save dark mode preference to localStorage
pub fn save_dark_mode_preference(is_dark: bool) {
    if let Some(storage) = get_storage() {
        let _ = storage.set_item("dark_mode", if is_dark { "true" } else { "false" });
    }
}