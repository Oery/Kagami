pub mod error;
pub mod parser;
pub mod serializer;
pub mod tag;

pub use error::{NbtError, NbtResult};
pub use parser::{from_bytes, from_compound_bytes, parse_nbt};
pub use serializer::{to_bytes, to_compound_bytes};
pub use tag::{
    NbtCompound, NbtTag, TAG_BYTE, TAG_BYTE_ARRAY, TAG_COMPOUND, TAG_DOUBLE, TAG_END, TAG_FLOAT, TAG_INT,
    TAG_INT_ARRAY, TAG_LIST, TAG_LONG, TAG_LONG_ARRAY, TAG_SHORT, TAG_STRING,
};
