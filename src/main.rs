use make_rs::{Arguments, MakeError, MakefileParser, Rule, ResourceDependencyGraph};
use petgraph::{Graph, algo::is_cyclic_directed};
use std::{env, process, thread};
use std::sync::{Arc, Mutex};
use std::cmp::{max, min};
use tsq::ThreadSafeQueue;
use clap::Parser;

type RuleQueue = ThreadSafeQueue<Option<Rule>>;

fn make_worker(rdg: Arc<Mutex<ResourceDependencyGraph>>, q: Arc<RuleQueue>) {
    
}

pub fn make() -> Result<(), MakeError> {
    let mut args: Arguments = Arguments::parse();    
    let mut parser = MakefileParser::new(args.file)?;
    let rdg = parser.parse(&mut args.rules)?;

    let max_degree = rdg.max_rule_degree();
    let num_threads: usize = min(max_degree, max(1, args.jobs.into()));

    let queue: Arc<RuleQueue> = Arc::new(ThreadSafeQueue::new());
    let rdg: Arc<Mutex<ResourceDependencyGraph>> = Arc::new(Mutex::new(rdg));
    (0..num_threads)
        .map(|_| {
            let qc: Arc<RuleQueue> = Arc::clone(&queue);
            let rdgc = Arc::clone(&rdg);
            thread::spawn(move || make_worker(rdgc, qc))
        })
        .for_each(|h| h.join().unwrap());

    Ok(())
}

fn main() {
    if let Err(e) = make() {
        let program_name = env::args().nth(0).unwrap();
        eprintln!("{}: {}", program_name, e.reason());
        process::exit(e.exit_code());
    }
}
