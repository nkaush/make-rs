use std::collections::{HashMap, HashSet, hash_map::Values};
use thiserror::Error;
use crate::Rule;

#[derive(Debug, Error)]
#[error("Invalid target '{target}'")]
pub struct InvalidTargetError {
    target: String
}

impl InvalidTargetError {
    fn from_target(target: &str) -> Self {
        Self { target: target.into() }
    }
}

pub struct DependencyGraph {
    rules: HashMap<String, Rule>,
    dependencies: HashMap<String, Vec<String>>
}

impl DependencyGraph {
    pub(in crate::parser) fn new(rules: HashMap<String, Rule>, dependencies: HashMap<String, Vec<String>>) -> Self {
        Self {
            rules,
            dependencies
        }
    }

    // pub fn add_rule(&mut self, rule: Rule) {
    //     let target: String = rule.get_target().into();
    //     self.rules.insert(target.clone(), rule);
    //     self.dependencies.insert(target, Vec::new());
    // }

    // pub fn add_dependency(&mut self, parent: &str, child: &str) -> Result<(), InvalidTargetError> {
    //     let contains_parent = self.rules.contains_key(parent);
    //     let contains_child = self.rules.contains_key(child);
    //     if contains_parent && contains_child {
    //         Ok(self.dependencies.get_mut(parent).unwrap().push(child.into()))
    //     } else if !contains_parent {
    //         Err(InvalidTargetError::from_target(parent))
    //     } else {
    //         Err(InvalidTargetError::from_target(child))
    //     }
    // }

    pub fn get_rule_mut(&mut self, rule_name: &str) -> Option<&mut Rule> {
        self.rules.get_mut(rule_name)
    }

    pub fn get_rule(&self, rule_name: &str) -> Option<&Rule> {
        self.rules.get(rule_name)
    }

    pub fn rules(&self) -> Values<String, Rule> {
        self.rules.values()
    }

    pub fn get_dependencies(&self, rule_name: &str) -> Option<&Vec<String>> {
        self.dependencies.get(rule_name)
    }

    pub fn max_rule_degree(&self) -> usize {
        self.dependencies
            .values()
            .map(Vec::len)
            .max()
            .unwrap()
    }

    fn is_cyclic_helper(&self, ordered_nodes: &mut Vec<&Rule>, discovered: &mut HashSet<&Rule>, finished: &mut HashSet<&Rule>, target: &str) -> bool {
        false
    }

    pub fn is_cyclic(&self, target: &str) -> bool {
        let mut ordered_nodes = Vec::new();
        let mut discovered = HashSet::new();
        let mut finished = HashSet::new();

        self.is_cyclic_helper(&mut ordered_nodes, &mut discovered, &mut finished, target)
    }
}