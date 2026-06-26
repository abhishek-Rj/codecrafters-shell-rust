mod command;
mod executor;
mod redirection;

use command::CommandOutput;
use executor::parser;
use redirection::{if_operator, operator};
use std::io::{self, Write};

pub fn run() {
    // TODO: Uncomment the code below to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
    
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        if command.trim().is_empty() {
            continue;
        }

        if command.trim() == String::from("exit") {
            break;
        }
        let mut current: String =  String::new();
        let mut token: Vec<String> = Vec::new();
        let mut is_single_quotes: bool = false;
        let mut is_double_quotes: bool = false;
        let mut is_backslash: bool = false;

        for i in command.chars() {
            match i {
                '\\' => {
                    if is_backslash {
                        current.push(i);                        
                        is_backslash = !is_backslash;
                    } else if is_single_quotes {
                        current.push(i);
                    } else {
                        is_backslash = !is_backslash;
                    }
                }
                '\'' => {
                    if is_backslash {
                        current.push(i);
                        is_backslash = !is_backslash;
                    } else if is_double_quotes{
                        current.push(i);
                    } else {
                        is_single_quotes = !is_single_quotes;
                    }
                }
                '\"' => {
                    if is_backslash {
                        current.push(i);
                        is_backslash = !is_backslash;
                    } else if is_single_quotes {
                        current.push(i);
                    } else {
                        is_double_quotes = !is_double_quotes;
                    }
                }
                ' ' if !is_single_quotes && !is_double_quotes && !is_backslash => {
                    if !current.is_empty() {
                        token.push(std::mem::take(&mut current));
                    }
                }
                ' ' if is_backslash => {
                    current.push(i);
                    is_backslash = !is_backslash;
                }
                _ => {
                    current.push(i);     
                    if is_backslash {
                        is_backslash = !is_backslash;
                    }
                }
            }
        }

        if !current.is_empty() {
            token.push(current.trim_end().to_string());
        }

        let (operator_bool, in_use_operator, index_position) = if_operator(&token[1..]);
        
        if operator_bool {
            let in_use_operator = in_use_operator.unwrap();
            let index_position = index_position.unwrap();
            let next_args = &token[2 + index_position..];
            let (stdout, stderr) = parser(token[0].as_str(), &token[1..=index_position]); 
            if in_use_operator == ">" || in_use_operator == "1>" || in_use_operator == ">>" || in_use_operator == "1>>" {
                let buf = match stdout {
                    Some(CommandOutput::Stdout(stdout)) => stdout,
                    None => String::new(),
                    _ => unreachable!()
                };
                operator(in_use_operator, next_args, buf);
                if let Some(CommandOutput::Stderr(buf)) = stderr {
                    if !buf.is_empty() {
                        eprint!("{buf}");
                    }
                }
            } else if in_use_operator == "2>" || in_use_operator == "2>>" {
                if let Some(CommandOutput::Stdout(buf)) = stdout {
                    if !buf.is_empty() {
                        eprint!("{buf}");
                    }
                }
                let buf = match stderr {
                    Some(CommandOutput::Stderr(stderr)) => stderr,
                    None => String::new(),
                    _ => unreachable!()
                };
                operator(in_use_operator, next_args, buf);
            }  
        } else {
            let (stdout, stderr) = parser(token[0].as_str(), &token[1..]); 
            if let Some(CommandOutput::Stdout(buf)) = stdout {
                if !buf.is_empty() {
                    print!("{buf}");
                }
            }
            if let Some(CommandOutput::Stderr(buf)) = stderr {
                if !buf.is_empty() {
                    eprint!("{buf}");
                }
            }
        }

    }
}
