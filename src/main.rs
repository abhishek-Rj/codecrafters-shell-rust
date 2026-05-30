#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, path::Path, process::Command};

enum Commands {
    Echo,
    Type,
    Exit
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
        check_command(args[0].as_str(), &args[1..]); 
    }
}

fn parser(command: &str) -> Result<Commands, String> {
    if command == "echo" {
        Ok(Commands::Echo)
    } else if command == "type" {
        Ok(Commands::Type)
    } else if command == "exit" {
        Ok(Commands::Exit)
    } else {
        Err("error parsing the command".into())
    }
}

fn check_command(command: &str, res_args: &[String]) {
    if let Ok(command)= parser(command) {
        match command {
            Commands::Echo => {
                for i in 0..res_args.len() {
                    print!("{}", res_args[i]);
                    if i != res_args.len() - 1 {
                        print!(" ");
                    }
                }
                println!("");
                io::stdout().flush().unwrap();
            },
            Commands::Exit => {
                ()
            }
            Commands::Type => {
                if res_args.len() == 1 {
                    if let Ok(_) = parser(res_args[0].as_str()) {
                        println!("{} is a shell builtin", res_args[0]);
                    } 
                    match env::var("PATH") {
                        Ok(path) => {
                            let directories: Vec<String> = path.split(":").map(|s| s.to_string()).collect();
                            for dir in directories {
                                let path = format!("{}/{}", dir, res_args[0]);

                                if Path::new(&path).is_file() {
                                    println!("{} is {}", res_args[0], path);
                                    return;
                                }
                            }
                            eprintln!("{}: not found", res_args[0]);
                        },
                        Err(e) => eprintln!("Coudn't read PATH: {}", e),
                    }
                }
            }
        }
    } else {
        eprintln!("{}: command not found", command)
    }
}