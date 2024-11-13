#[macro_export]
macro_rules! generate_enum {
    // The macro takes the enum name and its variants as input
    ($enum_name:ident { $($variant_name:ident),* }) => {
        // Conditional derive attributes for the database feature
        #[derive(strum_macros::EnumString, strum_macros::Display, Clone, Eq, PartialEq)]
        #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
        pub enum $enum_name {
            $(
                $variant_name,  // Add each variant to the enum
            )*
        }

        // Implement common traits for the enum
        medullah_web::impl_enum_common_traits!($enum_name);
    };
}
