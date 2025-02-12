// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    BadRequestError(crate::error::BadRequestError),
    CapacityExceededError(crate::error::CapacityExceededError),
    InvalidSessionError(crate::error::InvalidSessionError),
    LimitExceededError(crate::error::LimitExceededError),
    OccConflictError(crate::error::OccConflictError),
    RateExceededError(crate::error::RateExceededError),
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BadRequestError(inner) => inner.fmt(f),
            Error::CapacityExceededError(inner) => inner.fmt(f),
            Error::InvalidSessionError(inner) => inner.fmt(f),
            Error::LimitExceededError(inner) => inner.fmt(f),
            Error::OccConflictError(inner) => inner.fmt(f),
            Error::RateExceededError(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl From<smithy_http::result::SdkError<crate::error::SendCommandError>> for Error {
    fn from(err: smithy_http::result::SdkError<crate::error::SendCommandError>) -> Self {
        match err {
            smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::SendCommandErrorKind::BadRequestError(inner) => {
                    Error::BadRequestError(inner)
                }
                crate::error::SendCommandErrorKind::CapacityExceededError(inner) => {
                    Error::CapacityExceededError(inner)
                }
                crate::error::SendCommandErrorKind::InvalidSessionError(inner) => {
                    Error::InvalidSessionError(inner)
                }
                crate::error::SendCommandErrorKind::LimitExceededError(inner) => {
                    Error::LimitExceededError(inner)
                }
                crate::error::SendCommandErrorKind::OccConflictError(inner) => {
                    Error::OccConflictError(inner)
                }
                crate::error::SendCommandErrorKind::RateExceededError(inner) => {
                    Error::RateExceededError(inner)
                }
                crate::error::SendCommandErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}
