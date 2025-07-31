use std::{fmt::Display, fs::Permissions, os::unix::prelude::PermissionsExt, path::Path};

use serde::{
    ser::{SerializeSeq, SerializeStruct},
    Serialize, Serializer,
};

use crate::{
    data_bytes::{FORMAT_VERSION_V2, MAGIC},
    to_bytes, Error, Result,
};

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Word(
    /// Use `String` here because it is read from dumped `user.history` so it must be valid UTF-8.
    pub String,
);

impl Word {
    /// Checks if this word is an empty string
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

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

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Sentence(pub Vec<Word>);

impl Sentence {
    /// Checks if all words inside this sentence are empty words
    pub fn is_empty(&self) -> bool {
        self.0.iter().all(Word::is_empty)
    }
}

impl Display for Sentence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .0
                .iter()
                .map(|w| w.to_string())
                .filter(|w| !w.is_empty())
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

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Pool(pub Vec<Sentence>);

impl Display for Pool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .0
                .iter()
                .map(|s| s.to_string())
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

impl Serialize for Pool {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> std::result::Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        let mut ser = serializer.serialize_seq(Some(self.0.len()))?;
        let oldest_first: Vec<Sentence> = self.0.iter().rev().cloned().collect();
        for sentence in &oldest_first {
            ser.serialize_element(&sentence)?;
        }
        ser.end()
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct History {
    pub magic: u32,
    pub format_version: u32,
    pub pools: Vec<Pool>,
}

impl History {
    pub fn new(pools: Vec<Pool>) -> Self {
        History {
            magic: MAGIC,
            format_version: FORMAT_VERSION_V2,
            pools,
        }
    }

    pub fn save<P>(&self, p: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        std::fs::write(p.as_ref(), to_bytes(&self)?)?;
        std::fs::set_permissions(p.as_ref(), Permissions::from_mode(0o600))?;
        Ok(())
    }

    pub fn load<P>(p: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let content = std::fs::read(&p)?;
        match History::load_from_bytes(&content) {
            Ok(hist) => Ok(hist),
            _ => match History::load_from_text(&content) {
                Ok(hist) => Ok(hist),
                _ => Err(Error::DeserializeError(format!(
                    "Could not load history from path '{}', tried binary and plain text",
                    p.as_ref().display(),
                ))),
            },
        }
    }

    /// Collects all sentences into one array.
    pub fn get_sentences(&self) -> Vec<Sentence> {
        let mut vvs: Vec<Vec<Sentence>> = self.pools.iter().map(|pool| pool.0.to_owned()).collect();
        let mut ret = Vec::new();
        for vs in &mut vvs {
            ret.append(vs)
        }
        ret
    }
}

impl Default for History {
    fn default() -> Self {
        History {
            magic: MAGIC,
            format_version: FORMAT_VERSION_V2,
            pools: vec![Pool::default(); 3], /* 3 pools from current
                                              * version of libime's saved
                                              * history data */
        }
    }
}

impl Display for History {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .pools
                .iter()
                .map(|pool| pool.to_string())
                .filter(|pool| !pool.is_empty())
                .collect::<Vec<_>>()
                .join("\n"),
        )
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
        for pool in &self.pools {
            ser.serialize_field("pool", &pool)?;
        }
        ser.end()
    }
}
