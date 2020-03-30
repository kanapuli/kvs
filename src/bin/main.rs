extern crate clap;

use clap::{App, Arg};

fn main() {
    let app = App::new("Kvs - InMemory Key value store")
        .version("1.0")
        .author("kanapuli")
        .about("A superfast InMemory key value store")
        .arg(
            Arg::with_name("get")
                .long("get")
                .help("get - retrieves a value for the given key")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("set")
                .long("set")
                .help("set - sets the value for a given key")
                .takes_value(true),
        )
        .get_matches();
}
