/// Implement [`sqlx::Decode`] trait for a type that implements [`TryFrom<&\[u8\]>`].
#[macro_export]
macro_rules! impl_sqlx_decode_from_bytes {
    ($t:ty) => {
        impl<'r, DB> sqlx::Decode<'r, DB> for $t
        where
            DB: sqlx::Database,
            for<'a> &'a [u8]: sqlx::Decode<'a, DB>,
            for<'a> Self: TryFrom<&'a [u8]>,
            for<'a> <Self as TryFrom<&'a [u8]>>::Error: std::error::Error + Send + Sync + 'static,
        {
            fn decode(
                value: <DB as sqlx::database::HasValueRef<'r>>::ValueRef,
            ) -> Result<Self, sqlx::error::BoxDynError> {
                let value = <&[u8] as sqlx::Decode<DB>>::decode(value)?;

                Ok(Self::try_from(value)?)
            }
        }
    };
}
