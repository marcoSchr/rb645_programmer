use clap::{Parser, Subcommand};
use env_logger::Builder;
use log::{LevelFilter, error, info};
use rb645communication::{read_data, send_connect, write_channels, write_settings};
use rb645memory::channel::default_channels::{default_frs_channels, default_pmr_channels};
use rb645memory::settings::default_settings::default_settings;
use std::time::Duration;

#[derive(Subcommand, Debug)]
enum Command {
    Read,
    WriteDefaultPmr,
    WriteDefaultFrs,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Serial Port radio is connected to
    #[arg(short, long)]
    serial_port: String,
    #[command(subcommand)]
    command: Command,
}

fn main() {
    Builder::new().filter_level(LevelFilter::Info).init();

    let args = Args::parse();
    info!("Starting");

    let mut port = serialport::new(args.serial_port, 9600)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");

    info!("Sending Connect");
    match send_connect(&mut port) {
        Ok(_) => {
            info!("Connected!");
            match args.command {
                Command::Read => {
                    let channels = read_data(&mut port);
                    for (channel_index, channel) in channels.unwrap().iter().enumerate() {
                        info!("Channel {}: {:?}", channel_index, channel);
                    }
                }
                Command::WriteDefaultPmr => {
                    let channels = default_pmr_channels();
                    write_channels(&mut port, channels).unwrap();
                    write_settings(&mut port, default_settings()).unwrap();
                    info!("Channels written!");
                }
                Command::WriteDefaultFrs => {
                    let channels = default_frs_channels();
                    write_channels(&mut port, channels).unwrap();
                    write_settings(&mut port, default_settings()).unwrap();
                    info!("Channels written!");
                }
            }
        }
        Err(_) => {
            error!("Error connecting to device");
        }
    }
}
