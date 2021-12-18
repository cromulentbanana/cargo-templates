use anyhow::Result; // Consider https://github.com/dtolnay/thiserror if we care what error type our functions return
use log;
use log::LevelFilter::{Debug, Error, Info, Trace};
use simple_logger::SimpleLogger;
use std::path::PathBuf;
use std::{
    fs,
    io::{self, Write},
};
use structopt::StructOpt;
// (Buf) Uncomment these lines to have the output buffered, this can provide
// better performance but is not always intuitive behaviour.
// use std::io::BufWriter;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "{{project-name}}",
    about = "A short description of this project.",
    version = concat!(env!("CARGO_PKG_VERSION"), concat!("_", env!("GIT_SHORT_HASH")))
)]
struct Options {
    /// Increase logging verbosity
    #[structopt(short, parse(from_occurrences))]
    verbosity: usize,

    /// Example optional boolean flag
    #[structopt(short, long)]
    some_flag: Option<bool>,

    /// Example filesystem path
    #[structopt(parse(from_os_str))]
    path: PathBuf,

    /// Example String-valued Argument
    #[structopt(default_value = "some value", env = "SOME_ENV_VAR")]
    pattern: String,
}

fn main() {
    let args = Options::from_args();
    let level = vec![Error, Info, Debug, Trace][(args.verbosity).min(3)];
    if args.verbosity > 0 {
        SimpleLogger::new()
            .with_level(level)
            .init()
            .expect("SimpleLogger instantiation failed.");
    }

    log::info!("Logging with level {}", level);

    let contents = fs::read_to_string(&args.path).expect("Could not read file.");
    let mut stdout = io::stdout();
    // (Buf) Wraps stdout in a buffer.
    // let mut stdout = BufWriter::new(stdout);

    for (line_no, line) in contents.lines().enumerate() {
        if line.contains(&args.pattern) {
            let _ = writeln!(stdout, "{}: {}", line_no + 1, line);
        }
    }
}
