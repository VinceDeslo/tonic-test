use clap::{App, Arg};
use log::{error, info};

mod commands;
use commands::{server, client};

fn main() {
    env_logger::init();

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
        info!("Matched on command: {}", c);
        match c {
            "server" => {
                info!("Running gRPC server");
                server().unwrap();
            }
            "client" => {
                info!("Running gRPC client");
                client().unwrap();
            },
            _ => error!("Command unavailable, try running a different command"),
        }
    }
}
