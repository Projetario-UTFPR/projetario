#[macro_export]
macro_rules! unwrap_or_redirect {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(redirect) => return redirect,
        }
    };
}
