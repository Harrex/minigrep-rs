use std::fs;
#[derive(Debug)]
pub enum ArgumentType {
    LongFlag,  // --whatever
    ShortFlag, // -w
    Arg,       // file.txt
}

#[derive(Debug)]
pub struct Argument {
    pub kind: ArgumentType,
    pub value: String,
}

#[derive(Debug)]
pub struct GrepOptions {
    pub case_sensitive: bool,
    pub filepath_string: String,
    pub grep_text: String,
}

impl GrepOptions {
    pub fn default() -> Self {
        GrepOptions {
            case_sensitive: true,
            filepath_string: String::from(""),
            grep_text: String::from(""),
        }
    }
}

pub const HELP_MESSAGE: &str = "\
    Help or somethin idk
";


pub fn grep_file_text(input: String, pattern: String) -> String {
    let mut output = String::new();

    for line in input.lines() {
        if line.contains(&pattern) {
            let x = line.replace(&pattern, &format!("\x1b[1m{}\x1b[0m", &pattern));
            output.push_str(&format!("{}\n", &x));
        }
    }

    return output;
}
pub fn grep_stdin(input: String, pattern: String) -> String {
    if input.contains(&pattern) {
        let x = input.replace(&pattern, &format!("\x1b[1m{}\x1b[0m", &pattern));
        return x;
    }
    String::from("")
}

pub fn get_file_text(filepath: String) -> String {
    let path = fs::canonicalize(filepath).expect("Couldn't get file path");
    fs::read_to_string(path).expect("Failed to open file")
}

pub fn parse_args(args: Vec<String>) -> Vec<Argument> {
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
