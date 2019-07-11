use std::prelude::v1::*;
use erased_serde::ser::*;
use serde_json;

fn test_json<T>(t: T)
where
    T: serde::Serialize,
{
    let expected = serde_json::to_vec(&t).unwrap();

    // test borrowed trait object
    {
        let obj: &Serialize = &t;

        let mut buf = Vec::new();

        {
            let mut ser = serde_json::Serializer::new(&mut buf);
            let ser: &mut Serializer = &mut Serializer::erase(&mut ser);

            obj.erased_serialize(ser).unwrap();
        }

        assert_eq!(buf, expected);
    }

    // test boxed trait object
    {
        let obj: Box<Serialize> = Box::new(t);

        let mut buf = Vec::new();

        {
            let mut ser = serde_json::Serializer::new(&mut buf);
            let mut ser: Box<Serializer> = Box::new(Serializer::erase(&mut ser));

            obj.erased_serialize(&mut ser).unwrap();
        }

        assert_eq!(buf, expected);
    }
}

//#[test]
pub fn test_vec() {
    test_json(vec!["a", "b"]);
}

//#[test]
pub fn test_struct() {
    #[derive(Serialize)]
    struct S {
        f: usize,
    }

    test_json(S { f: 256 });
}

//#[test]
pub fn test_enum() {
    #[derive(Serialize)]
    enum E {
        Unit,
        Newtype(bool),
        Tuple(bool, bool),
        Struct { t: bool, f: bool },
    }

    test_json(E::Unit);
    test_json(E::Newtype(true));
    test_json(E::Tuple(true, false));
    test_json(E::Struct { t: true, f: false });
}

//#[test]
pub fn assert_serialize() {
    fn assert<T: serde::Serialize>() {}

    assert::<&Serialize>();
    assert::<&(Serialize + Send)>();
    assert::<&(Serialize + Sync)>();
    assert::<&(Serialize + Send + Sync)>();
    assert::<&(Serialize + Sync + Send)>();
    assert::<Vec<&Serialize>>();
    assert::<Vec<&(Serialize + Send)>>();

    assert::<Box<Serialize>>();
    assert::<Box<Serialize + Send>>();
    assert::<Box<Serialize + Sync>>();
    assert::<Box<Serialize + Send + Sync>>();
    assert::<Box<Serialize + Sync + Send>>();
    assert::<Vec<Box<Serialize>>>();
    assert::<Vec<Box<Serialize + Send>>>();
}

//#[test]
pub fn assert_serializer() {
    fn assert<T: serde::Serializer>() {}

    assert::<&mut Serializer>();
    assert::<&mut (Serializer + Send)>();
    assert::<&mut (Serializer + Sync)>();
    assert::<&mut (Serializer + Send + Sync)>();
    assert::<&mut (Serializer + Sync + Send)>();
}
