use std::path::PathBuf;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum MakeError {
    MakefileDoesNotExist,
    MakefileParseError,
    RecipeBeforeTarget,
    MissingSeparator(usize),
    NoTargets,
    NoRuleToMakeTarget,
    NoSuchFileOrDirectory(PathBuf)
}

impl fmt::Display for MakeError {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        todo!() 
    }
}

impl MakeError {
    pub fn reason(&self) -> String {
        format!("{:?}", self)
    }

    pub fn exit_code(&self) -> i32 {
        2
    }
}

impl Error for MakeError {
    
}