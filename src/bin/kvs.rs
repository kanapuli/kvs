extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};
use kvs::Result;
use std::process::exit;

fn main() -> Result<()> {
    let app = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::VersionlessSubcommands)
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

    match app.subcommand() {
        ("get", Some(_args)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("set", Some(_args)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        ("rm", Some(_args)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        _ => unreachable!(),
    };
    Ok(())
}
