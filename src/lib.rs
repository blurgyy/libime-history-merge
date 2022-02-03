pub mod data;

mod de;
mod error;
mod ser;

pub use de::{from_bytes, Deserializer};
pub use error::{Error, Result};
pub use ser::{to_bytes, Serializer};

#[cfg(test)]
mod serde_tests {
    use pretty_assertions::assert_eq;

    use crate::{
        data::{History, Sentence, Session, Word},
        error::Result,
        from_bytes, to_bytes,
    };

    #[test]
    fn word() -> Result<()> {
        let word = Word("音乐".to_string());
        assert_eq!(word, from_bytes(&to_bytes(&word)?)?);
        Ok(())
    }

    #[test]
    fn sentence() -> Result<()> {
        let sentence = Sentence(vec![
            Word("音乐".to_string()),
            Word("好听".to_string()),
            Word("🎵".to_string()),
        ]);
        assert_eq!(sentence, from_bytes(&to_bytes(&sentence)?)?);
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
        assert_eq!(session, from_bytes(&to_bytes(&session)?)?);
        Ok(())
    }

    #[test]
    fn history() -> Result<()> {
        let words = vec![
            Word("🎵".to_string()),
            Word("音乐".to_string()),
            Word("💿".to_string()),
            Word("好听".to_string()),
        ];
        let sentence = Sentence(words);
        let session = Session(vec![sentence]);
        let history = History {
            magic: crate::data::MAGIC,
            format_version: crate::data::FORMAT_VERSION,
            sessions: vec![session],
        };
        assert_eq!(history, from_bytes(&to_bytes(&history)?)?);
        Ok(())
    }
}

// Author: Blurgy <gy@blurgy.xyz>
// Date:   Feb 03 2022, 11:45 [CST]
