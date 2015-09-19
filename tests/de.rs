extern crate serde_cbor;

use std::collections::HashMap;

use serde_cbor::{Value, ObjectKey, error, de};

#[test]
fn test_string1() {
    let value: error::Result<Value> = de::from_slice(&[0x66, 0x66, 0x6f, 0x6f, 0x62, 0x61, 0x72]);
    assert_eq!(value.unwrap(), Value::String("foobar".to_owned()));
}

#[test]
fn test_string2() {
    let value: error::Result<Value> = de::from_slice(&[0x71, 0x49, 0x20, 0x6d, 0x65, 0x74, 0x20, 0x61, 0x20, 0x74, 0x72, 0x61, 0x76,
        0x65, 0x6c, 0x6c, 0x65, 0x72]);
    assert_eq!(value.unwrap(), Value::String("I met a traveller".to_owned()));
}

#[test]
fn test_string3() {
    let slice = b"\x78\x2fI met a traveller from an antique land who said";
    let value: error::Result<Value> = de::from_slice(slice);
    assert_eq!(value.unwrap(), Value::String("I met a traveller from an antique land who said".to_owned()));
}

#[test]
fn test_byte_string() {
    let value: error::Result<Value> = de::from_slice(&[0x46, 0x66, 0x6f, 0x6f, 0x62, 0x61, 0x72]);
    assert_eq!(value.unwrap(), Value::Bytes(b"foobar".to_vec()));
}

#[test]
fn test_numbers1() {
    let value: error::Result<Value> = de::from_slice(&[0x00]);
    assert_eq!(value.unwrap(), Value::U64(0));
}

#[test]
fn test_numbers2() {
    let value: error::Result<Value> = de::from_slice(&[0x1a, 0x00, 0xbc, 0x61, 0x4e]);
    assert_eq!(value.unwrap(), Value::U64(12345678));
}

#[test]
fn test_numbers3() {
    let value: error::Result<Value> = de::from_slice(&[0x39, 0x07, 0xde]);
    assert_eq!(value.unwrap(), Value::I64(-2015));
}

#[test]
fn test_bool() {
    let value: error::Result<Value> = de::from_slice(b"\xf4");
    assert_eq!(value.unwrap(), Value::Bool(false));
}

#[test]
fn test_list1() {
    let value: error::Result<Value> = de::from_slice(b"\x83\x01\x02\x03");
    assert_eq!(value.unwrap(), Value::Array(vec![Value::U64(1), Value::U64(2), Value::U64(3)]));
}

#[test]
fn test_list2() {
    let value: error::Result<Value> = de::from_slice(b"\x82\x01\x82\x02\x81\x03");
    assert_eq!(value.unwrap(), Value::Array(vec![Value::U64(1), Value::Array(vec![Value::U64(2), Value::Array(vec![Value::U64(3)])])]));
}

#[test]
fn test_object() {
    let value: error::Result<Value> = de::from_slice(b"\xa5aaaAabaBacaCadaDaeaE");
    let mut object = HashMap::new();
    object.insert(ObjectKey::String("a".to_owned()), Value::String("A".to_owned()));
    object.insert(ObjectKey::String("b".to_owned()), Value::String("B".to_owned()));
    object.insert(ObjectKey::String("c".to_owned()), Value::String("C".to_owned()));
    object.insert(ObjectKey::String("d".to_owned()), Value::String("D".to_owned()));
    object.insert(ObjectKey::String("e".to_owned()), Value::String("E".to_owned()));
    assert_eq!(value.unwrap(), Value::Object(object));
}

#[test]
fn test_indefinite_object() {
    let value: error::Result<Value> = de::from_slice(b"\xbfaa\x01ab\x9f\x02\x03\xff\xff");
    let mut object = HashMap::new();
    object.insert(ObjectKey::String("a".to_owned()), Value::U64(1));
    object.insert(ObjectKey::String("b".to_owned()), Value::Array(vec![Value::U64(2), Value::U64(3)]));
    assert_eq!(value.unwrap(), Value::Object(object));
}

#[test]
fn test_indefinite_list() {
    let value: error::Result<Value> = de::from_slice(b"\x9f\x01\x02\x03\xff");
    assert_eq!(value.unwrap(), Value::Array(vec![Value::U64(1), Value::U64(2), Value::U64(3)]));
}

#[test]
fn test_indefinite_string() {
    let value: error::Result<Value> = de::from_slice(b"\x7f\x65Mary \x64Had \x62a \x67Little \x64Lamb\xff");
    assert_eq!(value.unwrap(), Value::String("Mary Had a Little Lamb".to_owned()));
}

#[test]
fn test_indefinite_byte_string() {
    let value: error::Result<Value> = de::from_slice(b"\x5f\x42\x01\x23\x42\x45\x67\xff");
    assert_eq!(value.unwrap(), Value::Bytes(b"\x01#Eg".to_vec()));
}

#[test]
fn test_float() {
    let value: error::Result<Value> = de::from_slice(b"\xfa\x47\xc3\x50\x00");
    assert_eq!(value.unwrap(), Value::F64(100000.0));
}


#[test]
fn test_self_describing() {
    let value: error::Result<Value> = de::from_slice(&[0xd9, 0xd9, 0xf7, 0x66, 0x66, 0x6f, 0x6f, 0x62, 0x61, 0x72]);
    assert_eq!(value.unwrap(), Value::String("foobar".to_owned()));
}


#[test]
fn test_f16() {
    let mut x: Value = de::from_slice(&[0xf9, 0x41, 0x00]).unwrap();
    assert_eq!(x, Value::F64(2.5));
    x = de::from_slice(&[0xf9, 0x41, 0x90]).unwrap();
    assert_eq!(x, Value::F64(2.78125));
    x = de::from_slice(&[0xf9, 0x50, 0x90]).unwrap();
    assert_eq!(x, Value::F64(36.5));
    x = de::from_slice(&[0xf9, 0xd0, 0x90]).unwrap();
    assert_eq!(x, Value::F64(-36.5));
}
