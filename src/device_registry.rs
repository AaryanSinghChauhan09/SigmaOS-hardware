#[derive(Debug, Clone, PartialEq)]
pub enum DeviceClass {
    Cpu,
    Memory,
    Storage,
    Network,
    Usb,
    Gpu,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Device {
    pub id: String,
    pub class: DeviceClass,
    pub vendor: String,
    pub model: String,
    pub enabled: bool,
}

impl Device {
    pub fn new(id: &str, class: DeviceClass, vendor: &str, model: &str) -> Self {
        Self { id: id.to_string(), class, vendor: vendor.to_string(), model: model.to_string(), enabled: true }
    }
}

pub struct DeviceRegistry {
    devices: Vec<Device>,
}

impl DeviceRegistry {
    pub fn new() -> Self { Self { devices: Vec::new() } }

    pub fn register(&mut self, device: Device) { self.devices.push(device); }

    pub fn count(&self) -> usize { self.devices.len() }

    pub fn by_class(&self, class: &DeviceClass) -> Vec<&Device> {
        self.devices.iter().filter(|d| &d.class == class).collect()
    }

    pub fn find(&self, id: &str) -> Option<&Device> {
        self.devices.iter().find(|d| d.id == id)
    }

    pub fn enabled_count(&self) -> usize {
        self.devices.iter().filter(|d| d.enabled).count()
    }
}

impl Default for DeviceRegistry {
    fn default() -> Self { Self::new() }
}
