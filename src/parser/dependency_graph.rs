use std::collections::HashMap;
use crate::Rule;

#[derive(Default)]
pub struct DependencyGraph {
    rules: HashMap<String, Rule>,
    dependencies: HashMap<String, Vec<String>>
}

impl DependencyGraph {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_rule(&mut self, rule_name: &str) {
        self.rules.insert(rule_name.into(), Rule::new(rule_name));
        self.dependencies.insert(rule_name.into(), Vec::new());
    }

    pub fn add_dependency(&mut self, parent: &str, child: &str) -> Result<(), ()> {
        if self.rules.contains_key(parent) && self.rules.contains_key(child) {
            Ok(self.dependencies.get_mut(parent).unwrap().push(child.into()))
        } else {
            Err(())
        }
    }

    pub fn get_rule_mut(&mut self, rule_name: &str) -> Option<&mut Rule> {
        self.rules.get_mut(rule_name)
    }

    pub fn get_rule(&self, rule_name: &str) -> Option<&Rule> {
        self.rules.get(rule_name)
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
}