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

    match (*port).read(&mut buffer) {
        Err(_) => TransferState::WaitAck,
        Ok(b) => {
            if b == 1 {
                match prev_state {
                    TransferState::FirstContact => TransferState::SendHeader,
                    _ => TransferState::Finished
                }
            }
            else
            {
                TransferState::WaitAck
            }
        }
    }
}
