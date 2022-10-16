use std::collections::{HashMap, HashSet, hash_map::Values, VecDeque};
use crate::Rule;

#[derive(Debug)]
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

    pub fn is_cyclic(&self, target: &str, ordered: &mut Vec<String>) -> bool {
        let mut visited: HashSet<&String> = HashSet::new();
        let mut stack = VecDeque::new();
        stack.push_back(target);

        while let Some(curr) = stack.pop_back() {
            println!("visiting {curr:?}");
            ordered.push(curr.into());
            for dep in self.dependencies.get(curr).unwrap() {
                println!("at dep {dep:?}");
                if !visited.contains(dep) {
                    visited.insert(dep);
                    stack.push_back(dep);
                } else {
                    ordered.push(dep.into());
                    return true;
                }
            }
        }

        false
    }
}