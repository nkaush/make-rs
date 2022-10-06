use crate::Rule;

pub struct RuleBuilder {
    target: String,
    commands: Vec<String>
}

impl RuleBuilder {
    pub fn new(target: &str) -> Self {
        Self {
            target: target.into(),
            commands: Vec::new()
        }
    }

    pub fn reset(&mut self) {
        self.commands.clear();
    }

    pub fn add_command(&mut self, command: String) {
        self.commands.push(command);
    }

    pub fn get_commands(&self) -> &Vec<String> {
        &self.commands
    }

    pub fn to_rule(self) -> Rule {
        Rule::new(self.target, self.commands)
    }
}