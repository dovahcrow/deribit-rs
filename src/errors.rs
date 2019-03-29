use failure::{Error, Fail};
use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, Error>;

#[derive(Fail, Debug)]
pub enum DeribitError {
    #[fail(display = "Dummy error")]
    Dummy,
    #[fail(display = "Deribit remote error {{code: {}, message: {}}}", code, message)]
    RemoteError { code: i64, message: String },
}
