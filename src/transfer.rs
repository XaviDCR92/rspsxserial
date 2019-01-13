#[derive(Copy, Clone)]
pub enum TransferState {
    FirstContact,
    WaitAck,
    SendHeader,
    SendExeSize,
    CleaningRAM,
    SendExeData,
    WaitFileRequest,
    SendFile,
    Finished
}

use serial;

pub fn first_contact(port : &mut serial::SystemPort) -> TransferState {
    const INITIAL_TRANSMISSION: u8 = 99u8;
    use std::io::Write;

    match (*port).write(&vec![INITIAL_TRANSMISSION]) {
        Err(_) => TransferState::FirstContact,
        Ok(b) => {
            if b == 1 {
                TransferState::WaitAck
            }
            else
            {
                TransferState::FirstContact
            }
        }
    }
}

fn wait_ack(port : &mut serial::SystemPort, buffer : &mut [u8]) -> Result<usize, std::io::Error> {
    // For some reason, this trait has to be imported,
    // but shouldn't serial::SerialPort be already doing this?
    use std::io::Read;

    use serial::SerialPort;

    const TIMEOUT_SECONDS : u64 = 2;

    (*port).set_timeout(std::time::Duration::from_secs(TIMEOUT_SECONDS)).expect("Could not adjust timeout");

    (*port).read(buffer)
}

pub fn wait_ack_default(port : &mut serial::SystemPort, prev_state: TransferState) -> TransferState {
    let mut buffer : [u8; 1] = [0];

    match wait_ack(port, &mut buffer) {
        Ok(1) => {
            if *(buffer.get(0).unwrap()) == 'b' as u8 {
                match prev_state {
                    TransferState::FirstContact => {
                        println!("Got response from the device");
                        TransferState::SendHeader
                    },
                    TransferState::SendHeader => TransferState::SendExeSize,
                    TransferState::SendExeSize => TransferState::CleaningRAM,
                    TransferState::CleaningRAM => TransferState::SendExeData,
                    TransferState::SendExeData => TransferState::SendExeData,
                    _ => TransferState::Finished
                }
            }
            else
            {
                prev_state
            }
        },
        _ => {
            match prev_state {
                TransferState::SendExeSize => TransferState::CleaningRAM,
                _ => prev_state
            }
        }
    }
}

const PACKET_SIZE : usize = 8 as usize;

pub fn send_header(port : &mut serial::SystemPort, exe_data: &Vec<u8>) -> TransferState {

    const HEADER_SIZE : usize = 32 as usize;
    for packet in (0..HEADER_SIZE).step_by(PACKET_SIZE) {
        match exe_data.get(packet..(packet + PACKET_SIZE)) {
            None => return TransferState::Finished,
            Some(chunk) => {
                use std::{thread, time};

                thread::sleep(time::Duration::from_millis(100));

                use std::io::Write;
                (*port).write(&chunk).expect("Could not write EXE header into the device");
            }
        }
    }

    TransferState::WaitAck
}

const EXE_DATA_OFFSET : usize = 2048 as usize;

pub fn send_exe_size(port: &mut serial::SystemPort, exe_data: &Vec<u8>) -> TransferState {
    if exe_data.len() > EXE_DATA_OFFSET {
        let exe_size = exe_data.len() - EXE_DATA_OFFSET;

        use std::io::Write;

        let exe_size_vec : [u8; 4] = [(exe_size & 0xFF) as u8,
                                      ((exe_size & 0xFF00) >> 8) as u8,
                                      ((exe_size & 0xFF0000) >> 16) as u8,
                                      ((exe_size & 0xFF000000) >> 24) as u8];
        (*port).write(&exe_size_vec).expect("Could not write EXE size into the device");

        TransferState::WaitAck
    }
    else
    {
        println!("PSX-EXE is too small");
        TransferState::Finished
    }
}

