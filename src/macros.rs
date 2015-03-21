/// Attempt to unwrap an Option<_>.
/// If the Option is None, return.
/// A return value is optional (depends on the function's return type).
macro_rules! some_or_return {
    ( $opt:expr ) => {
        match $opt {
            None => return,
            Some(val) => val,
        }
    };
    ( $opt:expr, $retval:expr ) => {
        match $opt {
            None => return $retval,
            Some(val) => val,
        }
    };
}

/// Macro to easily implement FromError.
/// from_error!(MyError, IoError, MyError::IoError)
macro_rules! from_error {
    ( $t:ty, $err:ty, $name:path ) => {
        use std;
        impl std::error::FromError<$err> for $t {
            fn from_error(err: $err) -> $t {
                $name(err)
            }
        }
    }
}
