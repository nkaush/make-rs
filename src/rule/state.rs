#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RuleState {
    #[default]
    NotStarted,
    InProgress,
    Completed,
    Failed
}