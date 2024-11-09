#[macro_export]
macro_rules! generate_diesel_enum {
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

#[macro_export]
macro_rules! generate_diesel_enum_with_optional_features {
    // The macro accepts a feature name, enum name, and its variants as input
    ($feature:literal, $enum_name:ident { $($variant_name:ident),* }) => {
        // Conditional derive attributes based on the feature passed
        #[cfg_attr(feature = $feature, derive(diesel::AsExpression, diesel::FromSqlRow))]
        #[cfg_attr(feature = $feature, diesel(sql_type = diesel::sql_types::Text))]
        #[derive(strum_macros::EnumString, strum_macros::Display, Clone, Eq, PartialEq)]
        #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
        pub enum $enum_name {
            $(
                $variant_name,  // Add each variant to the enum
            )*
        }

        // Implement common traits for the enum
        medullah_web::impl_enum_common_traits!($enum_name);

        // Optionally, if the database feature is enabled, implement additional database traits
        #[cfg(feature = $feature)]
        medullah_web::impl_enum_diesel_traits!($enum_name);
    };
}
