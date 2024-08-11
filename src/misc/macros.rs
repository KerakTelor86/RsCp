// What the fuck???
// https://github.com/rust-lang/rust/issues/35853#issuecomment-415993963
#[macro_export]
macro_rules! with_dollar_sign {
    ($($body:tt)*) => {
        macro_rules! __with_dollar_sign { $($body)* }
        __with_dollar_sign!($);
    }
}

pub use with_dollar_sign;