pub fn send_exe_data(port: &mut serial::SystemPort, sent_bytes: &mut usize, exe_data: &Vec<u8>) -> TransferState {
    let exe_size = exe_data.len();

    let total_sent_bytes = *sent_bytes + EXE_DATA_OFFSET;

    if total_sent_bytes < exe_size {
        match exe_data.get(total_sent_bytes..(total_sent_bytes + PACKET_SIZE)) {
            None => return TransferState::Finished,
            Some(chunk) => {
                use std::io::Write;
                (*port).write(&chunk).expect("Could not write EXE header into the device");

                *sent_bytes += PACKET_SIZE;

                if *sent_bytes % 32 == 0 {
                    print!("\rSent {:?}/{:?} bytes...", *sent_bytes, exe_size - EXE_DATA_OFFSET);
                }
            }
        }

        TransferState::WaitAck
    }
    else
    {
        println!("Finished");

        // Reset number of sent bytes.
        *sent_bytes = 0;
        TransferState::WaitFileRequest
    }
}

/// This function waits for a file read request from the device and
/// constructs a valid file name for it. If no valid data is provided,
/// this state is re-entered cyclically.
pub fn wait_file_request(port : &mut serial::SystemPort, requested_file : &mut String) -> TransferState {
    // For some reason, this trait has to be imported,
    // but shouldn't serial::SerialPort be already doing this?
    use std::io::Read;

    use serial::SerialPort;

    const TIMEOUT_SECONDS : u64 = 5;

    let mut buffer : [u8; 128] = [0; 128];

    (*port).set_timeout(std::time::Duration::from_secs(TIMEOUT_SECONDS)).expect("Could not adjust timeout");

    match (*port).read(&mut buffer) {
        Err(_) | Ok(0) => {
            println!("No information has been received yet");
            TransferState::WaitFileRequest
        },
        Ok(_) => get_file_name(&buffer.to_vec(), requested_file)
    }
}

fn get_file_name(buffer : &Vec<u8>, requested_file: &mut String) -> TransferState {
    // No valid header byte has been found yet.
    match buffer.iter().position(|&c| c == '#' as u8) {
        None => TransferState::WaitFileRequest,
        Some(pos) => {
            let final_pos : usize =
                match buffer.iter().position(|&c| c == '@' as u8) {
                    None => buffer.len() - 1,
                    Some(l) => l
                };
            requested_file.clone_from(&String::from_utf8(buffer[pos + 1..final_pos].to_vec()).unwrap());

            println!("Requested file: {}", requested_file);

            if requested_file.ends_with(";1") {
                TransferState::SendFile
            }
            else
            {
                TransferState::WaitFileRequest
            }
        }
    }
}

pub fn send_file(port : &mut serial::SystemPort, sent_bytes: &mut usize, requested_file: &String) -> TransferState {
    println!("Requested file: {}", requested_file);
    TransferState::Finished
}

pub fn get_exe_data(folder: &String) -> Option<Vec<u8>> {
    match get_exe_name(folder) {
        None => None,
        Some(exe_name) => {
            let exe_path = format!("{}/{}", folder, exe_name);

            use std::fs;

            match fs::read(&exe_path) {
                Err(e) => {
                    println!("{}. File path: {}", e, exe_path);
                    None
                },
                Ok(data) => {
                    Some(data)
                }
            }
        }
    }
}

fn get_exe_name(folder : &String) -> Option<String> {
    use std::fs;
    use regex::Regex;

    let path = format!("{}/{}", folder, "SYSTEM.CNF");

    let data_buffer = fs::read_to_string(&path).unwrap();

    lazy_static! {
        static ref RX: Regex = Regex::new(r"BOOT\s*=\s*cdrom:\\([aA-zZ0-9]{1,8}\.[aA-zZ0-9]{1,3}).+").expect("Could not compile regex");
    }

    match RX.captures(&data_buffer) {
        None => {
            println!("Could not find executable name on {}", &path);
            None
        },
        Some(s) => {
            match s.get(1) {
                None => {
                    println!("Internal error");
                    None
                },
                Some(s_) => Some(String::from(s_.as_str()))
            }
        }
    }
}
