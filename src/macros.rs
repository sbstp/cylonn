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
