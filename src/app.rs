use std:: {
    string::String,
    io:: {
        Result, Error, ErrorKind
    },
    collections::HashMap
};

/// This function is called once all command line a
/// rguments have been successfully parsed, and tries
/// to establish a TCP connection against a front-end
/// if configured by command line parameters.
pub fn app(arg_hash: HashMap<String, String>) -> Result<()> {
    use cmdline;

    let addr = arg_hash.get(&String::from(cmdline::TCP_ARG));

    match addr {
        None => println!("No TCP address specified"),
        Some(addr) => setup_tcp(addr)?
    };

    // Since this should never return None, always unwrap() it.
    let port_name = arg_hash.get(&String::from(cmdline::PORT_NAME_ARG)).unwrap();

    // Extract baud rate from command line parameters,
    // but don't process it yet.
    let baud_rate = arg_hash.get(&String::from(cmdline::BAUDRATE_ARG));

    // Extract folder where CD-ROM file system is mounted.
    let folder = arg_hash.get(&String::from(cmdline::CDIMG_FOLDER)).expect("Invalid given folder");

    serial_comm(addr, port_name, baud_rate, folder)?;

    Ok(())
}

fn setup_tcp(tcp_addr : &String) -> Result<()> {

    use std::net::{TcpListener};

    let listener = TcpListener::bind(tcp_addr)?;

    println!("Awaiting for connection on address {}", tcp_addr);

    //~ for stream in listener.incoming() {
        //~ match stream {
            //~ Ok(s) => {
                //~ // do something with the TcpStream
            //~ }
            //~ Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                //~ // wait until network socket is ready, typically implemented
                //~ // via platform-specific APIs such as epoll or IOCP
                //~ continue;
            //~ }
            //~ Err(e) => panic!("encountered IO error: {}", e),
        //~ }
    //~ }

    Ok(())
}

fn serial_comm(addr : Option<&String>, port_name : &String, baud_rate : Option<&String>, folder : &String) -> Result<()> {
    use transfer;
    use transfer::TransferState;

    let mut port = serial_init(addr, port_name, baud_rate).unwrap();

    let mut state = TransferState::FirstContact;
    let mut prev_state = state;
    let mut sent_bytes = 0 as usize;
    let mut requested_file = String::new();
    let exe_data = transfer::get_exe_data(&folder).unwrap();

    loop {
        state = match state {
            TransferState::FirstContact => transfer::first_contact(&mut port),
            TransferState::WaitAck => {
                state = transfer::wait_ack_default(&mut port, prev_state);
                prev_state = state;
                state
            },
            TransferState::SendHeader => transfer::send_header(&mut port, &exe_data),
            TransferState::SendExeSize => transfer::send_exe_size(&mut port, &exe_data),
            TransferState::CleaningRAM => transfer::wait_ack_default(&mut port, prev_state),
            TransferState::SendExeData => transfer::send_exe_data(&mut port, &mut sent_bytes, &exe_data),
            TransferState::WaitFileRequest => transfer::wait_file_request(&mut port, &mut requested_file),
            TransferState::SendFile => transfer::send_file(&mut port, &mut sent_bytes, &requested_file),
            TransferState::Finished => break
        };
    }

    Ok(())
}

/// This function initializes a serial device.
/// Command line parameters are extracted and parsed here.
fn serial_init(addr : Option<&String>, port_name : &String, baud_rate : Option<&String>) -> Result<serial::SystemPort> {
    use serial::SerialPort;

    // Try to open the serial device. If opened,
    // a SystemPort instance will be returned.
    let port = serial::open(port_name);

    let baud =  match baud_rate {
        // Assign default baud rate if no
        // option was specified.
        None => serial::Baud115200,
        Some(b) => {
            match b.parse() {
                // Parse user-specific baud rate.
                Ok(s) => serial::BaudRate::from_speed(s),
                // Could not parse input baud rate.
                Err(_) => return Err(Error::new(ErrorKind::Other, "Invalid baudrate")),
            }
        }
    };

    // This variable will be bound to a SystemPort
    // instance if everything could be configured successfully.
    let mut port_unwrapped;

    match port {
       Err(_) => {
           return Err(Error::new(ErrorKind::NotFound, "Could not open serial device"));
       },

       Ok(p) => {
           port_unwrapped = p;
       }
    };

    let settings =
        serial::PortSettings {
            baud_rate: baud,
            char_size: serial::Bits8,
            parity: serial::ParityNone,
            stop_bits: serial::Stop1,
            flow_control: serial::FlowNone
        };

    port_unwrapped.configure(&settings)?;

    // Return SystemPort instance if successful.
    Ok(port_unwrapped)
}
