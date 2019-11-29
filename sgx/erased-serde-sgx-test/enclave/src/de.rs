use std::prelude::v1::*;
use erased_serde::de::*;
use serde_json;
use std::fmt::Debug;

fn test_json<'de, T>(json: &'de [u8])
where
    T: serde::Deserialize<'de> + PartialEq + Debug,
{
    let expected: T = serde_json::from_slice(json).unwrap();

    // test borrowed trait object
    {
        let mut de = serde_json::Deserializer::from_slice(json);
        let de: &mut Deserializer = &mut Deserializer::erase(&mut de);
        assert_eq!(expected, deserialize::<T>(de).unwrap());
    }

    // test boxed trait object
    {
        let mut de = serde_json::Deserializer::from_slice(json);
        let mut de: Box<Deserializer> = Box::new(Deserializer::erase(&mut de));
        assert_eq!(expected, deserialize::<T>(&mut de).unwrap());
    }
}

//#[test]
pub fn test_value() {
    test_json::<serde_json::Value>(br#"["a", 1, [true], {"a": 1}]"#);
}

//#[test]
pub fn test_struct() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct S {
        f: usize,
    }

    test_json::<S>(br#"{"f":256}"#);
}

//#[test]
pub fn test_enum() {
    #[derive(Deserialize, PartialEq, Debug)]
    enum E {
        Unit,
        Newtype(bool),
        Tuple(bool, bool),
        Struct { t: bool, f: bool },
    }

    test_json::<E>(br#""Unit""#);
    test_json::<E>(br#"{"Newtype":true}"#);
    test_json::<E>(br#"{"Tuple":[true,false]}"#);
    test_json::<E>(br#"{"Struct":{"t":true,"f":false}}"#);
}

//#[test]
pub fn test_borrowed() {
    let bytes = br#""borrowed""#.to_owned();
    test_json::<&str>(&bytes);
}

//#[test]
pub fn assert_deserializer() {
    fn assert<'de, T: serde::Deserializer<'de>>() {}

    assert::<&mut Deserializer>();
    assert::<&mut (Deserializer + Send)>();
    assert::<&mut (Deserializer + Sync)>();
    assert::<&mut (Deserializer + Send + Sync)>();
    assert::<&mut (Deserializer + Sync + Send)>();

    assert::<Box<Deserializer>>();
    assert::<Box<Deserializer + Send>>();
    assert::<Box<Deserializer + Sync>>();
    assert::<Box<Deserializer + Send + Sync>>();
    assert::<Box<Deserializer + Sync + Send>>();
}
