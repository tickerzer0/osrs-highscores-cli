use reqwest::Error;

pub enum OsrsApiErr {
    UserNotFound,
    InvalidRequest,
    ApiIssue,
    NoContent,
    FailedRequest(Error),
    UnknownIssue
}

impl From<Error> for OsrsApiErr {
    fn from(value: Error) -> Self {
        OsrsApiErr::FailedRequest(value)
    }
}