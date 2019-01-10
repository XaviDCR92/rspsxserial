#[derive(Copy, Clone)]
pub enum TransferState {
    FirstContact,
    WaitAck,
    SendHeader,
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
                println!("Written 1 byte: {}", INITIAL_TRANSMISSION);
                TransferState::WaitAck
            }
            else
            {
                TransferState::FirstContact
            }
        }
    }
}

pub fn wait_ack(port : &mut serial::SystemPort, prev_state: TransferState) -> TransferState {

    // For some reason, this trait has to be imported,
    // but shouldn't serial::SerialPort be already doing this?
    use std::io::Read;

    let mut buffer : [u8; 1] = [0];

    use serial::SerialPort;

    (*port).set_timeout(std::time::Duration::from_secs(2)).expect("Could not adjust timeout");

    match (*port).read(&mut buffer) {
        Err(_) => {
            prev_state
        },
        Ok(b) => {
            if b == 1 {
                if buffer[0] == 'b' as u8 {
                    println!("Received ACK");
                    match prev_state {
                        TransferState::FirstContact => {
                            println!("Now send header");
                            TransferState::SendHeader
                        },
                        _ => TransferState::Finished
                    }
                }
                else
                {
                    prev_state
                }
            }
            else
            {
                prev_state
            }
        }
    }
}

pub fn send_header(port : &mut serial::SystemPort, folder : &String) -> TransferState {
    match get_exe_name(folder) {
        None => TransferState::Finished,
        Some(exe_name) => {
            let exe_path = format!("{}/{}", folder, exe_name);

            use std::fs;

            match fs::read(&exe_path) {
                Err(e) => {
                    println!("{:?}. File path: {}", e, exe_path);
                    TransferState::Finished
                },
                Ok(data) => {
                    use std::io::Write;

                    const HEADER_SIZE : usize = 32 as usize;
                    let mut header = data.clone();

                    header.truncate(HEADER_SIZE);

                    (*port).write(&header).expect("Could not write EXE header into the device");

                    TransferState::WaitAck
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
