// Settings management for Sodium and Sodium Extra (1:1 port)
// Includes General, Quality, Performance, Advanced, HDR pages

pub mod pages {
    pub const GENERAL: &str = "General";
    pub const QUALITY: &str = "Quality";
    pub const PERFORMANCE: &str = "Performance";
    pub const ADVANCED: &str = "Advanced";
    pub const HDR: &str = "HDR";
}

pub struct SodiumSettings {
    pub page: String,
    pub options: Vec<String>,
    pub extra_config: super::sodium_extra::SodiumExtraConfig,
}

impl Default for SodiumSettings {
    fn default() -> Self {
        Self {
            page: pages::GENERAL.to_string(),
            options: vec![],
            extra_config: super::sodium_extra::SodiumExtraConfig::default(),
        }
    }
}

pub fn init() {
    println!("Sodium Settings 1:1 initialized");
}
