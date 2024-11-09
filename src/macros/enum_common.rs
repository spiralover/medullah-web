#[macro_export]
/// Implements Display, Debug, AsRef<str>, serde::Serialize, serde::Deserialize traits for an enum.
macro_rules! impl_enum_common_traits {
    ($variant_name:ident) => {
        impl std::fmt::Debug for $variant_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.to_string())
            }
        }

        impl serde::Serialize for $variant_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.to_string())
            }
        }

        // Implement Deserialize
        impl<'de> serde::Deserialize<'de> for $variant_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct EnumVisitor;

                impl<'de> serde::de::Visitor<'de> for EnumVisitor {
                    type Value = $variant_name;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("a valid app event string")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<$variant_name, E>
                    where
                        E: serde::de::Error,
                    {
                        use std::str::FromStr;

                        $variant_name::from_str(value).map_err(|_| {
                            serde::de::Error::invalid_value(
                                serde::de::Unexpected::Str(value),
                                &self,
                            )
                        })
                    }
                }

                deserializer.deserialize_str(EnumVisitor)
            }
        }
    };
}

#[macro_export]
/// Implements Display, Debug, AsRef<str>, serde::Serialize, serde::Deserialize traits for an enum.
macro_rules! impl_enum_display_trait {
    ($variant_name:ident) => {
        impl std::fmt::Display for $variant_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
    };
}
