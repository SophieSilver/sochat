//! Utilities for type conversions

/// Generates an implementation of `From<A>` for type `C`` 
/// by first converting `A` to `B` and then `B` to `C`
/// 
/// Useful for converting error types, especially in conjunction with [`thiserror`]
/// 
/// # Example
/// ```
/// use common::from_passthrough;
/// struct A;
/// struct B;
/// struct C;
/// 
/// impl From<A> for B {
///     fn from(value: A) -> B {
///         B
///     }
/// }
/// 
/// impl From<B> for C {
///     fn from(value: B) -> C {
///         C
///     }
/// }
/// 
/// // implements From<A> for C by first converting A to B and then B to C
/// from_passthrough!(A => B => C);
/// 
/// let c: C = A.into();
/// ```
#[macro_export]
macro_rules! from_passthrough {
    ($a:ty => $b:ty => $c:ty) => {
        impl From<$a> for $c {
            fn from(value: $a) -> $c {
                <$b>::from(value).into()
            }
        }
    };
}