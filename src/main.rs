use std::process;
extern crate clap;
#[macro_use] extern crate log;

const VERSION: &str = concat!("v", clap::crate_version!());
const LONG_ABOUT: &str = r#"
Bulls and Cows is an old game played with pencil and paper that was later
implemented using computers.

Read the instructions at: https://rosettacode.org/wiki/Bulls_and_cows
"#;

fn main() {
    env_logger::init();
    clap::App::new(clap::crate_name!())
        .author(clap::crate_authors!())
        .version(VERSION)
        .about(clap::crate_description!())
        .long_about(LONG_ABOUT)
        .get_matches();
    info!("Starting the game!");
    if let Err(e) = bulls_and_cows::run() {
        eprintln!("{}", e);
        eprintln!("Sorry, but you lost!");
        process::exit(2);
    }
}
