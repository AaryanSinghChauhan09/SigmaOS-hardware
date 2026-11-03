use clap::Parser;
use hardware::hardware;

#[derive(Parser)]
struct Cli;

fn main() -> anyhow::Result<()> {
    let config = hardware::Config::default();
    let mut component = hardware::new(config);
    component.initialize()?;
    println!("SigmaOS-hardware ready");
    Ok(())
}
