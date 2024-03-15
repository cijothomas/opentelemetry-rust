use crate::Resource;

/// Default log configuration
pub fn config() -> Config {
    Config::default()
}

/// Log emitter configuration.
#[derive(Debug, Default)]
pub struct Config {
    /// Contains attributes representing an entity that produces telemetry.
    pub resource: Resource,
}

impl Config {
    /// Specify the attributes representing the entity that produces telemetry
    pub fn with_resource(mut self, resource: Resource) -> Self {
        self.resource = resource;
        self
    }
}
