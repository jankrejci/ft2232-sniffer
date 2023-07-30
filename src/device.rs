use anyhow::Result;
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::time::Duration;
use tokio::task;

const READ_TIMEOUT: Duration = Duration::from_secs(1);

#[derive(Deserialize)]
pub struct Descriptor {
    pub path: String,
    pub name: String,
    pub baudrate: u32,
}

pub struct Device {
    descriptor: Descriptor,
    pub handle: Option<task::JoinHandle<()>>,
}

impl Device {
    pub fn new(descriptor: Descriptor) -> Self {
        Device {
            descriptor,
            handle: None,
        }
    }

    pub fn run(&mut self) {
        println!("Starting {}", self.descriptor.name);

        self.handle = Some(tokio::spawn(Self::worker(
            self.descriptor.name.clone(),
            self.descriptor.path.clone(),
            self.descriptor.baudrate,
        )));
    }

    pub async fn worker(name: String, path: String, baudrate: u32) {
        println!("Starting worker {}", name);
        let mut port = tokio_serial::new(path, baudrate)
            .timeout(READ_TIMEOUT)
            .open()
            .expect("Failed to open device");
        println!("Port opened {}", name);

        let mut file = File::create(format!("{}.txt", name)).expect("Failed to create file");

        println!("File opened {}", name);

        let mut buffer = [0u8; 1024];
        loop {
            let hex_values = match port.read(&mut buffer) {
                Ok(n) => buffer[..n]
                    .iter()
                    .map(|&byte| format!("{:02X}", byte))
                    .collect::<Vec<_>>()
                    .join(" "),

                Err(e) => {
                    println!("{} {:?}", name, e);
                    continue;
                }
            };
            writeln!(file, "{}", hex_values).expect("Failed to write file");
        }
    }
}
