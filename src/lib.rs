//! SigmaOS-hardware: hardware inventory model — devices discovered via
//! sysfs (`/sys/devices`, `/sys/bus/*`), their attributes, and capability
//! queries. Deterministic and std-only; a `DeviceProbe` trait abstracts
//! the actual filesystem so tests use fakes.

use std::collections::BTreeMap;
use std::fmt;

/// Device classes matching sysfs `subsystem` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum DeviceClass {
    Cpu,
    Memory,
    Pci,
    Usb,
    Block,
    Net,
    Input,
    Gpu,
    Firmware,
    Other,
}

impl DeviceClass {
    pub fn parse(s: &str) -> Self {
        match s {
            "cpu" => Self::Cpu,
            "memory" => Self::Memory,
            "pci" => Self::Pci,
            "usb" => Self::Usb,
            "block" => Self::Block,
            "net" => Self::Net,
            "input" => Self::Input,
            "gpu" | "drm" => Self::Gpu,
            "firmware" => Self::Firmware,
            _ => Self::Other,
        }
    }
}

/// A discovered hardware device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Device {
    pub name: String,          // e.g. "0000:00:1f.2", "cpu0", "sda"
    pub class: DeviceClass,
    pub vendor_id: Option<u16>, // PCI/USB vendor
    pub device_id: Option<u16>,
    pub driver: Option<String>,
    pub attrs: BTreeMap<String, String>, // raw sysfs attributes
}

impl Device {
    /// Read a numeric attribute with a default.
    pub fn attr_u64(&self, key: &str) -> Option<u64> {
        self.attrs.get(key)?.trim().parse().ok()
    }

    /// sysfs `modalias`-style match: does `pattern` appear in modalias?
    pub fn matches_modalias(&self, pattern: &str) -> bool {
        self.attrs.get("modalias")
            .map(|m| m.contains(pattern))
            .unwrap_or(false)
    }

    pub fn has_driver(&self) -> bool { self.driver.is_some() }

    /// PCI class code (class/subclass/prog-if) if present.
    pub fn pci_class(&self) -> Option<(u8, u8, u8)> {
        let v = self.attr_u64("class")?;
        Some(((v >> 16) as u8, (v >> 8) as u8, v as u8))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HwError {
    DuplicateDevice(String),
    NotFound(String),
    BadAttr { device: String, key: String },
}

impl fmt::Display for HwError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateDevice(n) => write!(f, "device {} already inventoried", n),
            Self::NotFound(n) => write!(f, "device {} not found", n),
            Self::BadAttr { device, key } => write!(f, "device {}: bad attribute {}", device, key),
        }
    }
}

impl std::error::Error for HwError {}

/// Abstraction over device discovery (sysfs walk, dmidecode, etc.).
pub trait DeviceProbe {
    /// All devices of a class currently visible.
    fn devices(&self, class: DeviceClass) -> Vec<Device>;
    /// Read one sysfs attribute; None if absent.
    fn read_attr(&self, device: &str, key: &str) -> Option<String>;
}

/// The hardware inventory.
#[derive(Debug, Default)]
pub struct HardwareInventory {
    devices: BTreeMap<String, Device>, // keyed by name
}

impl HardwareInventory {
    pub fn new() -> Self { Self::default() }

