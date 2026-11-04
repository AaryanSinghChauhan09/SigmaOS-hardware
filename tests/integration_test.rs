use sigmaos_hardware::{Config, Device, DeviceClass, DeviceRegistry, HardwareComponent, Component};

#[test]
fn test_config_default_aaa() {
    let config = Config::default();
    assert!(config.enabled);
    assert!(config.scan_on_start);
}

#[test]
fn test_device_registry_register_aaa() {
    // Arrange
    let mut registry = DeviceRegistry::new();
    // Act
    registry.register(Device::new("cpu0", DeviceClass::Cpu, "Intel", "Core i9"));
    registry.register(Device::new("nvme0", DeviceClass::Storage, "Samsung", "990 Pro"));
    // Assert
    assert_eq!(registry.count(), 2);
}

#[test]
fn test_device_registry_by_class_aaa() {
    // Arrange
    let mut registry = DeviceRegistry::new();
    registry.register(Device::new("eth0", DeviceClass::Network, "Intel", "I219-V"));
    registry.register(Device::new("wlan0", DeviceClass::Network, "Realtek", "8821CE"));
    registry.register(Device::new("nvme0", DeviceClass::Storage, "Samsung", "990 Pro"));
    // Act
    let network_devs = registry.by_class(&DeviceClass::Network);
    // Assert
    assert_eq!(network_devs.len(), 2);
}

#[test]
fn test_device_registry_find_aaa() {
    // Arrange
    let mut registry = DeviceRegistry::new();
    registry.register(Device::new("gpu0", DeviceClass::Gpu, "NVIDIA", "RTX 4090"));
    // Act
    let found = registry.find("gpu0");
    // Assert
    assert!(found.is_some());
    assert_eq!(found.unwrap().vendor, "NVIDIA");
}

#[test]
fn test_device_registry_find_missing_aaa() {
    let registry = DeviceRegistry::new();
    assert!(registry.find("nonexistent").is_none());
}

#[test]
fn test_component_lifecycle_aaa() {
    let mut component = HardwareComponent::new(Config::default());
    assert!(component.initialize().is_ok());
    assert!(component.health_check().is_ok());
    assert!(component.shutdown().is_ok());
}

#[test]
fn test_component_name_aaa() {
    let component = HardwareComponent::new(Config::default());
    assert_eq!(component.name(), "sigmaos-hardware");
}
