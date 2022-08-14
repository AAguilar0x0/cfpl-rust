use std::{env, path::Path};

const HELP_MESSAGE: &'static str = "Usage: cfpl <type> <type parameter>
                                    \n\ttype:
                                    \n\t   --file or -f
                                    \n\t   type parameter:
                                    \n\t      <filename with path> (i.e. \"my_source_code.cfpl\")
                                    \n\t   --string or -s
                                    \n\t   type parameter:
                                    \n\t      <string source code> (i.e. \"VAR ab as INT\\nSTART\\nOUTPUT: ab\\nSTOP\")
                                    ";

struct Config<'a> {
    argument_type: &'a str,
    argument_type_parameter: &'a str,
}

impl Config<'_> {
    fn new(argument: &[String]) -> Result<Config, String> {
        if argument.len() != 3 {
            return Err(HELP_MESSAGE.to_owned());
        }

        let config = Config {
            argument_type: &argument[1],
            argument_type_parameter: &argument[2],
        };

        return match config.argument_type {
            "--file" | "-f" => {
                if let Some(extension) = Path::new(config.argument_type_parameter)
                    .extension()
                    .and_then(|extension| extension.to_str())
                {
                    match extension {
                        "cfpl" | "txt" => Ok(config),
                        _ => Err(format!(
                            "Invalid file extension {}\n{}",
                            extension, HELP_MESSAGE
                        )),
                    }
                } else {
                    Err(format!("Unidentified file\n{}", HELP_MESSAGE))
                }
            }
            "--string" | "-s" => Ok(config),
            _ => Err(format!(
                "Invalid argument type: {}\n{}",
                config.argument_type, HELP_MESSAGE
            )),
        };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    let is_success = match config {
        Ok(config) => cfpl::file(config.argument_type_parameter),
        Err(error) => {
            eprint!("{}", error);
            false
        }
    };

    std::process::exit(if is_success { 0 } else { 1 });
}
