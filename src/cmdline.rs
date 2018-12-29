use std::{string::String, env, collections::HashMap};

pub struct CmdLineArg {
    pub arg_str : &'static str,
    pub param_str : Option<&'static str>,
    is_required : bool,
    explanation : &'static str
}

pub const CMD_LINE_ARGS : [CmdLineArg; 4] =
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
    },

    CmdLineArg {
        arg_str : "--tcp",
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

pub fn process_arguments() -> Option<HashMap<String, String>> {

    if env::args_os().count() <= 1 {
        // Invalid number of command line arguments.
        // Show default help dialog.
        show_help();
        return None;
    }

    // This enum defines a finite-state machine that
    // will be used to determine what type of data is
    // being retrieved
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
