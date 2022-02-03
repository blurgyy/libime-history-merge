use std::marker::PhantomData;

use serde::de::{self, SeqAccess, Visitor};
use serde::Deserialize;

use crate::{Error, Result};

pub struct Deserializer<'de> {
    input: &'de [u8],
}

impl<'de> Deserializer<'de> {
    pub fn from_bytes(input: &'de [u8]) -> Self {
        Deserializer { input }
    }

    // Parsing helpers
    /// Consumes next `len` bytes from input and return it
    pub fn next_exact_bytes(&mut self, len: usize) -> Result<&[u8]> {
        if len > self.input.len() {
            Err(Error::Eof)
        } else {
            let slce = &self.input[..len];
            self.input = &self.input[len..];
            Ok(slce)
        }
    }
    /// Consumes 1 bytes of data from input and parse it into a u8
    pub fn parse_u8(&mut self) -> Result<u8> {
        let ret = u8::from_be_bytes(self.next_exact_bytes(1)?.try_into()?);
        Ok(ret)
    }

    /// Consumes 4 bytes of data from input and parse it into a u32
    pub fn parse_u32(&mut self) -> Result<u32> {
        let ret = u32::from_be_bytes(self.next_exact_bytes(4)?.try_into()?);
        Ok(ret)
    }

    /// Consumes next `len` bytes from input and parse it into a UTF-8 String
    pub fn parse_string(&mut self, len: usize) -> Result<String> {
        let ret = String::from_utf8(self.next_exact_bytes(len)?.into())?;
        Ok(ret)
    }
}

pub fn from_bytes<'a, T>(b: &'a Vec<u8>) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserer = Deserializer::from_bytes(b);
    let t = T::deserialize(&mut deserer)?;

    Ok(t)
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.parse_u8()?)
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.parse_u32()?)
    }

    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let byte_len = self.parse_u32()? as usize;
        visitor.visit_string(self.parse_string(byte_len)?)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let seq_len = self.parse_u32()? as usize;
        visitor.visit_seq(ElementSequence::new(self, seq_len))
    }

    fn deserialize_tuple<V>(
        self,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // visitor.visit_map(HistoryHelperStruct::new(self))
        unimplemented!()
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(ByteSequence::new(self))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }
}

struct ElementSequence<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    remaining_elems: usize,
}
impl<'a, 'de> ElementSequence<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>, total_length: usize) -> Self {
        Self {
            de,
            remaining_elems: total_length,
        }
    }
}

impl<'de, 'a> SeqAccess<'de> for ElementSequence<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        // Check if the sequence has reached an end
        if self.remaining_elems == 0 {
            Ok(None)
        } else {
            self.remaining_elems -= 1;
            seed.deserialize(&mut *self.de).map(Some)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.remaining_elems)
    }
}

struct ByteSequence<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}
impl<'a, 'de> ByteSequence<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        Self { de }
    }
}

impl<'de, 'a> SeqAccess<'de> for ByteSequence<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        // Check if the sequence has reached an end
        if self.size_hint().unwrap() == 0 {
            Ok(None)
        } else {
            seed.deserialize(&mut *self.de).map(Some)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.de.input.len())
    }
}

pub(crate) struct U32Visitor;
impl<'de> Visitor<'de> for U32Visitor {
    type Value = u32;
    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        formatter.write_str("An unsigned 32-bit int")
    }
    fn visit_u32<E>(self, v: u32) -> std::result::Result<u32, E>
    where
        E: serde::de::Error,
    {
        Ok(v)
    }
}

pub(crate) struct StringVisitor;
impl<'de> Visitor<'de> for StringVisitor {
    type Value = String;
    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        formatter.write_str("A UTF-8 encoded string")
    }
    fn visit_string<E>(self, v: String) -> std::result::Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v)
    }
}

