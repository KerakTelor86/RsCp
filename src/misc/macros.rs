#[macro_export]
macro_rules! ndvec {
    ([$($tokens:tt)+]) => {
        Vec<ndvec![$($tokens)*]>
    };
    ($type:ty) => {
        Vec<$type>
    };

    ($init:expr; ($cur_dim:expr, $($rest_dim:expr),*)) => {
        vec![ndvec![$init; ($($rest_dim),*)]; $cur_dim]
    };
    ($init:expr; ($dim:expr)) => {
        vec![$init; $dim]
    };
}

pub use ndvec;

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
