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

#[macro_export]
macro_rules! rec_lambda {
    (
        [$($immut_cap:ident: $immut_t:ty),*][$($mut_cap:ident: $mut_t:ty),*](
            $($param:ident: $param_t:ty),*
        ) -> $ret:ty $body:block
    ) => {{
        fn lambda(
            $($immut_cap: &$immut_t,)*
            $($mut_cap: &mut $mut_t,)*
            $($param: $param_t,)*
        ) -> $ret {
            with_dollar_sign! {
                ($d:tt) => {
                    macro_rules! recurse {
                        ($d($d arg:expr),*) => {
                            lambda($($immut_cap,)* $($mut_cap,)* $d($d arg,)*)
                        }
                    }
                }
            }
            $body
        }

        |$($param: $param_t,)*| {
            lambda($(&$immut_cap,)* $(&mut $mut_cap,)* $($param,)*)
        }
    }};
}

pub use rec_lambda;

#[macro_export]
macro_rules! debug {
    ($body:block) => {
        #[cfg(feature = "local")]
        {
            $body;
        }
    };
}

pub use debug;

#[macro_export]
macro_rules! debug_highlight {
    ($io:ident, $body:block) => {
        #[cfg(feature = "local")]
        {
            $io.write("\u{001b}[32m");
            $body;
            $io.write("\u{001b}[0m");
        }
    };
}

pub use debug_highlight;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rec_lambda() {
        let target = vec![69, 420, 1337, 6969, 42];
        let mut vector = vec![1, 2, 3, 4, 5];

        let mut test_sum_and_set = rec_lambda! {
            [target: Vec<i32>][vector: Vec<i32>](idx: usize) -> i32 {
                if idx == vector.len() {
                    return 0;
                }
                let temp = vector[idx];
                vector[idx] = target[idx];
                temp + recurse!(idx + 1)
            }
        };

        assert_eq!(test_sum_and_set(0), 15);
        assert_eq!(vector, target);
    }
}