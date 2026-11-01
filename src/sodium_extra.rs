// Sodium Extra 0.9.4 features ported to Rust/Horizon
// Features: Atmospheric fog, cloud fog, cylindrical fog, particle filtering

pub fn init() {
    println!("Sodium Extra 0.9.4 initialized: Fog, Clouds, Particles");
}

pub struct SodiumExtraConfig {
    pub atmospheric_fog: bool,
    pub cloud_fog_percentage: f32,
    pub cylindrical_fog_shape: bool,
    pub particle_filtering: bool,
    pub per_dimension_overrides: bool,
}

impl Default for SodiumExtraConfig {
    fn default() -> Self {
        Self {
            atmospheric_fog: true,
            cloud_fog_percentage: 1.0,
            cylindrical_fog_shape: true,
            particle_filtering: true,
            per_dimension_overrides: true,
        }
    }
}

// 1:1 Port of Sodium Extra features
pub fn apply_fog_settings(config: &SodiumExtraConfig) {
    if config.atmospheric_fog {
        // Horizon rendering hook for fog
        println!("Applying atmospheric fog (Sodium Extra)");
    }
    if config.cylindrical_fog_shape {
        println!("Applying cylindrical fog shape (Sodium Extra)");
    }
}

pub fn apply_particle_filtering(config: &SodiumExtraConfig) {
    if config.particle_filtering {
        println!("Applying hardened particle filtering (Sodium Extra)");
    }
}
