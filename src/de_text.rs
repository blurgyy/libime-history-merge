use std::marker::PhantomData;

use serde::de::{self, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer};

use crate::{Error, Result};

pub struct TextDeserializer<'de> {
    input: &'de [u8],
}

impl<'de> TextDeserializer<'de> {
    pub fn new(input: &'de [u8]) -> Self {
        Self { input }
    }

    pub fn ended(&self) -> bool {
        self.input.len() == 0
    }

    /// Returns the byte under cursor without consuming it
    pub fn peek_byte(&self) -> Result<u8> {
        if self.input.len() > 0 {
            Ok(self.input[0])
        } else {
            Err(Error::EofError)
        }
    }

    /// Consumes next `len` bytes from input and return it
    pub fn next_exact_bytes(&mut self, len: usize) -> Result<&[u8]> {
        if len > self.input.len() {
            Err(Error::EofError)
        } else {
            let slce = &self.input[..len];
            self.input = &self.input[len..];
            Ok(slce)
        }
    }

    /// Consumes until one of `candidate_chars` is occurred in input and return the visited bytes
    pub fn pop_until(&mut self, candidate_chars: &[u8]) -> Result<&[u8]> {
        let mut len: usize = 0;
        while len < self.input.len() {
            if candidate_chars.iter().any(|ch| ch == &self.input[len]) {
                break;
            }
            len += 1;
        }
        let slce = &self.input[..len];
        self.input = &self.input[len..];
        Ok(slce)
    }

    /// Load next word, words are delimetered by space or new line character
    pub fn next_word(&mut self) -> Result<String> {
        let ret = String::from_utf8(self.pop_until(&[b' ', b'\n'])?.into())?;
        Ok(ret)
    }
}

impl<'de, 'a> Deserializer<'de> for &'a mut TextDeserializer<'de> {
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

    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
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

    /// For WordFromText
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_string(self.next_word()?)
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

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    /// For SentenceFromText
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(SpaceSeparated::new(self))
    }

    fn deserialize_tuple<V>(self, _len: usize, _visitor: V) -> Result<V::Value>
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
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
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

struct SpaceSeparated<'a, 'de: 'a> {
    de: &'a mut TextDeserializer<'de>,
    first: bool,
}

impl<'a, 'de> SpaceSeparated<'a, 'de> {
    pub fn new(de: &'a mut TextDeserializer<'de>) -> Self {
        Self { de, first: true }
    }
}

impl<'a, 'de> SeqAccess<'de> for SpaceSeparated<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.de.peek_byte().is_err() {
            return Ok(None);
        }
        if !self.first && self.de.next_exact_bytes(1)? != &[b' '] {
            // return Err(Error::DeserializeError(
            //     "Expected space character".to_string(),
            // ));
            return Ok(None);
        }
        self.first = false;
        seed.deserialize(&mut *self.de).map(Some)
    }
}

pub(crate) struct SpaceSeparatedVisitor<'de, ElementType>(PhantomData<&'de ElementType>)
where
    ElementType: Deserialize<'de>;

impl<'de, ElementType> SpaceSeparatedVisitor<'de, ElementType>
where
    ElementType: Deserialize<'de>,
{
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<'de, ElementType> Visitor<'de> for SpaceSeparatedVisitor<'de, ElementType>
where
    ElementType: Deserialize<'de>,
{
    type Value = Vec<ElementType>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("space (' ') separated UTF-8 strings")
    }

    fn visit_seq<A>(self, mut seq: A) -> std::result::Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
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
