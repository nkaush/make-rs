use crate::{MakeError, ResourceDependencyGraph, Rule};
use std::io::{BufReader, BufRead};
use std::path::{Path, PathBuf};
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
    pub fn new(makefile: PathBuf) -> Result<Self, MakeError> {
        if Path::new(&makefile).exists() {
            Ok(Self { makefile: makefile })
        } else {
            Err(MakeError::MakefileDoesNotExist)
        }
    }

    pub fn parse(&mut self, goals: &mut Vec<String>) -> Result<ResourceDependencyGraph, MakeError> {
        let mut rdg: ResourceDependencyGraph = Default::default();
        rdg.add_rule("".into()); // Set the sentinel rule

        let rdr = match File::open(&self.makefile) {
            Ok(f) => BufReader::new(f),
            Err(_) => return Err(MakeError::MakefileDoesNotExist)
        };

        let mut first_rule: Option<String> = None;
        let mut current_rule: Option<&mut Rule> = None;
        let mut in_command_block: bool = false;

        for (line_num, l) in rdr.lines().enumerate() {
            let mut line = match l {
                Ok(l) => l.trim().to_string(),
                Err(_) => return Err(MakeError::MakefileParseError)
            };

            strip_comments(&mut line);
            if line.is_empty() {
                continue;
            }

            if line.starts_with('\t') { // This is a reciple line
                let rule = match current_rule.as_mut() {
                    None => return Err(MakeError::RecipeBeforeTarget),
                    Some(r) => r
                };

                // We have a rule, but we're in the command block...
                // So we're expecting an empty vector of commands
                if !in_command_block && !rule.get_commands().is_empty() {
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
                    rule.reset();
                }

                in_command_block = true;
                rule.add_command(line);
            } else if is_makefile_target(&line) {
                in_command_block = false;
                let (rule, deps) = match line.split_once(':') {
                    Some((r, d)) => (r.trim(), d.trim()),
                    None => return Err(MakeError::MissingSeparator)
                };

                rdg.add_rule(rule);
                if first_rule.is_none() {
                    first_rule = Some(rule.into());
                }

                for dep in deps.split_ascii_whitespace() {
                    rdg.add_rule(dep);
                    rdg.add_dependency(rule, dep).unwrap();
                }

                current_rule = rdg.get_rule_mut(rule);
            } else {
                return Err(MakeError::MissingSeparator);
            }
        }

        if goals.is_empty() {
            match first_rule {
                None => return Err(MakeError::NoTargets),
                Some(fr) => goals.push(fr)
            }
        }

        for goal in goals.iter() {
            match rdg.get_rule(goal) {
                Some(_) => rdg.add_dependency("", goal).unwrap(),
                None => return Err(MakeError::NoRuleToMakeTarget)
            }
        }

        Ok(rdg)
    }
}