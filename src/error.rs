pub trait MakeError {
    fn reason(&self) -> &str;
    fn exit_code(&self) -> i32;
}