use clap::{Arg, Command};
use std::error::Error;

pub fn cli() -> Command {
    Command::new("subcommand1")
        .about("Subcommand 1")
        .arg(Arg::new("path").value_name("PATH").default_value("."))
        .after_help("After subcommand1's help")
}

pub fn execute() -> Result<(), Box<dyn Error>> {
    let command = cli();
    let matches = command.get_matches();
    println!("In Subcommand 1");
    dbg!(matches);
    Ok(())
}
