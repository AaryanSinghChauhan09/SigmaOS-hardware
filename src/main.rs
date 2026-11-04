use sigmaos_hardware::{Config, HardwareComponent, Component};
fn main() {
    env_logger::init();
    let mut component = HardwareComponent::new(Config::default());
    if let Err(e) = component.initialize() { eprintln!("Failed: {}", e); std::process::exit(1); }
    println!("SigmaOS hardware running");
    let _ = component.shutdown();
}
