use std::collections::BTreeMap;

pub type NbtCompound = BTreeMap<String, NbtTag>;

pub const TAG_END: u8 = 0;
pub const TAG_BYTE: u8 = 1;
pub const TAG_SHORT: u8 = 2;
pub const TAG_INT: u8 = 3;
pub const TAG_LONG: u8 = 4;
pub const TAG_FLOAT: u8 = 5;
pub const TAG_DOUBLE: u8 = 6;
pub const TAG_BYTE_ARRAY: u8 = 7;
pub const TAG_STRING: u8 = 8;
pub const TAG_LIST: u8 = 9;
pub const TAG_COMPOUND: u8 = 10;
pub const TAG_INT_ARRAY: u8 = 11;
pub const TAG_LONG_ARRAY: u8 = 12;

#[derive(Debug, Clone, PartialEq)]
pub enum NbtTag {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<NbtTag>, u8),
    Compound(NbtCompound),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl NbtTag {
    pub fn as_byte(&self) -> Option<i8> {
        match self {
            NbtTag::Byte(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_short(&self) -> Option<i16> {
        match self {
            NbtTag::Short(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i32> {
        match self {
            NbtTag::Int(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_long(&self) -> Option<i64> {
        match self {
            NbtTag::Long(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f32> {
        match self {
            NbtTag::Float(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_double(&self) -> Option<f64> {
        match self {
            NbtTag::Double(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            NbtTag::String(v) => Some(v.as_str()),
            _ => None,
        }
    }

    pub fn as_byte_array(&self) -> Option<&[i8]> {
        match self {
            NbtTag::ByteArray(v) => Some(v.as_slice()),
            _ => None,
        }
    }

    pub fn as_int_array(&self) -> Option<&[i32]> {
        match self {
            NbtTag::IntArray(v) => Some(v.as_slice()),
            _ => None,
        }
    }

    pub fn as_long_array(&self) -> Option<&[i64]> {
        match self {
            NbtTag::LongArray(v) => Some(v.as_slice()),
            _ => None,
        }
    }

    pub fn as_compound(&self) -> Option<&NbtCompound> {
        match self {
            NbtTag::Compound(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_compound_mut(&mut self) -> Option<&mut NbtCompound> {
        match self {
            NbtTag::Compound(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&Vec<NbtTag>> {
        match self {
            NbtTag::List(v, _) => Some(v),
            _ => None,
        }
    }

    pub fn as_list_mut(&mut self) -> Option<&mut Vec<NbtTag>> {
        match self {
            NbtTag::List(v, _) => Some(v),
            _ => None,
        }
    }

    pub fn tag_type(&self) -> u8 {
        match self {
            NbtTag::Byte(_) => TAG_BYTE,
            NbtTag::Short(_) => TAG_SHORT,
            NbtTag::Int(_) => TAG_INT,
            NbtTag::Long(_) => TAG_LONG,
            NbtTag::Float(_) => TAG_FLOAT,
            NbtTag::Double(_) => TAG_DOUBLE,
            NbtTag::ByteArray(_) => TAG_BYTE_ARRAY,
            NbtTag::String(_) => TAG_STRING,
            NbtTag::List(_, _) => TAG_LIST,
            NbtTag::Compound(_) => TAG_COMPOUND,
            NbtTag::IntArray(_) => TAG_INT_ARRAY,
            NbtTag::LongArray(_) => TAG_LONG_ARRAY,
        }
    }

    pub fn list_element_type(&self) -> Option<u8> {
        match self {
            NbtTag::List(_, type_id) => Some(*type_id),
            _ => None,
        }
    }
}

impl NbtTag {
    pub fn get(&self, key: &str) -> Option<&NbtTag> {
        match self {
            NbtTag::Compound(map) => map.get(key),
            _ => None,
        }
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut NbtTag> {
        match self {
            NbtTag::Compound(map) => map.get_mut(key),
            _ => None,
        }
    }

    pub fn insert(&mut self, key: impl Into<String>, value: NbtTag) {
        if let NbtTag::Compound(map) = self {
            map.insert(key.into(), value);
        }
    }

    pub fn remove(&mut self, key: &str) -> Option<NbtTag> {
        match self {
            NbtTag::Compound(map) => map.remove(key),
            _ => None,
        }
    }

    pub fn contains_key(&self, key: &str) -> bool {
        match self {
            NbtTag::Compound(map) => map.contains_key(key),
            _ => false,
        }
    }
}
