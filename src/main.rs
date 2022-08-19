use std::{env, path::Path};

static HELP_MESSAGE: &str = "Usage: cfpl <type> <type parameter>
                                    \n\ttype:
                                    \n\t   --file or -f
                                    \n\t   type parameter:
                                    \n\t      <filename with path> (i.e. \"my_source_code.cfpl\")
                                    \n\t   --string or -s
                                    \n\t   type parameter:
                                    \n\t      <string source code> (i.e. \"VAR ab as INT\\nSTART\\nOUTPUT: ab\\nSTOP\")
                                    ";

enum ArgumentType {
    String,
    File,
}

struct Config<'a> {
    argument_type: ArgumentType,
    argument_type_parameter: &'a str,
}

impl Config<'_> {
    fn new(argument: &[String]) -> Result<Config, String> {
        if argument.len() != 3 {
            return Err(HELP_MESSAGE.to_owned());
        }

        let mut config = Config {
            argument_type: ArgumentType::File,
            argument_type_parameter: &argument[2],
        };

        return match argument[1].as_str() {
            "--file" | "-f" => {
                if let Some(extension) = Path::new(config.argument_type_parameter)
                    .extension()
                    .and_then(|extension| extension.to_str())
                {
                    match extension {
                        "cfpl" | "txt" => {
                            config.argument_type = ArgumentType::File;
                            Ok(config)
                        }
                        _ => Err(format!(
                            "Invalid file extension {}\n{}",
                            extension, HELP_MESSAGE
                        )),
                    }
                } else {
                    Err(format!("Unidentified file\n{}", HELP_MESSAGE))
                }
            }
            "--string" | "-s" => {
                config.argument_type = ArgumentType::String;
                Ok(config)
            }
            _ => Err(format!(
                "Invalid argument type: {}\n{}",
                argument[1], HELP_MESSAGE
            )),
        };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    let is_success = match config {
        Ok(config) => match config.argument_type {
            ArgumentType::File => cfpl::file(config.argument_type_parameter),
            ArgumentType::String => cfpl::execute(config.argument_type_parameter.to_owned()),
        },
        Err(error) => {
            eprint!("{}", error);
            false
        }
    };

    std::process::exit(if is_success { 0 } else { 1 });
}
