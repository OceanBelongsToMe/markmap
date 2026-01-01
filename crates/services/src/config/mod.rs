pub mod service;

use crate::builder::ServiceRegistry;
use crate::config::service::{GetEffectiveConfig, GetGlobalConfig, UpdateGlobalConfig};

pub fn register(registry: &mut ServiceRegistry) {
    GetGlobalConfig::register(registry);
    UpdateGlobalConfig::register(registry);
    GetEffectiveConfig::register(registry);
}
