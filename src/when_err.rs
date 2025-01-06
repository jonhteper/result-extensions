pub trait WhenErrExtension<T, E>
where
    E: std::error::Error,
{
    fn when_err(self, match_err: impl FnOnce(&E) -> bool) -> WhenError<T, E>;
}

pub struct WhenError<T, E>
where
    E: std::error::Error,
{
    result: Result<T, E>,
    is_err_match: bool,
}

impl<T, E> WhenError<T, E>
where
    E: std::error::Error,
{
    pub fn run(self, f: impl FnOnce() -> Result<T, E>) -> Result<T, E> {
        if let Ok(v) = self.result {
            return Ok(v);
        }
        if self.is_err_match {
            return f();
        }

        Err(self.result.err().unwrap())
    }
}

impl<T, E> WhenErrExtension<T, E> for Result<T, E>
where
    E: std::error::Error,
{
    fn when_err(self, match_err: impl FnOnce(&E) -> bool) -> WhenError<T, E> {
        let mut is_err_match = false;
        if let Err(e) = &self {
            is_err_match = match_err(&e);
        }

        WhenError {
            result: self,
            is_err_match,
        }
    }
}
