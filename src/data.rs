use std::fmt::Display;

use serde::{
    de::Visitor,
    ser::{SerializeSeq, SerializeStruct},
    Deserialize, Serialize, Serializer,
};

use crate::de::{SequenceVisitor, StringVisitor};

pub const MAGIC: u32 = 0x000FC315;
pub const FORMAT_VERSION: u32 = 0x02;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Word(
    /// Use `String` here because it is read from dumped `user.history` so it
    /// must be valid UTF-8.
    pub String,
);
impl Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl Serialize for Word {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> std::result::Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for Word {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Word(deserializer.deserialize_string(StringVisitor)?))
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Sentence(pub Vec<Word>);
impl Display for Sentence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .0
                .iter()
                .map(|w| w.to_string())
                .collect::<Vec<_>>()
                .join(" "),
        )
    }
}

impl Serialize for Sentence {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> std::result::Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut ser = serializer.serialize_seq(Some(self.0.len()))?;
        for word in &self.0 {
            ser.serialize_element(&word)?;
        }
        ser.end()
    }
}

impl<'de> Deserialize<'de> for Sentence {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Sentence(
            deserializer.deserialize_seq(SequenceVisitor::new())?,
        ))
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Session(pub Vec<Sentence>);
impl Display for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .0
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

impl Serialize for Session {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> std::result::Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut ser = serializer.serialize_seq(Some(self.0.len()))?;
        for sentence in &self.0 {
            ser.serialize_element(&sentence)?;
        }
        ser.end()
    }
}

impl<'de> Deserialize<'de> for Session {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Session(
            deserializer.deserialize_seq(SequenceVisitor::new())?,
        ))
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct History {
    pub magic: u32,
    pub format_version: u32,
    pub sessions: Vec<Session>,
}
impl Display for History {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "Magic: {}\nFormat version: {}\n{}",
            &self.magic,
            &self.format_version,
            &self
                .sessions
                .iter()
                .map(|sess| sess.to_string())
                .collect::<Vec<_>>()
                .join("\n"),
        ))
    }
}

impl Serialize for History {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> std::result::Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut ser = serializer.serialize_struct("HistoryData", 0)?;
        ser.serialize_field("magic", &self.magic)?;
        ser.serialize_field("format_version", &self.format_version)?;
        for session in &self.sessions {
            ser.serialize_field("session", &session)?;
        }
        ser.end()
    }
}

impl<'de> Deserialize<'de> for History {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct HistoryVisitor;
        impl<'de> Visitor<'de> for HistoryVisitor {
            type Value = History;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                formatter.write_str("4 bytes of u32, then another 4 bytes of u32, then an array of sessions (bincode)")
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                // Load magic bytes
                let mut magic_bytes: Vec<u8> = Vec::new();
                for _ in 0..4 {
                    magic_bytes.push(seq.next_element()?.unwrap());
                }
                let magic =
                    u32::from_be_bytes(magic_bytes.try_into().unwrap());
                if magic != MAGIC {
                    return Err(serde::de::Error::custom(format!(
                        "Invalid history magic (expected 0x{:08x}, found 0x{:08x})",
                        MAGIC,
                        magic,
                    )));
                }

                let mut format_version_bytes: Vec<u8> = Vec::new();
                for _ in 0..4 {
                    format_version_bytes.push(seq.next_element()?.unwrap());
                }
                let format_version = u32::from_be_bytes(
                    format_version_bytes.try_into().unwrap(),
                );
                if format_version != FORMAT_VERSION {
                    return Err(serde::de::Error::custom(format!(
                        "Invalid format version (expected 0x{:08x}, found 0x{:08x})",
                        FORMAT_VERSION,
                        format_version,
                    )));
                }

                let sessions = SequenceVisitor::new().visit_seq(seq)?;

                Ok(History {
                    magic,
                    format_version,
                    sessions,
                })
            }
        }

        Ok(deserializer.deserialize_struct("", &[""], HistoryVisitor)?)
    }
}

// Author: Blurgy <gy@blurgy.xyz>
// Date:   Feb 03 2022, 11:45 [CST]
