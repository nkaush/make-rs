use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum MakeError {
    MakefileDoesNotExist,
    MakefileParseError,
    RecipeBeforeTarget,
    MissingSeparator,
    NoTargets,
    NoRuleToMakeTarget
}

impl fmt::Display for MakeError {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        todo!() 
    }
}

impl MakeError {
    pub fn reason(&self) -> &str {
        todo!()
    }

    pub fn exit_code(&self) -> i32 {
        todo!()
    }
}

impl Error for MakeError {
    
}