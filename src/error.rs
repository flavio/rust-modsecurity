#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ModSecurityError {
    /// Error when converting a string to a C string
    Nul(std::ffi::NulError),
    /// Error when processing a connection
    ProcessConnection,
    /// Error when processing logging
    ProcessLogging,
    /// Error when processing the request body
    ProcessRequestBody,
    /// Error when processing the request headers
    ProcessRequestHeaders,
    /// Error when adding a request header
    AddRequestHeader,
    /// Error when checking for an intervention
    Intervention,
    /// Error when adding a file to the rule set
    RulesAddFile(String),
    /// Error when updating the status code
    UpdateStatusCode,
}

impl From<std::ffi::NulError> for ModSecurityError {
    fn from(err: std::ffi::NulError) -> Self {
        ModSecurityError::Nul(err)
    }
}
