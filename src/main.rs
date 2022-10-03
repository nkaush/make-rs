use make_rs::{Arguments, MakeError, MakefileParser};
use std::cmp::{max, min};
use std::{env, process};
use clap::Parser;

pub fn make() -> Result<(), MakeError> {
    let mut args: Arguments = Arguments::parse();

    if let Some(dir) = args.directory.as_ref() {
        if let Err(_) = env::set_current_dir(dir) {
            return Err(MakeError::NoSuchFileOrDirectory(dir.clone()));
        }
    }

    let mut parser = MakefileParser::new(&args.file)?;
    let rdg = parser.parse(&mut args.rules)?;

    let requested_threads: usize = max(1, args.jobs.into());
    let max_degree: usize = rdg.max_rule_degree();
    match min(max_degree, requested_threads) {
        1 => make_rs::sequential_make(rdg, args.get_configuration()),
        n if n > 1 => make_rs::parallel_make(rdg, args.get_configuration(), n),
        _ => unreachable!()
    }
}

fn main() {
    if let Err(e) = make() {
        let program_name = env::args().nth(0).unwrap();
        eprintln!("{}: {}", program_name, e.reason());
        process::exit(e.exit_code());
    }
}
