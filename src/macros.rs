



#[macro_export]
macro_rules! require {
    ( $constraint:expr, $error:expr ) => {
        if !$constraint {
            return Err($error.into());
        }
    };
}
