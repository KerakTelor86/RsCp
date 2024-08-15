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

// Ref:
// https://codeforces.com/blog/entry/103794?#comment-921890
pub use std::cell::UnsafeCell;
#[macro_export]
macro_rules! rec_lambda {
    (
        |$y:ident: Self $(, $param:ident: $param_t:ty)*| -> $ret:ty
        $body:block
    ) => {{
        trait Callable {
            fn call(&mut self, $($param: $param_t),*) -> $ret;
        }

        struct Lambda<F: FnMut(&mut dyn Callable, $($param_t),*) -> $ret> {
            func: UnsafeCell<F>,
        }

        impl<F: FnMut(&mut dyn Callable, $($param_t),*) -> $ret> Lambda<F> {
            fn new(func: F) -> Self {
                Self {
                    func: UnsafeCell::new(func),
                }
            }
        }

        impl<
            F: FnMut(&mut dyn Callable, $($param_t),*) -> $ret
        > Callable for Lambda<F> {
            fn call(&mut self, $($param: $param_t),*) -> $ret {
                let func = self.func.get();
                unsafe { (*func)(self, $($param),*) }
            }
        }

        |$($param: $param_t,)*| {
            let mut lambda = Lambda::new(|$y, $($param: $param_t),*| {
                let mut $y = |$($param: $param_t),*| {
                    $y.call($($param),*)
                };
                $body
            });
            lambda.call($($param),*)
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
            |rec: Self, idx: usize| -> i32 {
                if idx == vector.len() {
                    return 0;
                }
                let temp = vector[idx];
                vector[idx] = target[idx];
                temp + rec(idx + 1)
            }
        };

        assert_eq!(test_sum_and_set(0), 15);
        assert_eq!(vector, target);
    }
}