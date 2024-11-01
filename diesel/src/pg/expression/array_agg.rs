use diesel_derives::define_sql_function;

use crate::sql_types::SingleValue;

#[cfg(feature = "postgres_backend")]
define_sql_function! {
    #[aggregate]
    fn array_agg<T: SingleValue>(a: T) -> Array<T>;
}
