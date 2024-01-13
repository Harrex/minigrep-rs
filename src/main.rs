use std::{env, io};
use minigrep::*;

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
