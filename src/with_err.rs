pub trait WithErrExtension<T, E>
where
    E: std::error::Error,
{
    fn with_err(
        self,
        match_err: impl FnOnce(&E) -> bool,
        f: impl FnOnce() -> Result<T, E>,
    ) -> Result<T, E>;
}

impl<T, E> WithErrExtension<T, E> for Result<T, E>
where
    E: std::error::Error,
{
    fn with_err(
        self,
        match_err: impl FnOnce(&E) -> bool,
        f: impl FnOnce() -> Result<T, E>,
    ) -> Result<T, E> {
        if let Ok(t) = self {
            return Ok(t);
        }
        let err = self.err().unwrap();
        if match_err(&err) {
            return f();
        }

        Err(err)
    }
}
