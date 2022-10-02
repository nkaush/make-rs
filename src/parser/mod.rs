mod error;

use petgraph::graph::DiGraph;
use std::path::PathBuf;
use crate::Rule;

pub use error::*;

pub struct MakefileParser {
    makefile: PathBuf
}

impl MakefileParser {
    pub fn new(makefile: PathBuf) -> Result<Self, ParseError> {
        Ok(Self {
            makefile
        })
    }

    pub fn parse(&mut self) -> DiGraph<String, Rule> {
        let rdg: DiGraph<String, Rule> = DiGraph::new();

        rdg
    }
}