#[macro_export]
macro_rules! generate_enum {
    // The macro takes the enum name and its variants as input
    ($enum_name:ident { $($variant_name:ident $( ($variant_type:ty) )? ),* $(,)? }) => {
        #[derive(strum_macros::EnumString, strum_macros::Display, Clone, Eq, PartialEq)]
        #[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
        pub enum $enum_name {
            $(
                $variant_name $( ($variant_type) )?,
            )*
        }

        // Implement common traits for the enum
        medullah_web::impl_enum_common_traits!($enum_name);
    };
}
