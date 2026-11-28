use log::{debug, error, info};
use rb645memory::channel::Channel;
use rb645memory::settings::Settings;
use serialport::SerialPort;
use std::io::Write;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

const SUCCESS: u8 = 6;
const SERIAL_READ_DELAY: Duration = Duration::from_millis(10);

fn send_command(
    serial_port: &mut Box<dyn SerialPort>,
    command: &[u8],
    rx_length: usize,
) -> Result<Vec<u8>, ()> {
    let mut receive_buffer: Vec<u8> = vec![0; rx_length];
    serial_port.write_all(command).expect("Write failed!");
    sleep(SERIAL_READ_DELAY);
    loop {
        let bytes_read = serial_port.read(receive_buffer.as_mut_slice());
        match bytes_read {
            Ok(bytes_read) => {
                if bytes_read <= rx_length {
                    return Ok(receive_buffer.clone());
                } else {
                    error!("Received more bytes than expected");
                    return Err(());
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
            Err(e) => {
                error!("Error reading from serial port: {:?}", e);
                return Err(());
            }
        }
    }
}

fn send_connect0(serial_port: &mut Box<dyn SerialPort>) -> Result<(), ()> {
    let command = [0x02, 0x4d, 0x35, 0x39, 0x47, 0x52, 0x41, 0x4d];
    let mut retries = 3;
    let mut result;
    loop {
        result = send_command(serial_port, &command, 1);
        match &result {
            Ok(rx) => {
                if rx[0] == SUCCESS {
                    break;
                }
            }
            Err(_) => {
                if retries == 0 {
                    break;
                }
                retries -= 1;
            }
        }
    }
    Ok(())
}

fn send_connect1(serial_port: &mut Box<dyn SerialPort>) -> Result<(), ()> {
    let command = [0x02];
    let mut retries = 3;
    let mut result;
    loop {
        result = send_command(serial_port, &command, 7);
        match &result {
            Ok(rx) => {
                if rx[0] == SUCCESS {
                    debug!("{:?}", rx);
                    break;
                }
            }
            Err(_) => {
                if retries == 0 {
                    break;
                }
                retries -= 1;
            }
        }
    }
    Ok(())
}

fn send_connect2(serial_port: &mut Box<dyn SerialPort>) -> Result<(), ()> {
    let command = [0x06];
    let mut retries = 3;
    let mut result;
    loop {
        result = send_command(serial_port, &command, 1);
        match &result {
            Ok(rx) => {
                if rx[0] == SUCCESS {
                    break;
                }
            }
            Err(_) => {
                if retries == 0 {
                    break;
                }
                retries -= 1;
            }
        }
    }
    Ok(())
}

fn send_connect3(serial_port: &mut Box<dyn SerialPort>) -> Result<(), ()> {
    let command = [0x05];
    let mut retries = 3;
    let mut result;
    loop {
        result = send_command(serial_port, &command, 3);
        match &result {
            Ok(rx) => {
                debug!("{:?}", rx);
                break;
            }
            Err(_) => {
                if retries == 0 {
                    break;
                }
                retries -= 1;
            }
        }
    }
    Ok(())
}

fn send_connect4(serial_port: &mut Box<dyn SerialPort>) -> Result<(), ()> {
    let command = [0x06];
    let mut retries = 3;
    let mut result;
    loop {
        result = send_command(serial_port, &command, 1);
        match &result {
            Ok(rx) => {
                if rx[0] == SUCCESS {
                    break;
                }
            }
            Err(_) => {
                if retries == 0 {
                    break;
                }
                retries -= 1;
            }
        }
    }
    Ok(())
}

pub fn send_connect(serial_port: &mut Box<dyn SerialPort>) -> Result<(), ()> {
    send_connect0(serial_port)?;
    send_connect1(serial_port)?;
    send_connect2(serial_port)?;
    send_connect3(serial_port)?;
    send_connect4(serial_port)?;
    Ok(())
}

pub fn read_data(serial_port: &mut Box<dyn SerialPort>) -> Result<Vec<Option<Channel>>, ()> {
    let mut command = [0x52, 0x00, 0x00, 0x0b];
    let mut data = vec![];
    for i in 0..22 {
        let read_address: i16 = (i) * 11;
        let read_address_bytes = read_address.to_be_bytes();
        command[1] = read_address_bytes[0];
        command[2] = read_address_bytes[1];
        debug!("Sending command: {:x?}", command);
        let rx = send_command(serial_port, &command, 15)?;
        if rx[0] == 0x57 && rx[1] == command[1] && rx[2] == command[2] && rx[3] == 0x0b {
            let (_, rx_data) = rx.split_at(4);
            debug!("Received data from device: {:x?}", rx_data);
            if rx_data.iter().all(|x| *x == 0xff) {
                data.push(None)
            } else {
                match rx_data.try_into() {
                    Ok(channel) => {
                        data.push(Some(channel));
                    }
                    Err(_) => {
                        error!("Error converting data to channel");
                        data.push(None);
                    }
                }
            }
        } else {
            error!("Error reading data from device");
            exit(1);
        }
    }
    let read_address: i16 = 22 * 11;
    let read_address_bytes = read_address.to_be_bytes();
    command[1] = read_address_bytes[0];
    command[2] = read_address_bytes[1];
    debug!("Sending command: {:x?}", command);
    let rx = send_command(serial_port, &command, 15)?;
    let (_, rx_data) = rx.split_at(4);
    let settings: Settings = rx_data.try_into()?;
    info!("Settings: {:#?}", settings);
    Ok(data)
}

pub fn write_channels(
    serial_port: &mut Box<dyn SerialPort>,
    channels: Vec<Option<Channel>>,
) -> Result<(), ()> {
    if channels.len() > 22 {
        error!("Too many channels");
        Err(())
    } else {
        for i in 0..22 {
            let mut command: Vec<u8> = vec![0x57];
            let write_address: u16 = (i as u16) * 11;
            let write_address_bytes = write_address.to_be_bytes();
            command.push(write_address_bytes[0]);
            command.push(write_address_bytes[1]);
            command.push(0x0b);
            if let Some(Some(channel)) = channels.get(i) {
                let channel_data: Vec<u8> = channel.into();
                command.extend(channel_data);
            } else {
                command.extend([0xff; 11]);
            }
            debug!("Sending command: {:x?}", command);
            let rx = send_command(serial_port, &command, 1)?;
            match rx[0] {
                SUCCESS => (),
                i => {
                    error!("Error writing channel: {}", i);
                    exit(1);
                }
            }
        }
        Ok(())
    }
}

pub fn write_settings(serial_port: &mut Box<dyn SerialPort>, settings: Settings) -> Result<(), ()> {
    let mut command: Vec<u8> = vec![0x57];
    let write_address: u16 = (22) * 11;
    let write_address_bytes = write_address.to_be_bytes();
    command.push(write_address_bytes[0]);
    command.push(write_address_bytes[1]);
    command.push(0x0b);
    let settings_data: Vec<u8> = settings.into();
    command.extend(settings_data);
    debug!("Sending command: {:x?}", command);
    let rx = send_command(serial_port, &command, 1)?;
    match rx[0] {
        SUCCESS => (),
        i => {
            error!("Error writing settings: {}", i);
            exit(1);
        }
    }
    Ok(())
}
