use std::fs;
use std::io::Write;
use std::path::Path;

const OPERATOR_SYMBOLS: [&str; 7] = ["1>", ">", "2>", ">>", "1>>", "2>>", "|"];

pub fn if_operator(token: &[String]) -> (bool, Option<&str>, Option<usize>) {
    for i in OPERATOR_SYMBOLS {
        if token.contains(&i.to_string()) {
            let index = token.iter().position(|x| x == &i.to_string()).unwrap();
            return (true, Some(i), Some(index)) 
        }
    }
    (false, None, None)
}

pub fn operator(operator: &str, args: &[String], input: String) {
    match operator {
        "1>" | ">" | "2>" => {
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

        ">>" | "1>>" | "2>>" => {
            let path = &args[0];
            let mut file = fs::OpenOptions::new().append(true).create(true).open(path).unwrap();
            match file.write_all(input.as_bytes()) {
                Ok(_) => {()},
                Err(error) => {
                    eprintln!("{error}, cannot append to the file");
                }
            }
        },

        _ => {
            ()
        }
    }
}
