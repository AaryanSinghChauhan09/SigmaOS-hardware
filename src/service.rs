use crate::config::Config;
use crate::error::{ComponentError, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum ServiceStatus { Stopped, Running, Failed(String) }

pub struct Service {
    config: Config,
    status: ServiceStatus,
}

impl Service {
    pub fn new(config: Config) -> Self { Self { config, status: ServiceStatus::Stopped } }
    pub fn start(&mut self) -> Result<()> {
        if self.status == ServiceStatus::Running { return Err(ComponentError::Service("already running".to_string())); }
        self.status = ServiceStatus::Running; Ok(())
    }
    pub fn stop(&mut self) -> Result<()> {
        if self.status == ServiceStatus::Stopped { return Err(ComponentError::Service("not running".to_string())); }
        self.status = ServiceStatus::Stopped; Ok(())
    }
    pub fn is_running(&self) -> bool { self.status == ServiceStatus::Running }
    pub fn config(&self) -> &Config { &self.config }
}
