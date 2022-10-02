#[derive(Default)]

pub enum RuleState {
    #[default]
    NotStarted,
    InProgress,
    Completed,
    Failed
}