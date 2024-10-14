use prost::bytes::{Buf, BufMut};
use prost::{Message, Name};
use serde::{ser, Deserialize, Deserializer, Serialize, Serializer};

/// Works as a wrapper for Vec<u8> when working with structures that have an undefined/unknown type.
#[derive(:: serde :: Serialize, :: serde :: Deserialize, Clone, PartialEq, Debug, Default)]
pub struct GenericData(pub Vec<u8>);

impl Message for GenericData {
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: BufMut,
    {
        buf.put_slice(self.0.as_slice());
    }

    fn merge_field<B>(
        &mut self,
        tag: u32,
        _wire_type: prost::encoding::WireType,
        buf: &mut B,
        _ctx: prost::encoding::DecodeContext,
    ) -> Result<(), prost::DecodeError>
    where
        B: Buf,
    {
        if tag == 1 {
            self.0.push(10u8);
            while buf.has_remaining() {
                self.0.push(buf.get_u8());
            }
            Ok(())
        } else {
            Err(prost::DecodeError::new("invalid tag"))
        }
    }

    fn encoded_len(&self) -> usize {
        self.0.len()
    }

    fn clear(&mut self) {
        self.0.clear();
    }
}

impl Name for GenericData {
    const NAME: &'static str = "";
    const PACKAGE: &'static str = "";

    fn full_name() -> String {
        format!("{}{}", Self::PACKAGE, Self::NAME)
    }
}

// An improved any type that allows you to implement typing directly into it
#[derive(Clone, PartialEq, Serialize, Deserialize, Message)]
pub struct Any<T: Message + PartialEq + Default> {
    #[prost(string, tag = "1")]
    pub type_url: String,
    #[prost(message, required, tag = "2")]
    pub value: T,
}

impl<T: Message + Name + PartialEq + Default> Any<T> {
    pub fn new(value: T) -> Self {
        Self {
            type_url: T::full_name(),
            value,
        }
    }

    pub fn generic(value: T) -> Any<GenericData> {
        Any {
            type_url: T::full_name(),
            value: GenericData(value.encode_to_vec()),
        }
    }
}

// Deserialize data shown inside Any<T> and return T
pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Message + PartialEq + Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    Any::<T>::deserialize(deserializer).map(|any| any.value)
}

// Serialize T into Any<T>
pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize + Name,
{
    let mut state = Serializer::serialize_struct(serializer, "Any", 2)?;
    ser::SerializeStruct::serialize_field(&mut state, "type_url", &T::full_name())?;
    ser::SerializeStruct::serialize_field(&mut state, "value", value)?;
    ser::SerializeStruct::end(state)
}

pub mod option {
    use crate::any::Any;
    use prost::{Message, Name};
    use serde::de::DeserializeOwned;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    // Deserialize data shown inside Option<Any<T>> and return Option<T>
    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        T: Message + PartialEq + Default + DeserializeOwned,
        D: Deserializer<'de>,
    {
        Option::<Any<T>>::deserialize(deserializer).map(|option| option.map(|any| any.value))
    }

    pub fn generic_deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        T: Message + PartialEq + Default + DeserializeOwned,
        D: Deserializer<'de>,
    {
        Option::<T>::deserialize(deserializer)
    }

    // Serialize Option<T> into Option<Any<T>>
    pub fn serialize<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize + Message + Name + PartialEq + Default + Clone,
    {
        match value {
            None => Serializer::serialize_none(serializer),
            Some(value) => Serializer::serialize_some(serializer, &Any::new(value.clone())),
        }
    }

    pub fn generic_serialize<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize + Message + PartialEq + Default,
    {
        match value {
            None => Serializer::serialize_none(serializer),
            Some(value) => Serializer::serialize_some(serializer, &value),
        }
    }
}

pub mod vec {
    use crate::any::Any;
    use prost::{Message, Name};
    use serde::de::DeserializeOwned;
    use serde::ser::SerializeSeq;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    // Deserialize data shown inside Vec<Any<T>> and return Vec<T>
    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Vec<T>, D::Error>
    where
        T: Message + PartialEq + Default + DeserializeOwned,
        D: Deserializer<'de>,
    {
        let list = Vec::<Any<T>>::deserialize(deserializer)?;

        let mut res = vec![];
        for any in list {
            res.push(any.value)
        }

        Ok(res)
    }

    pub fn generic_deserialize<'de, T, D>(deserializer: D) -> Result<Vec<T>, D::Error>
    where
        T: Message + PartialEq + Default + DeserializeOwned,
        D: Deserializer<'de>,
    {
        Vec::<T>::deserialize(deserializer)
    }

    // Serialize Vec<T> into Vec<Any<T>>
    pub fn serialize<S, T>(value: &Vec<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize + Message + Name + PartialEq + Default + Clone,
    {
        let mut seq = Serializer::serialize_seq(serializer, Some(value.len()))?;
        for element in value {
            seq.serialize_element(&Any::new(element.clone()))?;
        }
        seq.end()
    }

    pub fn generic_serialize<S, T>(value: &Vec<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize + Message + PartialEq + Default,
    {
        let mut seq = Serializer::serialize_seq(serializer, Some(value.len()))?;
        for element in value {
            seq.serialize_element(&element)?;
        }
        seq.end()
    }
}

#[cfg(test)]
mod test {
    use crate::any::{Any, GenericData};
    use prost::{Message, Name};
    use serde::de::DeserializeOwned;
    use serde::{Deserialize, Serialize};
    use serde_test::{assert_tokens, Token};

    #[derive(:: serde :: Serialize, :: serde :: Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, :: prost :: Message)]
    struct ExternalStructTest<
        A: Default + Message + Name + Send + Sync + Serialize + DeserializeOwned + PartialEq + Clone,
    > {
        #[prost(message, optional, tag = "1")]
        #[serde(
            serialize_with = "crate::any::option::serialize",
            deserialize_with = "crate::any::option::deserialize"
        )]
        pub optional: Option<A>,
    }

