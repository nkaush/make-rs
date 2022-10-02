mod state;

pub use state::*;

#[derive(Default)]
pub struct Rule {
    target: String,
    commands: Vec<String>,
    rule_state: RuleState
}

impl Rule {
    pub fn new(target: String) -> Self {
        Self {
            target,
            commands: Vec::new(),
            rule_state: RuleState::NotStarted
        }
    }

    pub fn add_command(&mut self, command: String) {
        self.commands.push(command);
    }
}