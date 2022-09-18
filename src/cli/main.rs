use clap::{App, Arg};

mod commands;
use commands::client;

fn main() {
    let command = Arg::new("command")
        .short('c')
        .long("command")
        .value_name("COMMAND_NAME")
        .help("Provides a value for the wanted command.")
        .takes_value(true);

    let matches = App::new("Command CLI")
        .version("1.0")
        .author("Vincent Desloover <vincent.desloover.dev@gmail.com>")
        .about("CLI tool to run a gRPC command.")
        .arg(command)
        .get_matches();

    if let Some(c) = matches.value_of("command") {
        println!("Matched on command: {}", c);
        match c {
            "client" => client().unwrap(),
            _ => println!("Command unavailable, try running a different command"),
        }
    }
}
