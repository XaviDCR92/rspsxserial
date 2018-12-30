const initial_transmission : u8 = 'b' as u8;

pub enum TransferState {
    FirstContact,
    WaitAck,
    Finished
}

use serial;
use serial::SystemPort;

pub fn first_contact(port : &mut serial::SystemPort) -> TransferState {

    *port.write(&initial_transmission);
    TransferState::WaitAck
}
