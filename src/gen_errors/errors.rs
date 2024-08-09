use core::fmt;
use error_stack::Context;

#[derive(Debug)]
pub struct GeneralReportError {
    pub original_message: String
}

impl fmt::Display for GeneralReportError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("")
    }


}


impl Context for GeneralReportError {}


impl GeneralReportError {
    pub fn from_err<E: ToString>(err: E) -> GeneralReportError {
        GeneralReportError {
            original_message: err.to_string(),
        }
    }
}
