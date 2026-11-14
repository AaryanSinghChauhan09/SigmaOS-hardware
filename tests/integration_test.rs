use sigmaos_hardware::{Device, DeviceClass, HardwareInventory};
use std::collections::BTreeMap;

#[test]
fn component_constructs_and_inventories() {
    let mut inv = HardwareInventory::new();
    inv.add(Device {
        name: "cpu0".into(), class: DeviceClass::Cpu,
        vendor_id: None, device_id: None,
        driver: Some("intel_pstate".into()),
        attrs: BTreeMap::new(),
    }).unwrap();
    assert_eq!(inv.cpu_count(), 1);
}
