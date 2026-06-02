#[allow(unused_imports)]
use std::fmt::format;
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

        let args: Vec<String> = command.trim().split(" ").map(|s| s.to_string()).collect();
        parser(args[0].as_str(), &args[1..]); 
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

fn parser(command: &str, res_args: &[String]) {
    if let Ok(cmmnd)= lexer(command) {
        match cmmnd {
            Commands::Builtin(BuiltInCommands::Exit) => {
                ()
            },
            Commands::Builtin(BuiltInCommands::Echo) => {
                for i in 0..res_args.len() {
                    print!("{}", res_args[i]);
                    if i != res_args.len() - 1 {
                        print!(" ");
                    }
                }
                println!("");
                io::stdout().flush().unwrap();
            },
            Commands::Builtin(BuiltInCommands::Type) => {
                if res_args.len() == 1 {
                    if let Ok(cmd) = lexer(res_args[0].as_str()) {
                        match cmd {
                            Commands::Builtin(_) => {
                                println!("{} is a shell builtin", res_args[0]);
                                return;
                            }
                            Commands::External(path) => {
                                println!("{} is {}", res_args[0], path);
                                return;
                            }
                        }
                    } else {
                        eprintln!("{}: not found", res_args[0]);
                        return;
                    } 
                }
            },
            Commands::Builtin(BuiltInCommands::Pwd) => {
                match env::current_dir() {
                    Ok(path) => {
                        println!("{}", path.display());
                        return;
                    },
                    Err(e) => {
                        eprintln!("Error getting current directory! {}", e.to_string());
                        return
                    }
                }
            },
            #[allow(unused_mut)]
            Commands::Builtin(BuiltInCommands::CurrentDirectory) => {
                let home = env::var("HOME").unwrap();               
                let mut new_current_dir: String = String::new();
                if res_args[0] == "~" {
                    new_current_dir = home;
                    change_current_directory(&new_current_dir);
                } else if res_args[0] == ".." {
                    let current_dir = env::current_dir().unwrap().display().to_string();
                    let vec: Vec<String> = current_dir.split("/").map(|s| s.to_string()).collect();
                    let new_current_directory_vec = &vec[..vec.len() - 1];
                    for i in new_current_directory_vec {
                        new_current_dir.push_str(i);     
                        new_current_dir.push('/');
                    }
                    change_current_directory(&new_current_dir);
                } else if res_args[0] == "." {
                    ()
                } else {
                    change_current_directory(&res_args[0]);
                }
                
            },
            Commands::External(_) => {
                let output = Command::new(command).args(&res_args[..]).output().expect("failed to execute program");
                let stdout = output.stdout;
                print!("{}", String::from_utf8_lossy(&stdout));
                io::stdout().flush().unwrap();
            }
        }
    } else {
        eprintln!("{}: command not found", command)
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