use std::{string::String, io, collections::HashMap};

/// This function is called once all command line a
/// rguments have been successfully parsed, and tries
/// to establish a TCP connection against a front-end
/// if configured by command line parameters.
pub fn app(arg_hash: HashMap<String, String>) -> io::Result<()> {

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

    serial_comm(addr, port_name, baud_rate)?;

    Ok(())
}

fn setup_tcp(tcp_addr : &String) -> io::Result<()> {

    use std::net::{TcpListener};

    let listener = TcpListener::bind(tcp_addr)?;

    println!("Awaiting for connection on address {}", tcp_addr);

    //~ for stream in listener.incoming() {
        //~ match stream {
            //~ Ok(s) => {
                //~ // do something with the TcpStream
            //~ }
            //~ Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                //~ // wait until network socket is ready, typically implemented
                //~ // via platform-specific APIs such as epoll or IOCP
                //~ continue;
            //~ }
            //~ Err(e) => panic!("encountered IO error: {}", e),
        //~ }
    //~ }

    Ok(())
}

fn serial_comm(addr : Option<&String>, port_name : &String, baud_rate : Option<&String>) -> io::Result<()> {
    serial_init(addr, port_name, baud_rate).unwrap();

    Ok(())
}

fn serial_init(addr : Option<&String>, port_name : &String, baud_rate : Option<&String>) -> io::Result<serial::SystemPort> {
    use serial::SerialPort;

    let port = serial::open(port_name);

    let mut port_unwrapped;

    let baud =  match baud_rate {
        None => serial::Baud4800,
        Some(b) => {
            match b.parse() {
                Ok(4800) => serial::Baud4800,
                Ok(9600) => serial::Baud9600,
                Err(_) | Ok(_) => return Err(io::Error::new(io::ErrorKind::Other, "Invalid baudrate")),
            }
        }
    };

    match port {
       Err(_) => {
           println!();
           return Err(io::Error::new(io::ErrorKind::NotFound, "Could not open serial device"));
       },

       Ok(p) => {
           port_unwrapped = p;
       }
    };

    let settings : serial::PortSettings = serial::PortSettings {
                baud_rate: baud,
                char_size: serial::Bits8,
                parity: serial::ParityOdd,
                stop_bits: serial::Stop1,
                flow_control: serial::FlowNone
            };

    port_unwrapped.configure(&settings)?;

    Ok(port_unwrapped)
}
