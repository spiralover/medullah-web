#[macro_export]
macro_rules! impl_enum_common_traits {
    ($variant_name:ident, $visitor_name:ident) => {
        use serde::de::{Unexpected, Visitor};
        use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
        use std::fmt;
        use std::fmt::{Debug, Display, Formatter};
        use std::str::FromStr;

        impl Display for $variant_name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl AsRef<str> for $variant_name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl Debug for $variant_name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl Serialize for $variant_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }

        // Implement Deserialize
        impl<'de> Deserialize<'de> for $variant_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct $visitor_name;

                impl<'de> Visitor<'de> for $visitor_name {
                    type Value = $variant_name;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("a valid app event string")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<$variant_name, E>
                    where
                        E: de::Error,
                    {
                        $variant_name::from_str(value)
                            .map_err(|_| de::Error::invalid_value(Unexpected::Str(value), &self))
                    }
                }

                deserializer.deserialize_str($visitor_name)
            }
        }
    };
}
