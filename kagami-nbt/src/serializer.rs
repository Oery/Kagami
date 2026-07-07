use std::io::Write;

use crate::tag::*;

pub fn write_string(buf: &mut Vec<u8>, s: &str) {
    let len = s.len() as u16;
    buf.write_all(&len.to_be_bytes()).unwrap();
    buf.write_all(s.as_bytes()).unwrap();
}

pub fn write_tag(buf: &mut Vec<u8>, name: &str, tag: &NbtTag) {
    buf.push(tag.tag_type());
    write_string(buf, name);
    write_value(buf, tag);
}

pub fn write_value(buf: &mut Vec<u8>, tag: &NbtTag) {
    match tag {
        NbtTag::Byte(v) => {
            buf.push(*v as u8);
        }
        NbtTag::Short(v) => {
            buf.write_all(&v.to_be_bytes()).unwrap();
        }
        NbtTag::Int(v) => {
            buf.write_all(&v.to_be_bytes()).unwrap();
        }
        NbtTag::Long(v) => {
            buf.write_all(&v.to_be_bytes()).unwrap();
        }
        NbtTag::Float(v) => {
            buf.write_all(&v.to_be_bytes()).unwrap();
        }
        NbtTag::Double(v) => {
            buf.write_all(&v.to_be_bytes()).unwrap();
        }
        NbtTag::String(v) => {
            write_string(buf, v);
        }
        NbtTag::ByteArray(v) => {
            let len = v.len() as i32;
            buf.write_all(&len.to_be_bytes()).unwrap();
            for b in v {
                buf.push(*b as u8);
            }
        }
        NbtTag::IntArray(v) => {
            let len = v.len() as i32;
            buf.write_all(&len.to_be_bytes()).unwrap();
            for n in v {
                buf.write_all(&n.to_be_bytes()).unwrap();
            }
        }
        NbtTag::LongArray(v) => {
            let len = v.len() as i32;
            buf.write_all(&len.to_be_bytes()).unwrap();
            for n in v {
                buf.write_all(&n.to_be_bytes()).unwrap();
            }
        }
        NbtTag::List(items, element_type) => {
            buf.push(*element_type);
            let len = items.len() as i32;
            buf.write_all(&len.to_be_bytes()).unwrap();
            for item in items {
                write_value(buf, item);
            }
        }
        NbtTag::Compound(map) => {
            for (name, value) in map {
                write_tag(buf, name, value);
            }
            buf.push(TAG_END);
        }
    }
}

pub fn to_bytes(tag: &NbtTag, name: &str) -> Vec<u8> {
    let mut buf = Vec::new();
    write_tag(&mut buf, name, tag);
    buf
}

pub fn to_compound_bytes(compound: &NbtCompound) -> Vec<u8> {
    let mut buf = Vec::new();
    for (name, value) in compound {
        write_tag(&mut buf, name, value);
    }
    buf.push(TAG_END);
    buf
}
