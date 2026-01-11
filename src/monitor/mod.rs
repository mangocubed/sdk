use apalis::prelude::{BoxDynError, Error};

pub trait OrApalisError<T> {
    fn or_apalis_error(self) -> Result<T, Error>;
}

impl<T, E: std::error::Error + Send + Sync + 'static> OrApalisError<T> for Result<T, E> {
    fn or_apalis_error(self) -> Result<T, Error> {
        match self {
            Ok(value) => Ok(value),
            Err(err) => Err(apalis::prelude::Error::from(Box::new(err) as BoxDynError)),
        }
    }
}
