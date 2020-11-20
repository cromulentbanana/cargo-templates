use anyhow::Result; // Consider https://github.com/dtolnay/thiserror if we care what error type our functions return
use clap::Parser;
use log;
use log::LevelFilter::{Debug, Error, Info, Trace};
use simple_logger::SimpleLogger;
use std::path::PathBuf;
use std::{
    fs,
    io::{self, Write},
};
// (Buf) Uncomment these lines to have the output buffered, this can provide
// better performance but is not always intuitive behaviour.
// use std::io::BufWriter;

mod webapp;

#[derive(Parser, Debug)]
#[clap(
    name = "{{project-name}}",
    about = "A short description of this project.",
    version = concat!(env!("CARGO_PKG_VERSION"), concat!("_", env!("GIT_SHORT_HASH")))
)]
struct Options {
    /// Suppress non-error messages
    #[structopt(short)]
    quiet: bool,

    /// Increase logging verbosity
    #[clap(short, parse(from_occurrences))]
    verbosity: usize,

    /// Example optional boolean flag
    #[clap(short, long)]
    some_flag: Option<bool>,

    /// Example filesystem path
    #[clap(parse(from_os_str))]
    path: PathBuf,

    /// Webserver Port Number
    #[clap(long, default_value = "8080", env = "BIND_PORT")]
    webserver_port: u16,

    /// Example String-valued Argument
    #[clap(default_value = "some value", env = "SOME_ENV_VAR")]
    pattern: String,

    /// Webserver IP Address
    #[clap(long, default_value = "::1", env = "BIND_ADDRESS")]
    webserver_bind_address: std::net::IpAddr,
}

fn main() {
    let args = Options::from_args();
    let level = match args.quiet {
        true => Error,
        false => vec![Info, Debug, Trace][(args.verbosity).min(2)],
    };

    SimpleLogger::new()
        .with_level(level)
        .init()
        .expect("SimpleLogger instantiation failed.");

    log::info!("Logging with level {}", level);

    webapp::main(args.webserver_bind_address, args.webserver_port).unwrap();
}
