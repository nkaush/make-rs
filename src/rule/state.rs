#[derive(Default, Copy, Clone)]
pub enum RuleState {
    #[default]
    NotStarted,
    InProgress,
    Completed,
    Failed
}