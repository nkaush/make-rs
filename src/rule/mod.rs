mod state;
mod builder;
pub use state::*;
pub use builder::*;

#[derive(Default)]
pub struct Rule {
    target: String,
    commands: Vec<String>,
    rule_state: RuleState
}

impl Rule {
    pub(in crate::rule) fn new(target: String, commands: Vec<String>) -> Self {
        Self {
            target,
            commands,
            rule_state: RuleState::NotStarted
        }
    }

    pub fn get_target(&self) -> &str {
        &self.target
    }

    pub fn get_commands(&self) -> &Vec<String> {
        &self.commands
    }

    pub fn get_state(&self) -> RuleState {
        self.rule_state
    }

    pub fn set_state(&mut self, new_state: RuleState) {
        self.rule_state = new_state;
    }
}