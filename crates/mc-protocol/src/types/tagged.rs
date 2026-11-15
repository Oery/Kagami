/// Declares an enum whose variant is chosen by a VarInt ID sent before its
/// value, from one table of `id NAME => Variant(Value)` rows.
///
/// Generates the enum, a module of the IDs as `NAME` constants, and `id`,
/// `read_value`, `encode_value` and `value_len`. Writing the ID itself is
/// left to the caller, whose framing differs. Rows without a value are
/// variants that carry nothing.
macro_rules! tagged_enum {
    (
        $(#[$enum_meta:meta])*
        pub enum $name:ident<$lt:lifetime>, ids in $ids:ident {
            $($(#[$meta:meta])* $id:literal $const_name:ident => $variant:ident $(($value:ty))?,)*
        }
    ) => {
        /// The wire IDs of each kind of value.
        pub mod $ids {
            $(pub const $const_name: i32 = $id;)*
        }

        $(#[$enum_meta])*
        #[derive(Clone, Debug, PartialEq, mc_protocol_macros::IntoOwned)]
        pub enum $name<$lt> {
            $($(#[$meta])* $variant $(($value))?,)*
        }

        impl<$lt> $name<$lt> {
            /// The ID sent before this value.
            pub fn id(&self) -> i32 {
                match self {
                    $(tagged_enum!(@pattern $variant, _value $(, $value)?) => $id,)*
                }
            }

            /// Reads the value that follows this ID.
            pub fn read_value(id: i32, input: &mut &$lt [u8]) -> crate::decode::Result<Self> {
                let value = match id {
                    $($id => tagged_enum!(@read $variant, input $(, $value)?),)*
                    _ => return Err(crate::error::DecodeError::EnumOutOfRange),
                };

                Ok(value)
            }

            /// Appends the value alone, without its ID.
            pub fn encode_value(&self, out: &mut Vec<u8>) -> crate::encode::Result {
                match self {
                    $(tagged_enum!(@pattern $variant, value $(, $value)?) => tagged_enum!(@encode value, out $(, $value)?),)*
                }
            }

            /// The length of the value alone, without its ID.
            pub fn value_len(&self) -> usize {
                match self {
                    $(tagged_enum!(@pattern $variant, value $(, $value)?) => tagged_enum!(@len value $(, $value)?),)*
                }
            }
        }
    };

    (@pattern $variant:ident, $binding:ident) => { Self::$variant };
    (@pattern $variant:ident, $binding:ident, $value:ty) => { Self::$variant($binding) };

    (@read $variant:ident, $input:ident) => { Self::$variant };
    (@read $variant:ident, $input:ident, $value:ty) => {
        Self::$variant(<$value as crate::packets::PacketWriter>::read($input)?)
    };

    (@encode $binding:ident, $out:ident) => { Ok(()) };
    (@encode $binding:ident, $out:ident, $value:ty) => { crate::packets::PacketWriter::encode($binding, $out) };

    (@len $binding:ident) => { 0 };
    (@len $binding:ident, $value:ty) => { crate::packets::PacketWriter::body_len($binding) };
}

pub(crate) use tagged_enum;
