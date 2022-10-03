mod state;
pub use state::*;

#[derive(Default)]
pub struct Rule {
    target: String,
    commands: Vec<String>,
    rule_state: RuleState
}

impl Rule {
    pub fn new(target: &str) -> Self {
        Self {
            target: target.into(),
            commands: Vec::new(),
            rule_state: RuleState::NotStarted
        }
    }

    pub fn reset(&mut self) {
        self.commands.clear();
    }

    pub fn add_command(&mut self, command: String) {
        self.commands.push(command);
    }

    pub fn get_target(&self) -> &String {
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