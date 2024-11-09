#[macro_export]
macro_rules! generate_enum {
    // The macro takes the enum name and its variants as input
    ($enum_name:ident { $($variant_name:ident),* }) => {
        // Conditional derive attributes for the database feature
        #[derive(diesel::AsExpression, diesel::FromSqlRow, strum_macros::EnumString, strum_macros::Display, Clone, Eq, PartialEq)]
        #[diesel(sql_type = diesel::sql_types::Text)]
        #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
        pub enum $enum_name {
            $(
                $variant_name,  // Add each variant to the enum
            )*
        }

        // Implement common traits for the enum
        medullah_web::impl_enum_common_traits!($enum_name);

        // Implement traits required for Diesel usage
        medullah_web::impl_enum_diesel_traits!($enum_name);
    };
}
