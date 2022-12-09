use std::fmt;
use std::error::Error;
use std::fmt::Display;

// ovs error definition
#[derive(Debug)]
pub struct OvsError{
    pub error_type: OvsErrorType,
    pub error_message: String,
    pub error_detail: String
}

#[derive(Debug)]
pub enum OvsErrorType{
    ConnectionError,
    InvalidResponse,
    InvalidResponseJson,
    UnexpectedResponse,
    InconsistentInstruction,
    QueryError
}

impl OvsError{
    pub fn new(t: OvsErrorType, message: &str) -> OvsError{
        OvsError{
            error_type: t,
            error_message : message.to_string(),
            error_detail : "".to_string()
        }
    }

    pub fn detail(mut self, detail: &str) -> Self{
        self.error_detail = detail.to_string();
        self
    }
}

impl Error for OvsError{
    fn description(&self) -> &str{
        "OvsError"
    }
}

impl Display for OvsError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        if self.error_detail == "" {
            write!(f, "[OvsError]{}", self.error_message)
        }
        else{
            write!(f, "[OvsError]{}\n  ->(detail){}", self.error_message, self.error_detail)
        }
    }
}
