use reqwest::Error;

pub enum OsrsApiErr {
    UserNotFound,
    InvalidRequest,
    ApiIssue,
    FailedRequest(Error),
    UnknownIssue
}

impl From<Error> for OsrsApiErr {
    fn from(value: Error) -> Self {
        OsrsApiErr::FailedRequest(value)
    }
}

impl std::fmt::Display for OsrsApiErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            OsrsApiErr::UserNotFound => write!(f, "No user found with that RSN"),
            OsrsApiErr::InvalidRequest => write!(f, "That request was invalid."),
            OsrsApiErr::ApiIssue => write!(f, "There was an issue reaching the OSRS API."),
            OsrsApiErr::FailedRequest(e) => write!(f, "The request failed: {e}."),
            OsrsApiErr::UnknownIssue => write!(f, "There was an unknown issue.")
        }
    }
}