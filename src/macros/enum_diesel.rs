#[macro_export]
macro_rules! impl_enum_diesel_traits {
    ($variant_name:ident) => {
        use diesel::deserialize::FromSql;
        use diesel::pg::{Pg, PgValue};
        use diesel::serialize::{IsNull, Output, ToSql};
        use diesel::sql_types::Text;

        use std::io::Write;


        impl FromSql<Text, Pg> for $variant_name {
            fn from_sql(bytes: PgValue) -> diesel::deserialize::Result<Self> {
                let value = <String as FromSql<Text, Pg>>::from_sql(bytes)?;
                match $variant_name::from_str(value.as_str()) {
                    Ok(variant) => Ok(variant),
                    Err(err) => Err(err.to_string().into())
                }
            }
        }

        impl ToSql<Text, Pg> for $variant_name {
            fn to_sql(&self, out: &mut Output<Pg>) -> diesel::serialize::Result {
                let value = self.as_str();
                out.write_all(value.as_bytes())?;
                Ok(IsNull::No)
            }
        }

    };
}
