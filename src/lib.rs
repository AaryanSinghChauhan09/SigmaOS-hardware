pub mod config;
pub mod device_registry;
pub mod error;
pub mod service;

pub use config::Config;
pub use device_registry::{Device, DeviceClass, DeviceRegistry};
pub use error::{ComponentError, Result};
pub use service::{Service, ServiceStatus};

pub trait Component {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn initialize(&mut self) -> error::Result<()>;
    fn shutdown(&mut self) -> error::Result<()>;
    fn health_check(&self) -> error::Result<()>;
}

pub struct HardwareComponent {
    service: service::Service,
}

impl HardwareComponent {
    pub fn new(config: Config) -> Self {
        Self { service: service::Service::new(config) }
    }
}

impl Component for HardwareComponent {
    fn name(&self) -> &str { "sigmaos-hardware" }
    fn version(&self) -> &str { env!("CARGO_PKG_VERSION") }
    fn initialize(&mut self) -> error::Result<()> { self.service.start() }
    fn shutdown(&mut self) -> error::Result<()> { self.service.stop() }
    fn health_check(&self) -> error::Result<()> {
        if self.service.is_running() { Ok(()) }
        else { Err(error::ComponentError::Service("hardware service not running".to_string())) }
    }
}
