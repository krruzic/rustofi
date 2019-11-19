use std::fmt;

use subprocess::PopenError;

/// types of errors running rofi returns
#[derive(Debug, Clone)]
pub enum WindowErrorType {
    /// something went wrong with `Popen`
    PopenError
}


/// error returned whenever rofi itself errors out, this can only happen if `Popen` returns a bad exit
/// code for some reason
#[derive(Clone)]
pub struct WindowError {
    error: WindowErrorType,
    message: String
}

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
