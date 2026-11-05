#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PowerState { BatterySaver, #[default] BalancedPerformance, HighPerformance }

#[derive(Debug, Default)]
pub struct HardwareManager {
    active_power: PowerState,
    battery_percentage: u8,
}

impl HardwareManager {
    pub fn new() -> Self { Self { active_power: PowerState::BalancedPerformance, battery_percentage: 100 } }
    pub fn update_battery(&mut self, pct: u8) {
        self.battery_percentage = pct;
        if pct < 20 {
            self.active_power = PowerState::BatterySaver;
        }
    }
    pub fn current_power_state(&self) -> PowerState { self.active_power }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hardware_battery_power_state_transition() {
        let mut hw = HardwareManager::new();
        hw.update_battery(15);
        assert_eq!(hw.current_power_state(), PowerState::BatterySaver);
    }
}
