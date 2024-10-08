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
#[macro_export]
macro_rules! rec_lambda {
    (
        |$rec:ident: Self $(, $param:ident: $param_t:ty)*| -> $ret:ty
        $body:block
    ) => {{
        use std::cell::UnsafeCell; // Macro-scoped import

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
            let mut lambda = Lambda::new(|$rec, $($param: $param_t),*| {
                let mut $rec = |$($param: $param_t),*| {
                    $rec.call($($param),*)
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
macro_rules! dbg_named_str {
    ($id:ident) => {
        &format!("[{} = {:?}]", stringify!($id), $id)
    };
}

pub use dbg_named_str;

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

    #[test]
    fn test_dbg_named_str() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(dbg_named_str!(arr), "[arr = [1, 2, 3, 4, 5]]");
    }
}
