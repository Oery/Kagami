use nom::IResult;
use nom::Parser;
use nom::bytes::streaming::take;
use nom::multi::many_m_n;

use crate::tag::*;

pub fn parse_byte(input: &[u8]) -> IResult<&[u8], i8> {
    let (input, bytes) = take(1usize)(input)?;
    Ok((input, bytes[0] as i8))
}

pub fn parse_short(input: &[u8]) -> IResult<&[u8], i16> {
    let (input, bytes) = take(2usize)(input)?;
    Ok((input, i16::from_be_bytes(bytes.try_into().unwrap())))
}

pub fn parse_int(input: &[u8]) -> IResult<&[u8], i32> {
    let (input, bytes) = take(4usize)(input)?;
    Ok((input, i32::from_be_bytes(bytes.try_into().unwrap())))
}

pub fn parse_long(input: &[u8]) -> IResult<&[u8], i64> {
    let (input, bytes) = take(8usize)(input)?;
    Ok((input, i64::from_be_bytes(bytes.try_into().unwrap())))
}

pub fn parse_float(input: &[u8]) -> IResult<&[u8], f32> {
    let (input, bytes) = take(4usize)(input)?;
    Ok((input, f32::from_be_bytes(bytes.try_into().unwrap())))
}

pub fn parse_double(input: &[u8]) -> IResult<&[u8], f64> {
    let (input, bytes) = take(8usize)(input)?;
    Ok((input, f64::from_be_bytes(bytes.try_into().unwrap())))
}

pub fn parse_string(input: &[u8]) -> IResult<&[u8], String> {
    let (input, length_bytes) = take(2usize)(input)?;
    let length = u16::from_be_bytes(length_bytes.try_into().unwrap()) as usize;
    let (input, bytes) = take(length)(input)?;
    let s = String::from_utf8(bytes.to_vec())
        .map_err(|_| nom::Err::Failure(nom::error::Error::new(input, nom::error::ErrorKind::Fail)))?;
    Ok((input, s))
}

pub fn parse_byte_array(input: &[u8]) -> IResult<&[u8], Vec<i8>> {
    let (input, length) = parse_int(input)?;
    let (input, bytes) = take(length as usize)(input)?;
    let vec: Vec<i8> = bytes.iter().map(|b| *b as i8).collect();
    Ok((input, vec))
}

pub fn parse_int_array(input: &[u8]) -> IResult<&[u8], Vec<i32>> {
    let (input, length) = parse_int(input)?;
    let (input, bytes) = take((length * 4) as usize)(input)?;
    let mut vec = Vec::with_capacity(length as usize);
    for i in 0..length as usize {
        let offset = i * 4;
        let val = i32::from_be_bytes(bytes[offset..offset + 4].try_into().unwrap());
        vec.push(val);
    }
    Ok((input, vec))
}

pub fn parse_long_array(input: &[u8]) -> IResult<&[u8], Vec<i64>> {
    let (input, length) = parse_int(input)?;
    let (input, bytes) = take((length * 8) as usize)(input)?;
    let mut vec = Vec::with_capacity(length as usize);
    for i in 0..length as usize {
        let offset = i * 8;
        let val = i64::from_be_bytes(bytes[offset..offset + 8].try_into().unwrap());
        vec.push(val);
    }
    Ok((input, vec))
}

