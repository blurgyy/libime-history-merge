use serde::{de::Visitor, Deserialize};

use crate::{
    data::{History, Pool, Sentence, Word},
    de::StringVisitor,
    de_bytes::ByteSequenceVisitor,
    from_bytes, Result,
};

pub const MAGIC: u32 = 0x000FC315;
pub const FORMAT_VERSION: u32 = 0x02;

impl History {
    /// Load a history object from a [`libime`][libime]-compatible user history blob.  The format
    /// is described as follows:
    ///
    /// * The blob begins with a 4-byte **file magic** [`00 0f c3 15`][MAGIC], then a 4-byte
    ///   **format version** [`00 00 00 02`][FORMAT_VERSION], followed by 3 **pool**s.
    /// * Each **pool** begins with a 4-byte **size** specifying the number of **sentence**(s)
    ///   inside this **pool**, followed by the **pool**'s **sentence**(s).
    /// * Each **sentence** begins with a 4-byte **size** specifying the number of **word**(s)
    ///   inside this **sentence**, followed by the **sentence**'s **word**(s).
    /// * Each **word** begins with a 4-byte **size** specifying the number of
    ///   [nibbles][nibble-wiki] that the UTF-8 encoded representation of this word occupies,
    ///   followed by the **word**'s [nibbles][nibble-wiki].
    ///
    /// The **file magic**, **format version**, **size**s are all [big-endian][endianness-wiki]
    /// encoded.
    ///
    /// [file-magic]: crate::data_bytes::MAGIC
    /// [format-version]: crate::data_bytes::FORMAT_VERSION
    /// [libime]: <https://github.com/fcitx/libme>
    /// [endianness-wiki]: <https://en.wikipedia.org/wiki/Endianness>
    /// [nibble-wiki]: <https://en.wikipedia.org/wiki/Nibble>
    pub(crate) fn load_from_bytes(content: &[u8]) -> Result<Self> {
        let ret: HistoryFromBytes = from_bytes(content)?;
        Ok(History::from(ret))
    }
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct WordFromBytes(
    /// Use `String` here because it is read from dumped `user.history` so it must be valid UTF-8.
    pub String,
);

impl From<WordFromBytes> for Word {
    fn from(wfb: WordFromBytes) -> Self {
        Word(wfb.0)
    }
}

impl<'de> Deserialize<'de> for WordFromBytes {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(WordFromBytes(
            deserializer.deserialize_string(StringVisitor)?,
        ))
    }
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SentenceFromBytes(pub Vec<WordFromBytes>);

impl From<SentenceFromBytes> for Sentence {
    fn from(sfb: SentenceFromBytes) -> Self {
        Self(sfb.0.into_iter().map(Word::from).collect())
    }
}

impl<'de> Deserialize<'de> for SentenceFromBytes {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(SentenceFromBytes(
            deserializer.deserialize_seq(ByteSequenceVisitor::new())?,
        ))
    }
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct PoolFromBytes(pub Vec<SentenceFromBytes>);

impl From<PoolFromBytes> for Pool {
    fn from(pfb: PoolFromBytes) -> Self {
        Pool(pfb.0.into_iter().map(Sentence::from).collect())
    }
}

impl<'de> Deserialize<'de> for PoolFromBytes {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let newest_first: Vec<SentenceFromBytes> =
            (deserializer.deserialize_seq(ByteSequenceVisitor::new())? as Vec<SentenceFromBytes>)
                .iter()
                .rev()
                .cloned()
                .collect();
        Ok(PoolFromBytes(newest_first))
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HistoryFromBytes {
    pub magic: u32,
    pub format_version: u32,
    pub pools: Vec<PoolFromBytes>,
}

impl From<HistoryFromBytes> for History {
    fn from(hfb: HistoryFromBytes) -> Self {
        History {
            magic: hfb.magic,
            format_version: hfb.format_version,
            pools: hfb.pools.into_iter().map(Pool::from).collect(),
        }
    }
}

impl<'de> Deserialize<'de> for HistoryFromBytes {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct HistoryVisitor;
        impl<'de> Visitor<'de> for HistoryVisitor {
            type Value = HistoryFromBytes;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(
                    "4 bytes of u32, then another 4 bytes of u32, then an array of pools (bincode)",
                )
            }
            fn visit_seq<A>(self, mut seq: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                // Load magic bytes
                let mut magic_bytes: Vec<u8> = Vec::new();
                for _ in 0..4 {
                    magic_bytes.push(seq.next_element()?.unwrap());
                }
                let magic = u32::from_be_bytes(magic_bytes.try_into().unwrap());
                if magic != MAGIC {
                    return Err(serde::de::Error::custom(format!(
                        "Invalid history magic (expected 0x{:08x}, got 0x{:08x})",
                        MAGIC, magic,
                    )));
                }

                let mut format_version_bytes: Vec<u8> = Vec::new();
                for _ in 0..4 {
                    format_version_bytes.push(seq.next_element()?.unwrap());
                }
                let format_version = u32::from_be_bytes(format_version_bytes.try_into().unwrap());
                if format_version != FORMAT_VERSION {
                    return Err(serde::de::Error::custom(format!(
                        "Invalid format version (expected 0x{:08x}, got 0x{:08x})",
                        FORMAT_VERSION, format_version,
                    )));
                }

                let pools = ByteSequenceVisitor::new().visit_seq(seq)?;

                Ok(HistoryFromBytes {
                    magic,
                    format_version,
                    pools,
                })
            }
        }

        deserializer.deserialize_struct("", &[""], HistoryVisitor)
    }
}

// Author: Blurgy <gy@blurgy.xyz>
// Date:   Feb 03 2022, 11:45 [CST]