pub(crate) struct SequenceVisitor<'de, ElementType: Deserialize<'de>>(
    PhantomData<&'de ElementType>,
);
impl<'de, ElementType: Deserialize<'de>> SequenceVisitor<'de, ElementType> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}
impl<'de, ElementType> Visitor<'de> for SequenceVisitor<'de, ElementType>
where
    ElementType: Deserialize<'de>,
{
    type Value = Vec<ElementType>;
    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        formatter.write_str(
            "A byte array begining with a 4-byte uint32 value `l`, then `l` chunks of data, length of each chunk depends on impl of ElementType",
        )
    }
    fn visit_seq<A>(
        self,
        mut seq: A,
    ) -> std::result::Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut ret: Vec<ElementType> = Vec::new();
        loop {
            let value = seq.next_element()?;
            match value {
                Some(value) => ret.push(value),
                None => break Ok(ret),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        data::{History, Sentence, Session, Word},
        from_bytes, Result,
    };

    #[test]
    fn word() -> Result<()> {
        let word_bytes = vec![0, 0, 0, 6, 233, 159, 179, 228, 185, 144];
        let expected_word = Word("音乐".to_string());
        assert_eq!(from_bytes::<Word>(&word_bytes)?, expected_word);
        Ok(())
    }

    #[test]
    fn sentence() -> Result<()> {
        let sentence_bytes = vec![
            0, 0, 0, 3, 0, 0, 0, 6, 233, 159, 179, 228, 185, 144, 0, 0, 0, 6,
            229, 165, 189, 229, 144, 172, 0, 0, 0, 4, 240, 159, 142, 181,
        ];
        let expected_sentence = Sentence(vec![
            Word("音乐".to_string()),
            Word("好听".to_string()),
            Word("🎵".to_string()),
        ]);
        assert_eq!(
            from_bytes::<Sentence>(&sentence_bytes)?,
            expected_sentence,
        );
        Ok(())
    }

    #[test]
    fn session() -> Result<()> {
        let session_bytes = vec![
            0, 0, 0, 1, 0, 0, 0, 3, 0, 0, 0, 6, 233, 159, 179, 228, 185, 144,
            0, 0, 0, 4, 240, 159, 142, 181, 0, 0, 0, 6, 229, 165, 189, 229,
            144, 172,
        ];
        let expected_session = Session(vec![Sentence(vec![
            Word("音乐".to_string()),
            Word("🎵".to_string()),
            Word("好听".to_string()),
        ])]);
        assert_eq!(from_bytes::<Session>(&session_bytes)?, expected_session);
        Ok(())
    }

    #[test]
    fn history() -> Result<()> {
        let history_bytes = vec![
            0x00, 0x0f, 0xc3, 0x15, 0x00, 0x00, 0x00, 0x02, 0, 0, 0, 2, 0, 0,
            0, 3, 0, 0, 0, 4, 240, 159, 142, 181, 0, 0, 0, 6, 233, 159, 179,
            228, 185, 144, 0, 0, 0, 4, 240, 159, 146, 191, 0, 0, 0, 1, 0, 0,
            0, 6, 229, 165, 189, 229, 144, 172, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0,
            0, 4, 240, 159, 142, 181, 0, 0, 0, 6, 233, 159, 179, 228, 185,
            144, 0, 0, 0, 4, 240, 159, 146, 191, 0, 0, 0, 1, 0, 0, 0, 6, 229,
            165, 189, 229, 144, 172, 0, 0, 0, 4, 0, 0, 0, 4, 240, 159, 142,
            181, 0, 0, 0, 6, 233, 159, 179, 228, 185, 144, 0, 0, 0, 4, 240,
            159, 146, 191, 0, 0, 0, 6, 229, 165, 189, 229, 144, 172,
        ];
        dbg!(crate::data::MAGIC);
        dbg!(crate::data::FORMAT_VERSION);
        let expected_history = History {
            magic: crate::data::MAGIC,
            format_version: crate::data::FORMAT_VERSION,
            sessions: vec![
                Session(vec![
                    Sentence(vec![
                        Word("🎵".to_string()),
                        Word("音乐".to_string()),
                        Word("💿".to_string()),
                    ]),
                    Sentence(vec![Word("好听".to_string())]),
                ]),
                Session(vec![
                    Sentence(vec![
                        Word("🎵".to_string()),
                        Word("音乐".to_string()),
                        Word("💿".to_string()),
                    ]),
                    Sentence(vec![Word("好听".to_string())]),
                    Sentence(vec![
                        Word("🎵".to_string()),
                        Word("音乐".to_string()),
                        Word("💿".to_string()),
                        Word("好听".to_string()),
                    ]),
                ]),
            ],
        };
        assert_eq!(from_bytes::<History>(&history_bytes)?, expected_history);
        Ok(())
    }
}

// Author: Blurgy <gy@blurgy.xyz>
// Date:   Feb 03 2022, 14:49 [CST]
