extern crate serial;

mod cmdline;
mod app;

/// Main function.
fn main() {
    // Read command line arguments.
    match cmdline::process_arguments() {
        None => return,
        Some(hash) => {
            match app::app(hash) {
                _ => return
            }
        }
    };
}

//~ /** let mut port = serial::open(&arg).unwrap();
        //~ interact(&mut port).unwrap();*/

//~ /// This function reconfigures a serial port with default parameters
//~ fn interact<T: SerialPort>(port: &mut T) -> io::Result<()> {
    //~ port.reconfigure(&|settings| {
        //~ settings.set_baud_rate(serial::Baud9600)?;
        //~ settings.set_char_size(serial::Bits8);
        //~ settings.set_parity(serial::ParityNone);
        //~ settings.set_stop_bits(serial::Stop1);
        //~ settings.set_flow_control(serial::FlowNone);
        //~ Ok(())
    //~ })?;

    //~ port.set_timeout(Duration::from_millis(1000))?;

    //~ let buf: Vec<u8> = (0..255).collect();

    //~ port.write(&buf[..])?;
    //~ //port.read(&mut buf[..])?;

    //~ Ok(())
//~ }
