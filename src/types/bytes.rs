use serde::{
    de::{Error, Unexpected, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt;
use faster_hex::hex_decode;

/// Raw bytes wrapper
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Bytes(pub Vec<u8>);

impl<T: Into<Vec<u8>>> From<T> for Bytes {
    fn from(data: T) -> Self {
        Bytes(data.into())
    }
}

impl Serialize for Bytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut serialized = "0x".to_owned();
        serialized.push_str(&hex::encode(&self.0));
        serializer.serialize_str(serialized.as_ref())
    }
}

impl<'a> Deserialize<'a> for Bytes {
    fn deserialize<D>(deserializer: D) -> Result<Bytes, D::Error>
    where
        D: Deserializer<'a>,
    {
        deserializer.deserialize_identifier(BytesVisitor)
    }
}

struct BytesVisitor;

impl<'a> Visitor<'a> for BytesVisitor {
    type Value = Bytes;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a 0x-prefixed hex-encoded vector of bytes")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if value.len() >= 2 && &value[0..2] == "0x" {
            let mut bytes = vec![0_u8; (value.len() - 2) >> 1];
            if bytes.len() > 0 {
                hex_decode(value[2..].as_bytes(), &mut bytes).map_err(|e| Error::custom(format!("Invalid hex: {}", e)))?;
            }
            Ok(Bytes(bytes))
        } else {
            Err(Error::invalid_value(Unexpected::Str(value), &"0x prefix"))
        }
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: Error,
    {
        self.visit_str(value.as_ref())
    }
}
