pub trait IgnoreErrExtension<T, E>
where
    E: std::error::Error,
{
    fn ignore_err(self, f: impl FnOnce(&E) -> bool) -> Result<T, E>;
}

impl<T, E> IgnoreErrExtension<T, E> for Result<T, E>
where
    T: Default,
    E: std::error::Error,
{
    fn ignore_err(self, f: impl FnOnce(&E) -> bool) -> Result<T, E> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => {
                if f(&e) {
                    Ok(T::default())
                } else {
                    Err(e)
                }
            }
        }
    }
}
