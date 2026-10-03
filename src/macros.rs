//! Macros shared by the models.

/// Generates a chainable setter for each optional field:
/// - `field: T` takes `impl Into<T>`,
/// - `field: [T]` takes `impl IntoIterator<Item = impl Into<T>>` for an array field,
/// - `field: u32` takes `u32` so that integer literals infer.
macro_rules! setters {
    ($ty:ident { $($fields:tt)* }) => {
        impl $ty {
            setters!(@field $($fields)*);
        }
    };
    (@field) => {};
    (@field $field:ident: [$t:ty] $(, $($rest:tt)*)?) => {
        #[doc = concat!("Sets `", stringify!($field), "`.")]
        pub fn $field(mut self, values: impl IntoIterator<Item = impl Into<$t>>) -> Self {
            self.$field = Some(values.into_iter().map(Into::into).collect());
            self
        }
        setters!(@field $($($rest)*)?);
    };
    (@field $field:ident: u32 $(, $($rest:tt)*)?) => {
        #[doc = concat!("Sets `", stringify!($field), "`.")]
        pub fn $field(mut self, value: u32) -> Self {
            self.$field = Some(value);
            self
        }
        setters!(@field $($($rest)*)?);
    };
    (@field $field:ident: $t:ty $(, $($rest:tt)*)?) => {
        #[doc = concat!("Sets `", stringify!($field), "`.")]
        pub fn $field(mut self, value: impl Into<$t>) -> Self {
            self.$field = Some(value.into());
            self
        }
        setters!(@field $($($rest)*)?);
    };
}

/// Defines an object that is discriminated by its `type` property.
///
/// Each tuple variant wraps a struct that writes its own `type` tag (with
/// `#[serde(tag = "type", rename = "...")]`), so the struct also serializes correctly
/// on its own. A unit variant stands for an object that has no property besides `type`.
/// Values whose `type` is not listed are kept verbatim in `Unknown`, so objects added
/// to the API later can still be decoded and sent back unchanged. A listed `type` whose
/// body does not match its struct is an error, as is a missing `type`.
macro_rules! tagged_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $( $(#[$vmeta:meta])* $tag:literal => $variant:ident $(($inner:ty))?, )*
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq)]
        #[allow(clippy::large_enum_variant)] // variants are built and matched by value; boxing would hurt ergonomics
        pub enum $name {
            $(
                #[doc = concat!("`", $tag, "`")]
                $(#[$vmeta])*
                $variant $(($inner))?,
            )*
            /// A `type` not known to this version of the crate, kept as received.
            Unknown(serde_json::Value),
        }

        impl serde::Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
                match self {
                    $( $name::$variant $((tagged_enum!(@bind value $inner)))? => {
                        tagged_enum!(@serialize serializer $tag $(value $inner)?)
                    } )*
                    $name::Unknown(value) => value.serialize(serializer),
                }
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
                let value = serde_json::Value::deserialize(deserializer)?;
                let Some(tag) = value.get("type").and_then(serde_json::Value::as_str) else {
                    return Err(serde::de::Error::missing_field("type"));
                };
                match tag {
                    $( $tag => tagged_enum!(@deserialize value $name::$variant $($inner)?), )*
                    _ => Ok($name::Unknown(value)),
                }
            }
        }

        $($(
            impl From<$inner> for $name {
                fn from(value: $inner) -> Self {
                    $name::$variant(value)
                }
            }
        )?)*
    };
    (@bind $value:ident $inner:ty) => {
        $value
    };
    (@serialize $serializer:ident $tag:literal $value:ident $inner:ty) => {
        $value.serialize($serializer)
    };
    (@serialize $serializer:ident $tag:literal) => {{
        use serde::ser::SerializeMap;
        let mut map = $serializer.serialize_map(Some(1))?;
        map.serialize_entry("type", $tag)?;
        map.end()
    }};
    (@deserialize $value:ident $name:ident::$variant:ident $inner:ty) => {
        <$inner>::deserialize($value)
            .map($name::$variant)
            .map_err(serde::de::Error::custom)
    };
    (@deserialize $value:ident $name:ident::$variant:ident) => {
        Ok($name::$variant)
    };
}
