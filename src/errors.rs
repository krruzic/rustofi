use std::fmt;

use subprocess::PopenError;

impl From<PopenError> for WindowError {
    fn from(error: PopenError) -> Self {
        WindowError {
            error: WindowErrorType::PopenError,
            message: format!("{:?}", error)
        }
    }
}

impl fmt::Debug for WindowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut error_string = String::new();
        match self.error {
            WindowErrorType::PopenError => error_string.push_str("PopenError")
        }
        write!(f, "[{}]: {}", error_string, self.message)
    }
}

#[derive(Debug, Clone)]
pub enum WindowErrorType {
    PopenError
}

#[derive(Clone)]
pub struct WindowError {
    error: WindowErrorType,
    message: String
}