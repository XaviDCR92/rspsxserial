use std::{string::String, io, collections::HashMap};

/// This function is called once all command line a
/// rguments have been successfully parsed, and tries
/// to establish a TCP connection against a front-end
/// if configured by command line parameters.
pub fn app(arg_hash: HashMap<String, String>) -> io::Result<()> {

    match arg_hash.get(&String::from("--tcp")) {
        None => println!("No TCP address specified"),
        Some(addr) => setup_tcp(addr)?
    };

    Ok(())
}

fn setup_tcp(tcp_addr : &String) -> io::Result<()> {

    use std::net::{TcpListener};

    let listener = TcpListener::bind(tcp_addr)?;

    println!("Awaiting for connection on address {}", tcp_addr);

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                // do something with the TcpStream
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                // wait until network socket is ready, typically implemented
                // via platform-specific APIs such as epoll or IOCP
                continue;
            }
            Err(e) => panic!("encountered IO error: {}", e),
        }
    }

    Ok(())
}
