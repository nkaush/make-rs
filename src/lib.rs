mod arguments;
mod parser;
mod graph;
mod error;
mod rule;

pub use arguments::*;
pub use parser::*;
pub use graph::*;
pub use error::*;
pub use rule::*;

use std::sync::{Arc, Mutex};
use tsq::ThreadSafeQueue;
use std::thread;

type RuleQueue = ThreadSafeQueue<Option<Rule>>;

pub fn sequential_make(rdg: ResourceDependencyGraph) -> Result<(), MakeError> {
    Ok(())
}

pub fn parallel_make(rdg: ResourceDependencyGraph, num_threads: usize) -> Result<(), MakeError> {
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

fn make_worker(rdg: Arc<Mutex<ResourceDependencyGraph>>, q: Arc<RuleQueue>) {
    
}