use crate::MakeError;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ParseError {
    MakefileDoesNotExist
}

impl fmt::Display for ParseError {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        todo!() 
    }
}

impl Error for ParseError {

}

impl MakeError for ParseError {
    fn reason(&self) -> &str { 
        todo!() 
    }

    fn exit_code(&self) -> i32 { 
        todo!() 
    }
}