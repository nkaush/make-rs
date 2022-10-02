use make_rs::{Arguments, MakeError, MakefileParser, ParseError, Rule};
use std::{env, process, thread};
use tsq::ThreadSafeQueue;
use std::error::Error;
use std::sync::Arc;
use std::cmp::min;
use clap::Parser;

fn make() -> Result<(), impl Error + MakeError> {
    let args: Arguments = Arguments::parse();    
    let mut parser = MakefileParser::new(args.file)?;
    let rdg = parser.parse();

    let queue: Arc<ThreadSafeQueue<Rule>>;

    Ok::<(), ParseError>(())
}

fn main() {
    if let Err(e) = make() {
        let program_name = env::args().nth(0).unwrap();
        eprintln!("{}: {}", program_name, e.reason());
        process::exit(e.exit_code());
    }
}