pub fn parse_value(input: &[u8], tag_type: u8) -> IResult<&[u8], NbtTag> {
    match tag_type {
        TAG_BYTE => {
            let (input, v) = parse_byte(input)?;
            Ok((input, NbtTag::Byte(v)))
        }
        TAG_SHORT => {
            let (input, v) = parse_short(input)?;
            Ok((input, NbtTag::Short(v)))
        }
        TAG_INT => {
            let (input, v) = parse_int(input)?;
            Ok((input, NbtTag::Int(v)))
        }
        TAG_LONG => {
            let (input, v) = parse_long(input)?;
            Ok((input, NbtTag::Long(v)))
        }
        TAG_FLOAT => {
            let (input, v) = parse_float(input)?;
            Ok((input, NbtTag::Float(v)))
        }
        TAG_DOUBLE => {
            let (input, v) = parse_double(input)?;
            Ok((input, NbtTag::Double(v)))
        }
        TAG_BYTE_ARRAY => {
            let (input, v) = parse_byte_array(input)?;
            Ok((input, NbtTag::ByteArray(v)))
        }
        TAG_STRING => {
            let (input, v) = parse_string(input)?;
            Ok((input, NbtTag::String(v)))
        }
        TAG_LIST => {
            let (input, v) = parse_list(input)?;
            Ok((input, NbtTag::List(v.0, v.1)))
        }
        TAG_COMPOUND => {
            let (input, v) = parse_compound(input)?;
            Ok((input, NbtTag::Compound(v)))
        }
        TAG_INT_ARRAY => {
            let (input, v) = parse_int_array(input)?;
            Ok((input, NbtTag::IntArray(v)))
        }
        TAG_LONG_ARRAY => {
            let (input, v) = parse_long_array(input)?;
            Ok((input, NbtTag::LongArray(v)))
        }
        TAG_END => Ok((input, NbtTag::Byte(0))),
        _ => Err(nom::Err::Failure(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Fail,
        ))),
    }
}

pub fn parse_named_tag(input: &[u8]) -> IResult<&[u8], (String, NbtTag)> {
    let (input, tag_type) = parse_byte(input)?;
    if tag_type == TAG_END as i8 {
        return Ok((input, (String::new(), NbtTag::Byte(0))));
    }
    let tag_type = tag_type as u8;
    let (input, name) = parse_string(input)?;
    let (input, value) = parse_value(input, tag_type)?;
    Ok((input, (name, value)))
}

pub fn parse_compound(input: &[u8]) -> IResult<&[u8], NbtCompound> {
    let mut map = NbtCompound::new();
    let mut remaining = input;

    while let Some(&tag_type) = remaining.first() {
        if tag_type == TAG_END {
            let (rest, _) = take(1usize)(remaining)?;
            remaining = rest;
            break;
        }

        let (rest, (name, value)) = parse_named_tag(remaining)?;
        map.insert(name, value);
        remaining = rest;
    }

    Ok((remaining, map))
}

pub fn parse_list(input: &[u8]) -> IResult<&[u8], (Vec<NbtTag>, u8)> {
    let (input, element_type) = take(1usize)(input)?;
    let element_type = element_type[0];
    let (input, length) = parse_int(input)?;

    if length == 0 {
        return Ok((input, (Vec::new(), element_type)));
    }

    let (input, items) =
        many_m_n(length as usize, length as usize, |i| parse_value(i, element_type)).parse(input)?;
    Ok((input, (items, element_type)))
}

pub fn parse_nbt(input: &[u8]) -> IResult<&[u8], (String, NbtTag)> {
    parse_named_tag(input)
}

pub fn from_bytes(input: &[u8]) -> crate::error::NbtResult<NbtTag> {
    let (_, (_name, tag)) =
        parse_nbt(input).map_err(|e| crate::error::NbtError::Parse(e.map_input(|i| i.to_vec())))?;
    Ok(tag)
}

