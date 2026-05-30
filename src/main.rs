#[allow(unused_imports)]
use std::io::{self, Write};

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

fn check_command(command: &str, res_args: &[String]) {
    if command == "echo" {
        for i in 0..res_args.len() {
            print!("{}", res_args[i]);
            if i != res_args.len() - 1 {
                print!(" ");
            }
        }
        println!("");
        io::stdout().flush().unwrap();
    } else {
        eprintln!("{}: command not found!", command);
    }
}