    impl<
            A: Default
                + Message
                + Name
                + Send
                + Sync
                + Serialize
                + DeserializeOwned
                + PartialEq
                + Clone,
        > Name for ExternalStructTest<A>
    {
        const NAME: &'static str = "AnyValue";
        const PACKAGE: &'static str = "test";

        fn full_name() -> String {
            format!("{}.{}", Self::PACKAGE, Self::NAME)
        }
    }

    #[derive(:: serde :: Serialize, :: serde :: Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, :: prost :: Message)]
    struct BuildTest<
        A: Default + Message + Name + Send + Sync + Serialize + DeserializeOwned + PartialEq + Clone,
        C: Default + Message + Name + Send + Sync + Serialize + DeserializeOwned + PartialEq + Clone,
    > {
        #[prost(message, optional, tag = "1")]
        #[serde(
            serialize_with = "crate::any::option::generic_serialize",
            deserialize_with = "crate::any::option::generic_deserialize"
        )]
        pub optional: Option<ExternalStructTest<C>>,
        #[prost(message, repeated, tag = "2")]
        #[serde(
            serialize_with = "crate::any::vec::serialize",
            deserialize_with = "crate::any::vec::deserialize"
        )]
        pub repeated: Vec<A>,
    }

    #[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
    struct Test {
        #[prost(message, optional, tag = "1")]
        #[serde(
            serialize_with = "crate::any::option::serialize",
            deserialize_with = "crate::any::option::deserialize"
        )]
        pub optional: Option<AnyValue>,
        #[prost(message, repeated, tag = "2")]
        #[serde(
            serialize_with = "crate::any::vec::serialize",
            deserialize_with = "crate::any::vec::deserialize"
        )]
        pub repeated: Vec<AnyValue>,
    }

    #[derive(Clone, PartialEq, Message, Serialize, Deserialize)]
    struct AnyValue {
        #[prost(string, tag = "1")]
        pub value: String,
        #[prost(uint64, tag = "2")]
        pub number: u64,
    }

    impl Name for AnyValue {
        const NAME: &'static str = "AnyValue";
        const PACKAGE: &'static str = "test";

        fn full_name() -> String {
            format!("{}.{}", Self::PACKAGE, Self::NAME)
        }
    }

    #[test]
    fn generic_equal_any() {
        let val = AnyValue {
            value: "testinnnng".to_string(),
            number: 33,
        };

        dbg!(val.encode_to_vec());
        dbg!(GenericData(val.encode_to_vec()).encode_to_vec());

        let real_any = Any::new(val.clone());
        let bytes_any = Any::generic(val);

        let mut real_encoded = real_any.encode_to_vec();
        let mut bytes_encoded = bytes_any.encode_to_vec();

        assert_eq!(real_encoded, bytes_encoded);

        let real_decoded = Any::<AnyValue>::decode(bytes_encoded.as_slice()).unwrap();
        let bytes_decoded = Any::<GenericData>::decode(real_encoded.as_slice()).unwrap();

        assert_eq!(real_decoded.type_url, bytes_decoded.type_url);
        assert_eq!(real_decoded.value.encode_to_vec(), bytes_decoded.value.0);
    }

    #[test]
    fn test_ser_de() {
        let test = Test {
            optional: Some(AnyValue {
                value: "testing".to_string(),
                number: 10,
            }),
            repeated: vec![
                AnyValue {
                    value: "idx0".to_string(),
                    number: 0,
                },
                AnyValue {
                    value: "idx1".to_string(),
                    number: 1,
                },
            ],
        };

        assert_tokens(
            &test,
            &[
                Token::Struct {
                    name: "Test",
                    len: 2,
                },
                Token::Str("optional"),
                Token::Some,
                Token::Struct {
                    name: "Any",
                    len: 2,
                },
                Token::Str("type_url"),
                Token::String("test.AnyValue"),
                Token::Str("value"),
                Token::Struct {
                    name: "AnyValue",
                    len: 2,
                },
                Token::Str("value"),
                Token::String("testing"),
                Token::Str("number"),
                Token::U64(10),
                Token::StructEnd,
                Token::StructEnd,
                Token::Str("repeated"),
                Token::Seq { len: Some(2) },
                Token::Struct {
                    name: "Any",
                    len: 2,
                },
                Token::Str("type_url"),
                Token::String("test.AnyValue"),
                Token::Str("value"),
                Token::Struct {
                    name: "AnyValue",
                    len: 2,
                },
                Token::Str("value"),
                Token::String("idx0"),
                Token::Str("number"),
                Token::U64(0),
                Token::StructEnd,
                Token::StructEnd,
                Token::Struct {
                    name: "Any",
                    len: 2,
                },
                Token::Str("type_url"),
                Token::String("test.AnyValue"),
                Token::Str("value"),
                Token::Struct {
                    name: "AnyValue",
                    len: 2,
                },
                Token::Str("value"),
                Token::String("idx1"),
                Token::Str("number"),
                Token::U64(1),
                Token::StructEnd,
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }
}