pub fn from_compound_bytes(input: &[u8]) -> crate::error::NbtResult<NbtCompound> {
    let (_, compound) =
        parse_compound(input).map_err(|e| crate::error::NbtError::Parse(e.map_input(|i| i.to_vec())))?;
    Ok(compound)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serializer;

    fn roundtrip(tag: NbtTag, name: &str) {
        let bytes = serializer::to_bytes(&tag, name);
        let (remaining, (parsed_name, parsed)) = parse_nbt(&bytes).unwrap();
        assert!(remaining.is_empty());
        assert_eq!(parsed_name, name);
        assert_eq!(parsed, tag);
    }

    #[test]
    fn test_byte() {
        let tag = NbtTag::Byte(42);
        roundtrip(tag, "test");
    }

    #[test]
    fn test_byte_negative() {
        let tag = NbtTag::Byte(-128);
        roundtrip(tag, "neg");
    }

    #[test]
    fn test_short() {
        let tag = NbtTag::Short(32767);
        roundtrip(tag, "s");
    }

    #[test]
    fn test_int() {
        let tag = NbtTag::Int(-2147483648);
        roundtrip(tag, "i");
    }

    #[test]
    fn test_long() {
        let tag = NbtTag::Long(9223372036854775807);
        roundtrip(tag, "l");
    }

    #[test]
    fn test_float() {
        let tag = NbtTag::Float(3.14);
        roundtrip(tag, "pi");
    }

    #[test]
    fn test_double() {
        let tag = NbtTag::Double(2.718281828459045);
        roundtrip(tag, "e");
    }

    #[test]
    fn test_string() {
        let tag = NbtTag::String("Hello, 世界!".into());
        roundtrip(tag, "greeting");
    }

    #[test]
    fn test_byte_array() {
        let tag = NbtTag::ByteArray(vec![1, -2, 3, -4, 5]);
        roundtrip(tag, "arr");
    }

    #[test]
    fn test_int_array() {
        let tag = NbtTag::IntArray(vec![100, -200, 300]);
        roundtrip(tag, "iarr");
    }

    #[test]
    fn test_long_array() {
        let tag = NbtTag::LongArray(vec![1i64 << 40, -(1i64 << 50)]);
        roundtrip(tag, "larr");
    }

    #[test]
    fn test_empty_compound() {
        let tag = NbtTag::Compound(NbtCompound::new());
        roundtrip(tag, "empty");
    }

    #[test]
    fn test_nested_compound() {
        let mut inner = NbtCompound::new();
        inner.insert("x".into(), NbtTag::Int(10));
        inner.insert("y".into(), NbtTag::Int(20));

        let mut outer = NbtCompound::new();
        outer.insert("pos".into(), NbtTag::Compound(inner));
        outer.insert("name".into(), NbtTag::String("hello".into()));

        let tag = NbtTag::Compound(outer);
        roundtrip(tag, "root");
    }

    #[test]
    fn test_empty_list() {
        let tag = NbtTag::List(vec![], TAG_END);
        roundtrip(tag, "empty_list");
    }

    #[test]
    fn test_int_list() {
        let items = vec![NbtTag::Int(1), NbtTag::Int(2), NbtTag::Int(3)];
        let tag = NbtTag::List(items, TAG_INT);
        roundtrip(tag, "scores");
    }

    #[test]
    fn test_mixed_complex() {
        let mut compound = NbtCompound::new();
        compound.insert("byte".into(), NbtTag::Byte(99));
        compound.insert("short".into(), NbtTag::Short(42));
        compound.insert("int".into(), NbtTag::Int(12345));
        compound.insert("long".into(), NbtTag::Long(9876543210));
        compound.insert("float".into(), NbtTag::Float(1.5));
        compound.insert("double".into(), NbtTag::Double(0.001));
        compound.insert("str".into(), NbtTag::String("abc".into()));
        compound.insert("byte_arr".into(), NbtTag::ByteArray(vec![0, 1, 2]));
        compound.insert("int_arr".into(), NbtTag::IntArray(vec![10, 20]));
        compound.insert("long_arr".into(), NbtTag::LongArray(vec![100, 200]));

        let mut inner = NbtCompound::new();
        inner.insert("key".into(), NbtTag::String("val".into()));
        compound.insert("nested".into(), NbtTag::Compound(inner));

        let tag = NbtTag::Compound(compound);
        roundtrip(tag, "complex");
    }

    #[test]
    fn test_from_bytes_compound() {
        let mut compound = NbtCompound::new();
        compound.insert("name".into(), NbtTag::String("test".into()));
        let tag = NbtTag::Compound(compound);

        let bytes = serializer::to_bytes(&tag, "");
        let parsed = from_bytes(&bytes).unwrap();
        assert_eq!(parsed, tag);
    }

    #[test]
    fn test_from_compound_bytes() {
        let mut compound = NbtCompound::new();
        compound.insert("a".into(), NbtTag::Byte(1));
        compound.insert("b".into(), NbtTag::Byte(2));

        let bytes = serializer::to_compound_bytes(&compound);
        let parsed = from_compound_bytes(&bytes).unwrap();
        assert_eq!(parsed, compound);
    }

    #[test]
    fn test_long_string() {
        let long = "a".repeat(65535);
        let tag = NbtTag::String(long.clone());
        let bytes = serializer::to_bytes(&tag, "long");
        let (_, (name, parsed)) = parse_nbt(&bytes).unwrap();
        assert_eq!(name, "long");
        assert_eq!(parsed.as_string().unwrap(), &long);
    }

    #[test]
    fn test_list_of_lists() {
        let inner1 = NbtTag::List(vec![NbtTag::Int(1), NbtTag::Int(2)], TAG_INT);
        let inner2 = NbtTag::List(vec![NbtTag::Int(3), NbtTag::Int(4)], TAG_INT);
        let tag = NbtTag::List(vec![inner1, inner2], TAG_LIST);
        roundtrip(tag, "matrix");
    }

    #[test]
    fn test_deep_nesting() {
        let mut current = NbtCompound::new();
        current.insert("value".into(), NbtTag::Int(1));
        for _ in 0..100 {
            let mut next = NbtCompound::new();
            next.insert("child".into(), NbtTag::Compound(current));
            current = next;
        }
        let tag = NbtTag::Compound(current);
        roundtrip(tag, "deep");
    }

    #[test]
    fn test_accessors() {
        let mut compound = NbtCompound::new();
        compound.insert("x".into(), NbtTag::Int(42));
        let mut tag = NbtTag::Compound(compound);

        assert_eq!(tag.get("x").and_then(|t| t.as_int()), Some(42));
        assert_eq!(tag.get("nonexistent"), None);

        *tag.get_mut("x").unwrap() = NbtTag::Int(100);
        assert_eq!(tag.get("x").and_then(|t| t.as_int()), Some(100));

        tag.insert("y", NbtTag::String("hello".into()));
        assert_eq!(tag.get("y").and_then(|t| t.as_string()), Some("hello"));

        assert!(tag.contains_key("x"));
        let removed = tag.remove("x");
        assert!(removed.is_some());
        assert!(!tag.contains_key("x"));
    }

    #[test]
    fn test_empty_string() {
        let tag = NbtTag::String(String::new());
        roundtrip(tag, "empty_str");
    }

    #[test]
    fn test_empty_byte_array() {
        let tag = NbtTag::ByteArray(vec![]);
        roundtrip(tag, "empty_ba");
    }

    #[test]
    fn test_empty_int_array() {
        let tag = NbtTag::IntArray(vec![]);
        roundtrip(tag, "empty_ia");
    }

    #[test]
    fn test_empty_long_array() {
        let tag = NbtTag::LongArray(vec![]);
        roundtrip(tag, "empty_la");
    }

    #[test]
    fn test_as_byte() {
        assert_eq!(NbtTag::Byte(1).as_byte(), Some(1));
        assert_eq!(NbtTag::Int(1).as_byte(), None);
    }

    #[test]
    fn test_as_int_none_on_non_int() {
        assert_eq!(NbtTag::Byte(1).as_int(), None);
        assert_eq!(NbtTag::Short(1).as_int(), None);
        assert_eq!(NbtTag::String("x".into()).as_int(), None);
    }

    #[test]
    fn test_as_string_none_on_non_string() {
        assert_eq!(NbtTag::Int(1).as_string(), None);
        assert_eq!(NbtTag::Byte(1).as_string(), None);
        assert_eq!(NbtTag::Compound(NbtCompound::new()).as_string(), None);
    }

    #[test]
    fn test_as_short() {
        assert_eq!(NbtTag::Short(255).as_short(), Some(255));
        assert_eq!(NbtTag::Int(1).as_short(), None);
    }

    #[test]
    fn test_as_long() {
        assert_eq!(NbtTag::Long(1 << 40).as_long(), Some(1 << 40));
        assert_eq!(NbtTag::Int(1).as_long(), None);
    }

    #[test]
    fn test_as_float() {
        let f = NbtTag::Float(1.5);
        assert!((f.as_float().unwrap() - 1.5).abs() < 1e-6);
        assert_eq!(NbtTag::Int(1).as_float(), None);
    }

    #[test]
    fn test_as_double() {
        let d = NbtTag::Double(3.14);
        assert!((d.as_double().unwrap() - 3.14).abs() < 1e-10);
        assert_eq!(NbtTag::Int(1).as_double(), None);
    }

    #[test]
    fn test_as_byte_array() {
        let ba = NbtTag::ByteArray(vec![1, 2, 3]);
        assert_eq!(ba.as_byte_array(), Some(&[1, 2, 3][..]));
        assert_eq!(NbtTag::Int(1).as_byte_array(), None);
    }

    #[test]
    fn test_as_int_array() {
        let ia = NbtTag::IntArray(vec![10, 20]);
        assert_eq!(ia.as_int_array(), Some(&[10, 20][..]));
        assert_eq!(NbtTag::Int(1).as_int_array(), None);
    }

    #[test]
    fn test_as_long_array() {
        let la = NbtTag::LongArray(vec![100, 200]);
        assert_eq!(la.as_long_array(), Some(&[100, 200][..]));
        assert_eq!(NbtTag::Int(1).as_long_array(), None);
    }

    #[test]
    fn test_as_compound_ref() {
        let c = NbtTag::Compound(NbtCompound::new());
        assert!(c.as_compound().is_some());
        assert_eq!(NbtTag::Int(1).as_compound(), None);
    }

    #[test]
    fn test_as_compound_mut() {
        let mut c = NbtTag::Compound(NbtCompound::new());
        assert!(c.as_compound_mut().is_some());
        let mut not = NbtTag::Int(1);
        assert!(not.as_compound_mut().is_none());
    }

    #[test]
    fn test_as_list() {
        let l = NbtTag::List(vec![], TAG_END);
        assert!(l.as_list().is_some());
        assert_eq!(NbtTag::Int(1).as_list(), None);
    }

    #[test]
    fn test_as_list_mut() {
        let mut l = NbtTag::List(vec![], TAG_END);
        assert!(l.as_list_mut().is_some());
        let mut not = NbtTag::Int(1);
        assert!(not.as_list_mut().is_none());
    }

    #[test]
    fn test_list_element_type() {
        assert_eq!(NbtTag::List(vec![], TAG_INT).list_element_type(), Some(TAG_INT));
        assert_eq!(
            NbtTag::List(vec![NbtTag::Int(1)], TAG_INT).list_element_type(),
            Some(TAG_INT)
        );
        assert_eq!(NbtTag::Int(1).list_element_type(), None);
    }

    #[test]
    fn test_non_compound_accessors_return_none() {
        let mut t = NbtTag::Int(42);
        assert_eq!(t.get("key"), None);
        assert_eq!(t.get_mut("key"), None);
        assert_eq!(t.remove("key"), None);
        assert!(!t.contains_key("key"));
    }

    #[test]
    fn test_insert_on_non_compound_is_noop() {
        let mut t = NbtTag::Int(42);
        t.insert("key", NbtTag::Byte(1));
        assert_eq!(t, NbtTag::Int(42));
    }

    #[test]
    fn test_parse_invalid_utf8_string() {
        let bytes = [0, 2, 0xFF, 0xFE];
        let result = parse_string(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_unknown_tag_type() {
        let bytes = [0xFF];
        let result = parse_value(&bytes, 0xFF);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_bytes_invalid_data() {
        let result = from_bytes(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_compound_bytes_invalid_data() {
        let result = from_compound_bytes(&[0xFF, 0x00]);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_value_tag_end() {
        let (remaining, tag) = parse_value(&[], TAG_END).unwrap();
        assert!(remaining.is_empty());
        assert_eq!(tag, NbtTag::Byte(0));
    }

    #[test]
    fn test_parse_named_tag_end() {
        let result = parse_named_tag(&[TAG_END]);
        let (remaining, (name, tag)) = result.unwrap();
        assert!(remaining.is_empty());
        assert_eq!(name, "");
        assert_eq!(tag, NbtTag::Byte(0));
    }

    #[test]
    fn test_parse_compound_with_end_tag() {
        let mut compound = NbtCompound::new();
        compound.insert("a".into(), NbtTag::Int(1));

        let bytes = serializer::to_compound_bytes(&compound);
        let mut extra = bytes.to_vec();
        extra.extend_from_slice(b"extra");

        let (remaining, parsed) = parse_compound(&extra).unwrap();
        assert_eq!(parsed, compound);
        assert_eq!(remaining, b"extra");
    }
}
