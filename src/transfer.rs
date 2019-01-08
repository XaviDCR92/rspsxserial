#[derive(Copy, Clone)]
pub enum TransferState {
    FirstContact,
    WaitAck,
    SendHeader,
    Finished
}

use serial;

pub fn first_contact(port : &mut serial::SystemPort) -> TransferState {
    const INITIAL_TRANSMISSION: u8 = 'b' as u8;
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
                        TransferState::FirstContact => TransferState::SendHeader,
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
    use std::fs;
    use regex::Regex;

    let data_buffer = fs::read_to_string(format!("{}/{}", folder, "SYSTEM.CNF")).unwrap();
    lazy_static! {
        static ref RX: Regex = Regex::new(r"BOOT\s*=\s*cdrom:\(.+\.EXE);1").unwrap();
    }

    println!("{:?}", RX.captures(&data_buffer).unwrap());

    TransferState::SendHeader
}
