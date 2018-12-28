extern crate serial;
use serial::prelude::*;

use std::{string::String, env, io, collections::HashMap, time::Duration};

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
    println!("App!");
}

struct CmdLineArg {
    arg_str : &'static str,
    param_str : Option<&'static str>,
    is_required : bool,
    explanation : &'static str
}

impl CmdLineArg {
    fn get_explanation(&self) -> String {
        self.explanation.to_string()
    }

    fn get_string(&self) -> String {
        self.arg_str.to_string()
    }

    fn get_param(&self) -> Option<String> {
        match self.param_str {
            None => None,
            _ => Some(self.param_str.unwrap().to_string())
        }
    }

    fn has_parameters(&self) -> bool {
        match self.param_str {
            None => false,
            _ => true
        }
    }
}

const CMD_LINE_ARGS : [CmdLineArg; 3] =
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
    },

    CmdLineArg {
        arg_str : "--baudrate",
        param_str : Some("[BAUDRATE]"),
        is_required : false,
        explanation : "Sets serial port baudrate. Defaults to 4800bps"
    }
];

fn show_help() {
    println!("rspsxserial command line arguments:");

    for arg in CMD_LINE_ARGS.iter() {
        let mut line = String::new();

        line.push_str(arg.arg_str);

        if arg.has_parameters() {
                line.push(' ');
                line.push_str(arg.param_str.unwrap());
        };

        line.push('\t');
        line.push_str(arg.explanation);
        line.push('.');

        println!("{}", line);
    }
}

fn process_arguments() -> Option<HashMap<String, String>> {

    if env::args_os().count() <= 1 {
        show_help();
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

    // Check all needed parameters have been given
    for arg in CMD_LINE_ARGS.iter() {
        if arg.is_required {
            let arg_string = arg.arg_str.to_string();

            if !arg_hash.contains_key(&arg_string) {
                println!("Missing required option {}", arg_string);
                return None
            }
        }
    }

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
