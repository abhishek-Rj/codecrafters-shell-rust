pub enum BuiltInCommands {
    Exit,
    Echo,
    Type,
    Pwd,
    CurrentDirectory
}

pub enum Commands {
    Builtin(BuiltInCommands),
    External(String) 
}

pub enum CommandOutput {
    Stdout(String),
    Stderr(String),
}
