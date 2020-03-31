extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
    let app = App::new("Kvs - InMemory Key value store")
        .version("1.0")
        .author("kanapuli")
        .about("A superfast InMemory key value store")
        .subcommand(
            SubCommand::with_name("get")
                .about("get - retrieves a value for the given key")
                .arg(
                    Arg::with_name("KEY")
                        .help("string value of the key")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("rm - Deletes the key and its value from the kvs store")
                .arg(
                    Arg::with_name("KEY")
                        .help("string value of the key")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("set - stores a value for the given key")
                .arg(
                    Arg::with_name("KEY")
                        .help("string value of the key")
                        .required(true),
                )
                .arg(Arg::with_name("VALUE").help("string value").required(true)),
        )
        .get_matches();
}
