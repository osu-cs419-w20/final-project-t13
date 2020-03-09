use deadpool_postgres::PoolError;
use warp::Rejection;

#[derive(Debug)]
pub enum Error {
    DBError(tokio_postgres::Error),
    DBPoolError(PoolError),
}

impl warp::reject::Reject for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        use Error::*;
        match self {
            DBError(_) => write!(fmt, "postgres database error"),
            DBPoolError(_) => write!(fmt, "postgres pool error"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use Error::*;
        match self {
            DBError(e) => Some(e),
            DBPoolError(e) => Some(e),
        }
    }
}

impl From<Error> for Rejection {
    fn from(e: Error) -> Rejection {
        warp::reject::custom(e)
    }
}

impl From<tokio_postgres::Error> for Error {
    fn from(e: tokio_postgres::Error) -> Self {
        Error::DBError(e)
    }
}

impl From<PoolError> for Error {
    fn from(e: PoolError) -> Self {
        Error::DBPoolError(e)
    }
}
