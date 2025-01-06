mod ignore_error;
mod when_err;
mod with_err;

pub use ignore_error::*;
pub use when_err::*;
pub use with_err::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(thiserror::Error, Debug, PartialEq, Eq)]
    enum Error {
        #[error("A error")]
        A,

        #[error("B error")]
        B,
    }

    #[derive(Eq, PartialEq, Debug, Default)]
    struct ReturnType;

    struct Executor;

    impl Executor {
        fn should_fail_with_a(&self) -> Result<ReturnType, Error> {
            Err(Error::A)
        }

        fn should_fail_with_b(&self) -> Result<ReturnType, Error> {
            Err(Error::B)
        }

        fn should_succeed(&self) -> Result<ReturnType, Error> {
            Ok(ReturnType)
        }
    }

    #[test]
    fn test_ignore_err() {
        let executor = Executor;

        assert_eq!(
            executor
                .should_fail_with_a()
                .ignore_err(|e| matches!(e, Error::A)),
            Ok(ReturnType)
        );
        assert_eq!(
            executor
                .should_fail_with_a()
                .ignore_err(|e| matches!(e, Error::B)),
            Err(Error::A)
        );
        assert_eq!(
            executor
                .should_fail_with_b()
                .ignore_err(|e| matches!(e, Error::B)),
            Ok(ReturnType)
        );
        assert_eq!(
            executor
                .should_fail_with_b()
                .ignore_err(|e| matches!(e, Error::A)),
            Err(Error::B)
        );
        assert_eq!(
            executor
                .should_succeed()
                .ignore_err(|e| matches!(e, Error::A)),
            Ok(ReturnType)
        );
        assert_eq!(
            executor
                .should_succeed()
                .ignore_err(|e| matches!(e, Error::B)),
            Ok(ReturnType)
        );
    }

    #[test]
    fn test_with_err() {
        let executor = Executor;

        assert_eq!(
            executor
                .should_fail_with_a()
                .with_err(|e| matches!(e, Error::A), || executor.should_succeed()),
            Ok(ReturnType)
        );
        assert_eq!(
            executor
                .should_fail_with_a()
                .with_err(|_| false, || Ok(ReturnType)),
            Err(Error::A)
        );
    }

    #[test]
    fn test_when_err() {
        let executor = Executor;

        assert_eq!(
            executor
                .should_fail_with_a()
                .when_err(|e| matches!(e, Error::A))
                .run(|| executor.should_succeed()),
            Ok(ReturnType)
        );

        assert_eq!(
            executor
                .should_fail_with_a()
                .when_err(|e| matches!(e, Error::B))
                .run(|| executor.should_succeed()),
            Err(Error::A)
        );

        assert_eq!(
            executor
                .should_succeed()
                .when_err(|e| matches!(e, Error::B))
                .run(|| executor.should_fail_with_a()),
            Ok(ReturnType)
        );
    }
}
