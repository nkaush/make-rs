mod dependency_graph;
pub use dependency_graph::*;

use crate::{MakeError, RuleBuilder};
use std::io::{BufReader, BufRead};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs::File;

pub struct MakefileParser {
    makefile: PathBuf
}

fn strip_comments(s: &mut String) {
    let mut result: Option<&str> = None;
    for (idx, _m) in s.match_indices('#') {
        let prefix = &s[..idx];
        let qc_single = prefix.chars().filter(|c| *c == '\'').count();
        let qc_double = prefix.chars().filter(|c| *c == '"').count();
        
        if qc_single % 2 == 0 && qc_double % 2 == 0 {
            result = Some(prefix);
            break;
        }
    }

    if let Some(r) = result {
        *s = r.to_string();
    }
}

fn is_makefile_target(s: &str) -> bool {
    s.starts_with(char::is_alphanumeric) || s.starts_with(&['.', '/'])
}

impl MakefileParser {
    pub fn new(makefile: &PathBuf) -> Result<Self, MakeError> {
        if Path::new(&makefile).exists() {
            Ok(Self { makefile: makefile.into() })
        } else {
            Err(MakeError::MakefileDoesNotExist)
        }
    }

    pub fn parse(&mut self, goals: &mut Vec<String>) -> Result<DependencyGraph, MakeError> {
        let rdr = match File::open(&self.makefile) {
            Ok(f) => BufReader::new(f),
            Err(_) => return Err(MakeError::MakefileDoesNotExist)
        };

        let mut rules: HashMap<String, RuleBuilder> = HashMap::new();
        let mut dependencies: HashMap<String, Vec<String>> = HashMap::new();
        let mut first_rule: Option<String> = None;
        let mut current_rule: Option<&mut RuleBuilder> = None;
        let mut in_command_block: bool = false;

        for (line_num, l) in rdr.lines().enumerate() {
            let (mut line, untrimmed) = match l {
                Ok(l) => (l.trim().to_string(), l),
                Err(_) => return Err(MakeError::MakefileParseError)
            };

            strip_comments(&mut line);
            if line.is_empty() { // Skip empty lines
                continue;
            }

            if untrimmed.starts_with('\t') { // This is a reciple line
                let rule_builder = match current_rule.as_mut() {
                    None => return Err(MakeError::RecipeBeforeTarget),
                    Some(r) => r
                };

                // We have a rule, but we're in the command block...
                // So we're expecting an empty vector of commands
                if !in_command_block && !rule_builder.get_commands().is_empty() {
                    // WARNING: This rule's commands are being redefined
                    // fprintf(stderr,
                    //         "%s:%zu: warning: overriding recipe for target "
                    //         "'%s'\n",
                    //         makeFileName, line_number, curr_rule->target);
                    // fprintf(stderr,
                    //         "%s:%zu: warning: ignoring old recipe for target "
                    //         "'%s'\n",
                    //         makeFileName, (size_t)curr_rule->state,
                    //         curr_rule->target);
                    rule_builder.reset();
                }

                in_command_block = true;
                rule_builder.add_command(line);
            } else if is_makefile_target(&line) { // Start of a rule definition
                in_command_block = false;
                let (rule, deps) = match line.split_once(':') {
                    Some((r, d)) => (r.trim(), d.trim()),
                    None => return Err(MakeError::MissingSeparator(line_num))
                };

                if !rules.contains_key(rule) {
                    rules.insert(rule.into(), RuleBuilder::new(rule));
                    dependencies.insert(rule.into(), Vec::new());
                }

                // define the first rule in case make is run without rules
                if first_rule.is_none() {
                    first_rule = Some(rule.into());
                }

                for dep in deps.split_ascii_whitespace() {
                    // Add dep as a rule that needs to be defined in case the 
                    // parser has not yet seen it defined
                    if !rules.contains_key(dep) {
                        rules.insert(dep.into(), RuleBuilder::new(dep));
                        dependencies.insert(dep.into(), Vec::new());
                    }

                    // Add dep as a one of rule's dependencies
                    dependencies.get_mut(rule)
                        .unwrap()
                        .push(dep.into());
                }

                current_rule = rules.get_mut(rule);
            } else {
                return Err(MakeError::MissingSeparator(line_num));
            }
        }

        if goals.is_empty() {
            match first_rule {
                None => return Err(MakeError::NoTargets),
                Some(fr) => goals.push(fr)
            }
        }

        // Add sentinel rule to add an entrypoint for all requested rules
        let sentinel: String = "".into();
        rules.insert(sentinel.clone(), RuleBuilder::new(&sentinel));
        dependencies.insert(sentinel.clone(), Vec::new());
        let sentinel_deps = dependencies.get_mut(&sentinel).unwrap();

        for goal in goals.iter() {
            match rules.get(goal) {
                Some(_) => sentinel_deps.push(goal.into()),
                None => return Err(MakeError::NoRuleToMakeTarget)
            }
        }

        // Convert all of the RuleBuilders into Rules
        let rules = rules.into_iter()
            .map(|(k, v)| (k, v.to_rule()))
            .collect();

        Ok(DependencyGraph::new(rules, dependencies))
    }
}