use crate::config::Config;
use clap::Parser;
use device::Device;

mod config;
mod device;

#[derive(Parser)]
struct Cli {
    /// Config file
    #[arg(short, long)]
    config: String,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let config = Config::from_file(&args.config).expect("Failed to read config");

    let mut workers = Vec::new();
    for descriptor in config.devices {
        println!("Spawning {} {}", descriptor.name, descriptor.path);
        let mut worker = Device::new(descriptor);
        worker.run();
        workers.push(worker);
    }

    for worker in workers {
        worker.handle.unwrap().await.expect("BUG");
    }
}
