extern crate serial;
use serial::prelude::*;

use std::{string::String, env, io, collections::HashMap, time::Duration};

const PARAMETER_TABLE : [&'static str; 2] = ["--port-name", "--disable-output"];

/// Main function.
fn main() {

    let arg_hash: Option<HashMap<String, String>> = process_arguments();

    // Read command line arguments.
    match arg_hash {
        None => return,
        _ => app(arg_hash.unwrap())
    }
}

fn app(arg_hash: HashMap<String, String>) {

}

struct CmdLineArg {
    arg_str : &'static str,
    param_str : Option<&'static str>,
    is_required : bool,
    explanation : &'static str
}

const CMD_LINE_ARGS : [CmdLineArg; 2] =
[
    CmdLineArg {
        arg_str : "--port-name",
        param_str : Some("[PORT]"),
        is_required : true,
        explanation : "Sets serial port"
    },

    CmdLineArg {
        arg_str : "--disable-output",
        param_str : None,
        is_required : false,
        explanation : "Disables incoming debug messages from the console"
    }
];

fn process_arguments() -> Option<HashMap<String, String>> {

    if env::args_os().count() <= 1 {
        println!("rspsxserial command line arguments:");

        for arg in CMD_LINE_ARGS.iter() {
            let mut line = String::new();

            line.push_str(arg.arg_str);

            match arg.param_str {
                None => {},
                _ => {
                    line.push(' ');
                    line.push_str(arg.param_str.unwrap());
                }
            };

            line.push('\t');
            line.push_str(arg.explanation);

            println!("{}", line);
        }
        return None;
    }

    enum ExpectedParameter
    {
        ParameterOption,
        ParameterValue
    };

    let mut parameter_state = ExpectedParameter::ParameterOption;

    let mut arg_hash: HashMap<String, String> = HashMap::new();
    let mut parameter_name = String::new();

    for arg in env::args_os().skip(1) {
        let arg_str = arg.into_string().unwrap();

        match parameter_state {
            ExpectedParameter::ParameterOption => {
                if arg_str.starts_with("--")
                {
                    // Looks like a valid parameter

                    for param in CMD_LINE_ARGS.iter() {

                        if arg_str == param.arg_str.to_string() {
                            println!("Found valid parameter {}", arg_str);
                            parameter_name = arg_str;

                            match param.param_str {
                                None => {
                                    println!("This option uses no parameters");
                                    arg_hash.insert(parameter_name.clone(), String::from(""));
                                    parameter_state = ExpectedParameter::ParameterOption;
                                }

                                _ => {
                                    parameter_state = ExpectedParameter::ParameterValue;
                                }
                            }

                            break;
                        }
                    }
                }
                else
                {
                    return None;
                }
            },

            ExpectedParameter::ParameterValue => {
                let parameter_value = arg_str;

                arg_hash.insert(parameter_name.clone(), parameter_value.clone());
                parameter_state = ExpectedParameter::ParameterOption;
            }
        }
    }

    println!("Hash = {:?}", arg_hash);

    Some(arg_hash)
}

/** let mut port = serial::open(&arg).unwrap();
        interact(&mut port).unwrap();*/

/// This function reconfigures a serial port with default parameters
fn interact<T: SerialPort>(port: &mut T) -> io::Result<()> {
    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud9600)?;
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    })?;

    port.set_timeout(Duration::from_millis(1000))?;

    let buf: Vec<u8> = (0..255).collect();

    port.write(&buf[..])?;
    //port.read(&mut buf[..])?;

    Ok(())
}