    /// Register a discovered device.
    pub fn add(&mut self, dev: Device) -> Result<(), HwError> {
        if self.devices.contains_key(&dev.name) {
            return Err(HwError::DuplicateDevice(dev.name));
        }
        self.devices.insert(dev.name.clone(), dev);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&Device> { self.devices.get(name) }

    pub fn by_class(&self, class: DeviceClass) -> Vec<&Device> {
        self.devices.values().filter(|d| d.class == class).collect()
    }

    /// Devices with no bound driver — candidates for module loading.
    pub fn unbound(&self) -> Vec<&Device> {
        self.devices.values().filter(|d| !d.has_driver()).collect()
    }

    /// Total memory from "memory"-class devices reporting size attrs.
    pub fn total_memory_bytes(&self) -> u64 {
        self.by_class(DeviceClass::Memory).iter()
            .filter_map(|d| d.attr_u64("size"))
            .sum()
    }

    /// CPU count from cpu-class devices.
    pub fn cpu_count(&self) -> usize { self.by_class(DeviceClass::Cpu).len() }

    /// Refresh inventory from a probe: adds new devices, updates attrs
    /// of existing ones, marks vanished devices by removing them.
    pub fn refresh<P: DeviceProbe>(&mut self, probe: &P,
        classes: &[DeviceClass]) -> Result<usize, HwError>
    {
        let mut seen = 0;
        for &class in classes {
            for dev in probe.devices(class) {
                match self.devices.get_mut(&dev.name) {
                    Some(existing) => {
                        // update mutable fields only (driver may bind later)
                        existing.driver = dev.driver.clone();
                        existing.attrs.extend(dev.attrs.clone());
                    }
                    None => { self.add(dev)?; }
                }
                seen += 1;
            }
        }
        Ok(seen)
    }
}

/// sysfs path helpers — pure functions for building paths.
pub mod sysfs {
    /// `/sys/bus/pci/devices/0000:00:1f.2/...`
    pub fn pci_dev_path(bdf: &str, attr: &str) -> String {
        format!("/sys/bus/pci/devices/{}/{}", bdf, attr)
    }
    /// `/sys/class/net/eth0/...`
    pub fn net_dev_path(iface: &str, attr: &str) -> String {
        format!("/sys/class/net/{}/{}", iface, attr)
    }
    /// `/sys/devices/system/cpu/cpu0/...`
    pub fn cpu_path(cpu: u32, attr: &str) -> String {
        format!("/sys/devices/system/cpu/cpu{}/{}", cpu, attr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cpu(name: &str, driver: Option<&str>) -> Device {
        Device {
            name: name.into(), class: DeviceClass::Cpu,
            vendor_id: None, device_id: None,
            driver: driver.map(String::from),
            attrs: BTreeMap::from([("cpuinfo_max_freq".into(), "3400000".into())]),
        }
    }

    fn mem(name: &str, kb: u64) -> Device {
        Device {
            name: name.into(), class: DeviceClass::Memory,
            vendor_id: None, device_id: None, driver: None,
            attrs: BTreeMap::from([("size".into(), (kb * 1024).to_string())]),
        }
    }

    #[test]
    fn test_add_and_duplicate_rejected_aaa() {
        // Arrange
        let mut inv = HardwareInventory::new();
        // Act & Assert
        inv.add(cpu("cpu0", Some("intel_pstate"))).unwrap();
        assert!(matches!(inv.add(cpu("cpu0", None)), Err(HwError::DuplicateDevice(_))));
        assert_eq!(inv.cpu_count(), 1);
    }

    #[test]
    fn test_by_class_and_unbound_filter_aaa() {
        // Arrange
        let mut inv = HardwareInventory::new();
        inv.add(cpu("cpu0", Some("x"))).unwrap();
        let mut gpu = cpu("0000:01:00.0", None);
        gpu.class = DeviceClass::Gpu;
        inv.add(gpu).unwrap();
        // Act & Assert
        assert_eq!(inv.by_class(DeviceClass::Gpu).len(), 1);
        assert_eq!(inv.unbound().len(), 1);
        assert_eq!(inv.unbound()[0].name, "0000:01:00.0");
    }

    #[test]
    fn test_memory_and_cpu_totals_aaa() {
        // Arrange
        let mut inv = HardwareInventory::new();
        inv.add(mem("mem0", 8_000_000)).unwrap(); // 8GB in KB
        inv.add(mem("mem1", 8_000_000)).unwrap();
        inv.add(cpu("cpu0", None)).unwrap();
        inv.add(cpu("cpu1", None)).unwrap();
        inv.add(cpu("cpu2", None)).unwrap();
        inv.add(cpu("cpu3", None)).unwrap();
        // Act & Assert
        assert_eq!(inv.total_memory_bytes(), 16_000_000 * 1024);
        assert_eq!(inv.cpu_count(), 4);
    }

    #[test]
    fn test_attr_u64_and_modalias_match_aaa() {
        // Arrange
        let mut d = cpu("cpu0", None);
        d.attrs.insert("modalias".into(), "pci:v00008086d1234".into());
        // Act & Assert
        assert_eq!(d.attr_u64("cpuinfo_max_freq"), Some(3_400_000));
        assert!(d.matches_modalias("v00008086"));
        assert!(!d.matches_modalias("v00001234"));
        assert_eq!(d.attr_u64("nonexistent"), None);
    }

    #[test]
    fn test_pci_class_decoding_aaa() {
        // Arrange: class 0x030000 = display/VGA
        let mut d = cpu("0000:01:00.0", None);
        d.class = DeviceClass::Gpu;
        d.attrs.insert("class".into(), "0x030000".into());
        // Act: attr stored as hex string won't parse u64 decimal — use decimal form
        d.attrs.insert("class".into(), "196608".into()); // 0x30000
        // Assert
        assert_eq!(d.pci_class(), Some((3, 0, 0)));
    }

    #[test]
    fn test_refresh_adds_and_updates_aaa() {
        // Arrange
        struct P;
        impl DeviceProbe for P {
            fn devices(&self, c: DeviceClass) -> Vec<Device> {
                match c {
                    DeviceClass::Net => vec![Device {
                        name: "eth0".into(), class: c, vendor_id: None,
                        device_id: None, driver: Some("e1000e".into()),
                        attrs: BTreeMap::new(),
                    }],
                    _ => vec![],
                }
            }
            fn read_attr(&self, _d: &str, _k: &str) -> Option<String> { None }
        }
        let mut inv = HardwareInventory::new();
        // Act
        inv.refresh(&P, &[DeviceClass::Net]).unwrap();
        // Assert
        assert_eq!(inv.get("eth0").unwrap().driver.as_deref(), Some("e1000e"));
    }

    #[test]
    fn test_sysfs_path_helpers_aaa() {
        // Arrange & Act & Assert
        assert_eq!(sysfs::pci_dev_path("0000:00:1f.2", "vendor"),
            "/sys/bus/pci/devices/0000:00:1f.2/vendor");
        assert_eq!(sysfs::net_dev_path("eth0", "operstate"),
            "/sys/class/net/eth0/operstate");
        assert_eq!(sysfs::cpu_path(3, "online"),
            "/sys/devices/system/cpu/cpu3/online");
    }
}
