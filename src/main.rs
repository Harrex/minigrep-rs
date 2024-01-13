use std::{env, fs, io};

#[derive(Debug)]
enum ArgumentType {
    LongFlag,  // --whatever
    ShortFlag, // -w
    Arg,       // file.txt
}

#[derive(Debug)]
struct Argument {
    kind: ArgumentType,
    value: String,
}

#[derive(Debug)]
struct GrepOptions {
    case_sensitive: bool,
    filepath_string: String,
    grep_text: String,
}

impl GrepOptions {
    fn default() -> Self {
        GrepOptions {
            case_sensitive: true,
            filepath_string: String::from(""),
            grep_text: String::from(""),
        }
    }
}

const HELP_MESSAGE: &str = "\
    Help or somethin idk
";

fn main() {
    let args: Vec<String> = env::args().collect();
    let parsed_args = parse_args(args);

    let mut options = GrepOptions::default();
    let mut actual_args: Vec<String> = Vec::new();

    for arg in parsed_args {
        match arg.kind {
            ArgumentType::LongFlag => match arg.value.as_str() {
                "insensitive" => options.case_sensitive = true,
                "help" => {
                    println!("{}", HELP_MESSAGE)
                }
                _ => {
                    println!("Invalid argument: {}", arg.value)
                }
            },
            ArgumentType::ShortFlag => match arg.value.as_str() {
                "i" => options.case_sensitive = false,
                "h" => {
                    println!("{}", HELP_MESSAGE)
                }
                _ => {
                    println!("Invalid argument: {}", arg.value)
                }
            },
            ArgumentType::Arg => actual_args.push(arg.value),
        }
    }

    if actual_args.len() == 2 {
        options.grep_text = String::from(&actual_args[0]);
        options.filepath_string = String::from(&actual_args[1]);
    } else if actual_args.len() == 1 {
        options.grep_text = String::from(&actual_args[0]);
    }

    grep(options);
}

fn grep(mut options: GrepOptions) {
    let mut text_to_grep: String;
    if options.case_sensitive == false {
        options.grep_text = options.grep_text.to_lowercase();
    }
    if options.filepath_string.len() > 0 {
        // Check if this string is non-empty
        text_to_grep = get_file_text(options.filepath_string);
        println!("{}", grep_file_text(text_to_grep, options.grep_text));
    } else {
        loop {
            text_to_grep = String::new();
            match io::stdin().read_line(&mut text_to_grep) {
                Ok(n) => {
                    match n {
                        0 => return,
                        _ => {
                            let x = grep_stdin(text_to_grep.clone(), options.grep_text.clone());
                            if x.len() > 0 {
                                print!("{}", x);
                            }
                        },
                    }
                }
                Err(err) => println!("Error: {}", err),
            }
        }
    }
}

fn grep_file_text(input: String, pattern: String) -> String {
    let mut output = String::new();

    for line in input.lines() {
        if line.contains(&pattern) {
            let x = line.replace(&pattern, &format!("\x1b[1m{}\x1b[0m", &pattern));
            output.push_str(&format!("{}\n", &x));
        }
    }

    return output;
}
fn grep_stdin(input: String, pattern: String) -> String {
    if input.contains(&pattern) {
        let x = input.replace(&pattern, &format!("\x1b[1m{}\x1b[0m", &pattern));
        return x;
    }
    String::from("")
}

fn get_file_text(filepath: String) -> String {
    let path = fs::canonicalize(filepath).expect("Couldn't get file path");
    fs::read_to_string(path).expect("Failed to open file")
}

fn parse_args(args: Vec<String>) -> Vec<Argument> {
    let mut to_return: Vec<Argument> = Vec::new();
    let args = &args[1..];
    for arg in args {
        if arg.starts_with("--") {
            // Read the whole argument and pass as LongFlag (Including --)
            let x = arg.split("--").last().unwrap();
            to_return.push(Argument {
                kind: ArgumentType::LongFlag,
                value: String::from(x),
            })
        } else if arg.starts_with("-") {
            // Read each letter of the arg and pass as ShortFlag (Not Including -)
            for i in [1..arg.len()] {
                // 1 to ignore the first -
                to_return.push(Argument {
                    kind: ArgumentType::ShortFlag,
                    value: arg[i].to_string(),
                })
            }
        } else {
            to_return.push(Argument {
                kind: ArgumentType::Arg,
                value: arg.to_string(),
            })
        }
    }
    return to_return;
}
