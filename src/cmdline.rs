use std::{string::String, env, collections::HashMap};

/// This structure defines a command line
/// argument and its low-level parameters.
/// An array of CmdLineArg instances,
/// called CMD_LINE_ARGS, is defined.
pub struct CmdLineArg {
    pub arg_str : &'static str,
    pub param_str : Option<&'static str>,
    is_required : bool,
    explanation : &'static str
}

/// This parameter allows defining serial port name.
pub const PORT_NAME_ARG : &'static str = "--port-name";

/// This parameter disables sending any information
/// coming from the console to stdout.
pub const DISABLE_OUTPUT_ARG : &'static str = "--disable-output";

/// This parameter allows defining a specific baud rate,
pub const BAUDRATE_ARG : &'static str = "--baud-rate";

/// This parameter allows using a TCP connection
/// against a GUI front-end.
pub const TCP_ARG : &'static str = "--tcp";

const CMD_LINE_ARGS : [CmdLineArg; 4] =
[
    CmdLineArg {
        arg_str : PORT_NAME_ARG,
        param_str : Some("[PORT]"),
        is_required : true,
        explanation : "Sets serial port"
    },

    CmdLineArg {
        arg_str : DISABLE_OUTPUT_ARG,
        param_str : None,
        is_required : false,
        explanation : "Disables incoming debug messages from the console"
    },

    CmdLineArg {
        arg_str : BAUDRATE_ARG,
        param_str : Some("[BAUDRATE]"),
        is_required : false,
        explanation : "Sets serial port baudrate. Defaults to 115200 bps"
    },

    CmdLineArg {
        arg_str : TCP_ARG,
        param_str : Some("[IPv4:PORT]"),
        is_required : false,
        explanation : "Sets a TCP connection against a compatible \
                      front-end application"
    }
];

fn show_help() {
    println!("rspsxserial command line arguments:");

    for arg in CMD_LINE_ARGS.iter() {
        let line = format!("{} {}\t{}.",
                                arg.arg_str,
                                match arg.param_str {
                                    None => "",
                                    Some(param_str) => param_str
                                },
                                arg.explanation);

        println!("{}", line);
    }
}

/// This function creates a Hashmap instance
/// that relates all command line arguments
/// against its parameters. For example:
/// ["--baud-rate", "4800"], ["--disable-output", ""]
pub fn process_arguments() -> Option<HashMap<String, String>> {

    if env::args_os().count() <= 1 {
        // Invalid number of command line arguments.
        // Show default help dialog.
        show_help();
        return None;
    }

    // This enum defines a finite-state machine that
    // will be used to determine what type of data is
    // being retrieved from command line arguments.
    enum ExpectedParameter
    {
        ParameterOption,
        ParameterValue
    };

    impl std::fmt::Debug for ExpectedParameter {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!( f,
                    "ExpectedParameter::{}",
                    match self {
                        ExpectedParameter::ParameterOption => "ParameterOption",
                        ExpectedParameter::ParameterValue => "ParameterValue"
                    })
        }
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
                    let mut result : bool = false;

                    for param in CMD_LINE_ARGS.iter() {

                        if arg_str == param.arg_str.to_string() {
                            parameter_name = arg_str;

                            match param.param_str {
                                None => {
                                    arg_hash.insert(parameter_name.clone(), String::from(""));
                                    parameter_state = ExpectedParameter::ParameterOption;
                                }

                                Some(_) => {
                                    parameter_state = ExpectedParameter::ParameterValue;
                                }
                            }

                            result = true;

                            break;
                        }
                    }

                    if !result {
                        // Parameter could not be found inside the list.
                        println!("Invalid parameter {}", parameter_name);
                        return None
                    }
                }
                else
                {
                    return None
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
