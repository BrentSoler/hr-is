use serde::Serialize;
use ts_rs::TS;

#[derive(thiserror::Error, Debug)]
pub enum Errors<'a> {
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    DateError(#[from] chrono::format::ParseError),

    #[error("{0} does not Exists")]
    RowNotFound(&'a str),

    #[error("{0}")]
    JwtError(&'a str),
}

#[derive(Serialize, TS)]
#[ts(export)]
pub struct ErrMsg {
    pub err_msg: String,
}
