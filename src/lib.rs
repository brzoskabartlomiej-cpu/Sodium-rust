// Sodium-rust for Horizon Launcher
// 1:1 port of Sodium 0.7 logic + Sodium Extra extensions
// Based on Horizon PR #5706 (sodium_style.rs) and Sodium Extra 0.9.4 features

#![allow(dead_code)]

pub mod sodium_style;
pub mod sodium_extra;
pub mod rendering;
pub mod settings;

// Horizon native entry point (if used as native-mod)
#[no_mangle]
pub extern "C" fn horizon_mod_init() -> *const u8 {
    // Initialize Sodium context
    sodium_style::init();
    sodium_extra::init();
    
    // Return module metadata (placeholder for horizon-native.json)
    let metadata = b"{\"id\": \"sodium\", \"version\": \"0.7.0\", \"extra\": true}";
    metadata.as_ptr()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sodium_init() {
        sodium_style::init();
        sodium_extra::init();
        assert!(true);
    }
}
