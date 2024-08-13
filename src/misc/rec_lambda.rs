#![allow(unused_imports, unused_macros)]

pub use super::macros::with_dollar_sign;

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
