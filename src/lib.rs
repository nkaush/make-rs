mod arguments;
mod parser;
mod error;
mod rule;

pub use arguments::*;
pub use parser::*;
pub use error::*;
pub use rule::*;

use std::sync::{Arc, Mutex};
use tsq::ThreadSafeQueue;
use std::thread;

type RuleQueue = ThreadSafeQueue<Option<Rule>>;

pub fn sequential_make(rdg: DependencyGraph, mcfg: MakeConfiguration) -> Result<(), MakeError> {
    Ok(())
}

pub fn parallel_make(rdg: DependencyGraph, mcfg: MakeConfiguration, num_threads: usize) -> Result<(), MakeError> {
    let queue: Arc<RuleQueue> = Arc::new(ThreadSafeQueue::new());
    let rdg: Arc<Mutex<DependencyGraph>> = Arc::new(Mutex::new(rdg));
    (0..num_threads)
        .map(|_| {
            let qc: Arc<RuleQueue> = Arc::clone(&queue);
            let rdgc = Arc::clone(&rdg);
            thread::spawn(move || make_worker(rdgc, qc))
        })
        .for_each(|h| h.join().unwrap());

    Ok(())
}

fn make_worker(rdg: Arc<Mutex<DependencyGraph>>, q: Arc<RuleQueue>) {
    
}