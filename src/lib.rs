pub mod data;

mod de;
mod error;
mod merging;
mod ser;
mod utils;

pub use de::{from_bytes, BytesDeserializer};
pub use error::{Error, Result};
pub use merging::merge;
pub use ser::{to_bytes, Serializer};

#[cfg(test)]
mod serde_tests {
    use pretty_assertions::assert_eq;

    use crate::{
        data::{History, Pool, Sentence, Word},
        data::{HistoryFromBytes, PoolFromBytes, SentenceFromBytes, WordFromBytes},
        error::Result,
        from_bytes, to_bytes,
    };

    #[test]
    fn word() -> Result<()> {
        let word = Word("音乐".to_string());
        assert_eq!(
            word,
            Word::from(from_bytes::<WordFromBytes>(&to_bytes(&word)?)?)
        );
        Ok(())
    }

    #[test]
    fn sentence() -> Result<()> {
        let sentence = Sentence(vec![
            Word("音乐".to_string()),
            Word("好听".to_string()),
            Word("🎵".to_string()),
        ]);
        assert_eq!(
            sentence,
            Sentence::from(from_bytes::<SentenceFromBytes>(&to_bytes(&sentence)?)?)
        );
        Ok(())
    }

    #[test]
    fn pool() -> Result<()> {
        let words = vec![
            Word("音乐".to_string()),
            Word("🎵".to_string()),
            Word("好听".to_string()),
        ];
        let sentence = Sentence(words);
        let pool = Pool(vec![sentence]);
        assert_eq!(
            pool,
            Pool::from(from_bytes::<PoolFromBytes>(&to_bytes(&pool)?)?)
        );
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
        let pool = Pool(vec![sentence]);
        let history = History {
            magic: crate::data::MAGIC,
            format_version: crate::data::FORMAT_VERSION,
            pools: vec![pool],
        };
        assert_eq!(
            history,
            History::from(from_bytes::<HistoryFromBytes>(&to_bytes(&history)?)?)
        );
        Ok(())
    }
}

// Author: Blurgy <gy@blurgy.xyz>
// Date:   Feb 03 2022, 11:45 [CST]
