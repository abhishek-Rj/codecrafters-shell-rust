#[allow(unused_imports)]
use std::fmt::format;
use std::fs;
use std::io::{self, Write};
use std::process::Command;
use std::{env, path::Path};
use std::os::unix::fs::PermissionsExt;
enum BuiltInCommands {
    Exit,
    Echo,
    Type,
    Pwd,
    CurrentDirectory
}

enum Commands {
    Builtin(BuiltInCommands),
    External(String) 
}

enum RedirectOperator {
    Stdout(String),
    Stderr(String),
}

#[allow(unused)]
enum Operator {
    Redirect(RedirectOperator),
    Pipe,
    None
}

const OPERATOR_SYMBOLS: [&str; 3] = ["1>", ">", "|"];

fn if_operator(token: &[String]) -> (bool, Option<&str>, Option<usize>) {
    for i in OPERATOR_SYMBOLS {
        if token.contains(&i.to_string()) {
            let index = token.iter().position(|x| x == &i.to_string()).unwrap();
            return (true, Some(i), Some(index)) 
        }
    }
    (false, None, None)
}

fn operator(operator: &str, args: &[String], input: String) {
    match operator {
        "1>" | ">" => {
            let path = &args[0];
            let parent = Path::new(path).parent().unwrap();
            fs::create_dir_all(parent).unwrap();
            match fs::write(path, input) {
                Ok(_) => {()}
                Err(error) => {
                    eprintln!("{error}, Unable to write stdout in corresponding file");
                }
            } 
        },

        _ => {
            ()
        }
    }
}

fn main() {
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
            if let Some(Operator::Redirect(RedirectOperator::Stdout(buf))) = stdout {
                operator(in_use_operator, next_args, buf);
            }
            if let Some(Operator::Redirect(RedirectOperator::Stderr(buf))) = stderr {
                if !buf.is_empty() {
                    eprint!("{buf}");
                }
            }
        } else {
            let (stdout, stderr) = parser(token[0].as_str(), &token[1..]); 
            if let Some(Operator::Redirect(RedirectOperator::Stdout(buf))) = stdout {
                print!("{buf}");
            }
            if let Some(Operator::Redirect(RedirectOperator::Stderr(buf))) = stderr {
                if !buf.is_empty() {
                    eprint!("{buf}");
                }
            }
        }

    }
}

fn lexer(command: &str) -> Result<Commands, String> {
    if command == "echo" {
        Ok(Commands::Builtin(BuiltInCommands::Echo))
    } else if command == "type" {
        Ok(Commands::Builtin(BuiltInCommands::Type))
    } else if command == "exit" {
        Ok(Commands::Builtin(BuiltInCommands::Exit))
    } else if command == "pwd" {
        Ok(Commands::Builtin(BuiltInCommands::Pwd))
    } else if command == "cd" {
        Ok(Commands::Builtin(BuiltInCommands::CurrentDirectory))
    } else {
        match env::var("PATH") {
            Ok(path) => {
                let directories: Vec<String> = path.split(":").map(|s| s.to_string()).collect();
                for dir in directories {
                    let path = format!("{}/{}", dir, command);

                    if if_file_exist_and_executable(&path) {
                        return Ok(Commands::External(path));
                    }
                }
                return Err("Coudn't parse the keyword".into());
            },
            Err(_) => {
                return Err("Coudn't parse the keyword".into());
            },
        }
    }
}

fn parser(command: &str, res_args: &[String]) -> (Option<Operator>, Option<Operator>) {
    let mut stdout_buffer = String::new();
    if let Ok(cmmnd)= lexer(command) {
        match cmmnd {
            Commands::Builtin(BuiltInCommands::Exit) => {
                (None, None)
            },

            Commands::Builtin(BuiltInCommands::Echo) => {
                //println!("{res_args:?}");
                for i in 0..res_args.len() {
                    stdout_buffer.push_str(res_args[i].as_str());
                    if i != res_args.len() - 1 {
                       stdout_buffer.push(' ');
                    }
                }
                stdout_buffer.push_str("\n");
                (Some(Operator::Redirect(RedirectOperator::Stdout(stdout_buffer))), None)
            },
            
            Commands::Builtin(BuiltInCommands::Type) => {
                //println!("{res_args:?}");
                if res_args.len() == 1 {
                    if let Ok(cmd) = lexer(res_args[0].as_str()) {
                        match cmd {
                            Commands::Builtin(_) => {
                                (Some(Operator::Redirect(RedirectOperator::Stdout(format!("{} is a shell builtin\n", res_args[0])))), None)
                            }
                            Commands::External(path) => {
                                (Some(Operator::Redirect(RedirectOperator::Stdout(format!("{} is {}\n", res_args[0], path)))), None)
                            }
                        }
                    } else {
                        (None, Some(Operator::Redirect(RedirectOperator::Stderr(format!("{}: not found\n", res_args[0])))))
                    } 
                } else {
                    (None, Some(Operator::Redirect(RedirectOperator::Stderr(format!("More than one argument for type command not allowd\n")))))
                }
            },
            
            Commands::Builtin(BuiltInCommands::Pwd) => {
                match env::current_dir() {
                    Ok(path) => {
                        (Some(Operator::Redirect(RedirectOperator::Stdout(format!("{}\n", path.display())))), None)
                    },
                    Err(e) => {
                        (None, Some(Operator::Redirect(RedirectOperator::Stderr(format!("Error getting current directory! {}\n", e.to_string())))))
                    }
                }
            },

            Commands::Builtin(BuiltInCommands::CurrentDirectory) => {
                let home = env::var("HOME").unwrap();               
                let mut new_current_dir: String = String::new();
                if res_args[0] == "~" {
                    new_current_dir = home;
                    change_current_directory(&new_current_dir);
                    (None, None)
                } else if res_args[0] == ".." {
                    let current_dir = env::current_dir().unwrap().display().to_string();
                    let vec: Vec<String> = current_dir.split("/").map(|s| s.to_string()).collect();
                    let new_current_directory_vec = &vec[..vec.len() - 1];
                    for i in new_current_directory_vec {
                        new_current_dir.push_str(i);     
                        new_current_dir.push('/');
                    }
                    change_current_directory(&new_current_dir);
                    (None, None)
                } else if res_args[0] == "." {
                    (None, None)
                } else {
                    change_current_directory(&res_args[0]);
                    (None, None)
                }
            },
            
            Commands::External(_) => {
                let output = Command::new(command).args(&res_args[..]).output().expect("failed to execute program");
                let stdout = output.stdout;
                let stderr = output.stderr;
                (Some(Operator::Redirect(RedirectOperator::Stdout(format!("{}", String::from_utf8_lossy(&stdout))))), Some(Operator::Redirect(RedirectOperator::Stderr(format!("{}", String::from_utf8_lossy(&stderr))))))
            }
        }
    } else {
        (None, None)
        //eprintln!("{}: command not found", command)
    }
}

fn change_current_directory(path: &String) {
    match env::set_current_dir(path) {
        Ok(_) => {}
        Err(_) => {eprintln!("cd: {}: No such file or directory", path);}
    }
}

fn if_file_exist_and_executable(path: &String) -> bool {
    if Path::new(path).is_file() {
        let metadata = std::fs::metadata(path).unwrap();
        if metadata.permissions().mode() & 0o111 != 0 {
            return true
        }
    }
    false
}