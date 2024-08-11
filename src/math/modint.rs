use std::fmt::{Debug, Display, Formatter};
use std::ops::*;

#[rustfmt::skip]
macro_rules! define_modint {
    ($name:ident, $value_type:ty, $mult_type:ty) => {
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct $name<const MOD: $value_type>($value_type);

        macro_rules! impl_op {
            (
                $type:ident, $op_assign:ident, $op:ident, $fn_assign:ident,
                $fn_op:ident, $this:ident, $other:ident, $impl:block
            ) => {
                impl<const MOD: $value_type> $op_assign for $type<MOD> {
                    fn $fn_assign(&mut $this, $other: Self) {
                        $impl
                    }
                }

                impl<const MOD: $value_type> $op for $type<MOD> {
                    type Output = Self;

                    fn $fn_op(self, rhs: Self) -> Self::Output {
                        let mut res = self.clone();
                        res.$fn_assign(rhs);
                        res
                    }
                }
            };
        }

        impl_op!($name, AddAssign, Add, add_assign, add, self, other, {
            self.0 = (self.0 + other.0) % MOD;
        });
        impl_op!($name, SubAssign, Sub, sub_assign, sub, self, other, {
            self.0 = ((self.0 - other.0) % MOD + MOD) % MOD;
        });
        impl_op!($name, MulAssign, Mul, mul_assign, mul, self, other, {
            self.0 = ((self.0 as $mult_type * other.0 as $mult_type)
                % MOD as $mult_type) as $value_type;
        });

        impl<const MOD: $value_type> $name<MOD> {
            pub const fn new(value: $value_type) -> Self {
                Self(value)
            }
            
            pub fn pow(self, exp: u64) -> Self {
                if exp == 0 {
                    Self(1)
                } else if exp % 2 == 1 {
                    self * self.pow(exp - 1)
                } else {
                    let temp = self.pow(exp / 2);
                    temp * temp
                }
            }

            pub fn inv(self) -> Self {
                self.pow(MOD as u64 - 2)
            }
        }
        
        impl<const MOD: $value_type> From<$value_type> for $name<MOD> {
            fn from(value: $value_type) -> Self {
                Self::new(value % MOD)
            }
        }
        
        impl<const MOD: $value_type> From<usize> for $name<MOD> {
            fn from(value: usize) -> Self {
                Self::new((value % MOD as usize) as $value_type)
            }
        }

        impl_op!($name, DivAssign, Div, div_assign, div, self, other, {
            self.0 = (Self(self.0) * other.inv()).0;
        });

        impl<const MOD: $value_type> Into<$value_type> for $name<MOD> {
            fn into(self) -> $value_type {
                self.0
            }
        }

        impl <const MOD: $value_type> Display for $name<MOD> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
        
        impl <const MOD: $value_type> Debug for $name<MOD> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self.0)
            }
        }
    };
}

define_modint!(ModInt32, i32, i64);
define_modint!(ModInt64, i64, i128);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_all() {
        type ModInt = ModInt32<7>;

        assert_eq!(ModInt::from(2) + ModInt::from(5), ModInt::from(0));
        assert_eq!(ModInt::from(2) - ModInt::from(5), ModInt::from(4));
        assert_eq!(ModInt::from(6) * ModInt::from(5), ModInt::from(2));
        assert_eq!(ModInt::from(6) / ModInt::from(3), ModInt::from(2));
        assert_eq!(ModInt::from(2).pow(15), ModInt::from(1));
    }

    #[test]
    fn test_overflow() {
        type ModInt = ModInt32<10>;
        assert_eq!(ModInt::from(123123123).pow(8), ModInt::from(1));
    }
}
