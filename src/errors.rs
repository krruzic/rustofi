use std::fmt;
use subprocess::PopenError;

impl From<PopenError> for WindowError {
    fn from(error: PopenError) -> Self {
        WindowError {
            error: WindowErrorType::PopenError,
            message: format!("{:?}", error),
        }
    }
}

impl fmt::Debug for WindowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut error_string = String::new();
        match self.error {
            WindowErrorType::PopenError => error_string.push_str("PopenError"),
        }
        write!(f, "[{}]: {}", error_string, self.message)
    }
}

pub enum WindowErrorType {
    PopenError,
}

pub struct WindowError {
    error: WindowErrorType,
    message: String,
}

impl WindowError {
    pub(crate) fn new(err: WindowErrorType, msg: &str) -> Self {
        WindowError {
            error: err,
            message: msg.to_string(),
        }
    }
}

impl From<WindowError> for RustofiError {
    fn from(error: WindowError) -> Self {
        RustofiError {
            error: RustofiErrorType::WindowError,
            message: format!("{:?}", error),
        }
    }
}

impl fmt::Debug for RustofiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut error_string = String::new();
        match self.error {
            RustofiErrorType::BlankLineError => error_string.push_str("BlankLineError"),
            RustofiErrorType::WindowError => error_string.push_str("WindowError"),
        }
        write!(f, "[{}]: {}", error_string, self.message)
    }
}

pub enum RustofiErrorType {
    BlankLineError,
    WindowError,
}

pub struct RustofiError {
    error: RustofiErrorType,
    message: String,
}

impl RustofiError {
    pub(crate) fn new(err: RustofiErrorType, msg: &str) -> Self {
        RustofiError {
            error: err,
            message: msg.to_string(),
        }
    }
}
