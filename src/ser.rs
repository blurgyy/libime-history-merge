use serde::{ser, Serialize};

use crate::{Error, Result};

pub struct Serializer {
    output: Vec<u8>,
}

pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut serer = Serializer { output: Vec::new() };
    value.serialize(&mut serer)?;
    Ok(serer.output)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_u32(self, v: u32) -> Result<()> {
        // The `htonl` method from the Rust crate `socket` contains only 1
        // line:
        //
        // ```rust
        // pub fn htonl(hostlong: u32) -> u32 {
        //     hostlong.to_be()
        // }
        // ```
        //
        // So just convert it into big endian bytes here to avoid introducing
        // another dependency.
        //
        // Ref: <https://docs.rs/socket/0.0.7/src/socket/lib.rs.html#69-71>
        self.output.append(&mut v.to_be_bytes().into());
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        let v_bytes = v.as_bytes();
        self.serialize_u32(v_bytes.len() as u32)?;
        self.output.append(&mut v_bytes.into());
        Ok(())
    }

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.serialize_u32(len.unwrap() as u32)?;
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!()
    }

    fn serialize_map(
        self,
        _len: Option<usize>,
    ) -> Result<Self::SerializeMap> {
        unimplemented!()
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        unimplemented!()
    }

    fn serialize_i128(self, v: i128) -> Result<Self::Ok> {
        let _ = v;
        Err(ser::Error::custom("i128 is not supported"))
    }

    fn serialize_u128(self, v: u128) -> Result<Self::Ok> {
        let _ = v;
        Err(ser::Error::custom("u128 is not supported"))
    }

    fn collect_seq<I>(self, _iter: I) -> Result<Self::Ok>
    where
        I: IntoIterator,
        <I as IntoIterator>::Item: Serialize,
    {
        unimplemented!()
    }

    fn collect_map<K, V, I>(self, _iter: I) -> Result<Self::Ok>
    where
        K: Serialize,
        V: Serialize,
        I: IntoIterator<Item = (K, V)>,
    {
        unimplemented!()
    }

    fn collect_str<T: ?Sized>(self, _value: &T) -> Result<Self::Ok>
    where
        T: std::fmt::Display,
    {
        unimplemented!()
    }

    fn is_human_readable(&self) -> bool {
        true
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        // Do nothing
        Ok(())
    }
}
impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}
impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}
impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}
impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<()>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}
impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        // Do nothing
        Ok(())
    }
}
impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        data::{History, Sentence, Session, Word},
        to_bytes, Result,
    };

    #[test]
    fn word() -> Result<()> {
        let word = Word("音乐".to_string());
        let expected_word_bytes =
            vec![0, 0, 0, 6, 233, 159, 179, 228, 185, 144];
        assert_eq!(to_bytes(&word)?, expected_word_bytes);
        Ok(())
    }

    #[test]
    fn sentence() -> Result<()> {
        let sentence = Sentence(vec![
            Word("音乐".to_string()),
            Word("好听".to_string()),
            Word("🎵".to_string()),
        ]);
        let expected_sentence_bytes = vec![
            0, 0, 0, 3, 0, 0, 0, 6, 233, 159, 179, 228, 185, 144, 0, 0, 0, 6,
            229, 165, 189, 229, 144, 172, 0, 0, 0, 4, 240, 159, 142, 181,
        ];
        assert_eq!(to_bytes(&sentence)?, expected_sentence_bytes);
        Ok(())
    }

    #[test]
    fn session() -> Result<()> {
        let words = vec![
            Word("音乐".to_string()),
            Word("🎵".to_string()),
            Word("好听".to_string()),
        ];
        let sentence = Sentence(words);
        let session = Session(vec![sentence]);
        let expected_session_bytes = vec![
            0, 0, 0, 1, 0, 0, 0, 3, 0, 0, 0, 6, 233, 159, 179, 228, 185, 144,
            0, 0, 0, 4, 240, 159, 142, 181, 0, 0, 0, 6, 229, 165, 189, 229,
            144, 172,
        ];
        assert_eq!(to_bytes(&session)?, expected_session_bytes);
        Ok(())
    }

    #[test]
    fn history() -> Result<()> {
        let mut sentences: Vec<Sentence> = Vec::new();
        let mut sessions: Vec<Session> = Vec::new();

        let words = vec![
            Word("🎵".to_string()),
            Word("音乐".to_string()),
            Word("💿".to_string()),
        ];
        sentences.push(Sentence(words));
        let words = vec![Word("好听".to_string())];
        sentences.push(Sentence(words));
        sessions.push(Session(sentences.to_owned()));
        let words = vec![
            Word("🎵".to_string()),
            Word("音乐".to_string()),
            Word("💿".to_string()),
            Word("好听".to_string()),
        ];
        sentences.push(Sentence(words));
        sessions.push(Session(sentences));
        let history = History {
            magic: 998244353,
            format_version: 0x3f3f3f3f,
            sessions,
        };
        let expected_history_bytes = vec![
            59, 128, 0, 1, 63, 63, 63, 63, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0,
            4, 240, 159, 142, 181, 0, 0, 0, 6, 233, 159, 179, 228, 185, 144,
            0, 0, 0, 4, 240, 159, 146, 191, 0, 0, 0, 1, 0, 0, 0, 6, 229, 165,
            189, 229, 144, 172, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 4, 240, 159,
            142, 181, 0, 0, 0, 6, 233, 159, 179, 228, 185, 144, 0, 0, 0, 4,
            240, 159, 146, 191, 0, 0, 0, 1, 0, 0, 0, 6, 229, 165, 189, 229,
            144, 172, 0, 0, 0, 4, 0, 0, 0, 4, 240, 159, 142, 181, 0, 0, 0, 6,
            233, 159, 179, 228, 185, 144, 0, 0, 0, 4, 240, 159, 146, 191, 0,
            0, 0, 6, 229, 165, 189, 229, 144, 172,
        ];
        assert_eq!(to_bytes(&history)?, expected_history_bytes);
        Ok(())
    }
}

// Author: Blurgy <gy@blurgy.xyz>
// Date:   Feb 03 2022, 12:18 [CST]
