pub mod data;
pub mod data_bytes;
pub mod data_text;

mod de;
mod de_bytes;
mod de_text;
mod error;
mod merging;
mod ser;
mod utils;

pub use de_bytes::{from_bytes, BytesDeserializer};
pub use de_text::{from_text, TextDeserializer};
pub use error::{Error, Result};
pub use merging::merge;
pub use ser::{to_bytes, Serializer};

#[cfg(test)]
mod serde_tests {
    use pretty_assertions::assert_eq;

    use crate::{
        data::{History, Pool, Sentence, Word},
        data_bytes::{HistoryFromBytes, PoolFromBytes, SentenceFromBytes, WordFromBytes},
        from_bytes, to_bytes, Result,
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
            magic: crate::data_bytes::MAGIC,
            format_version: crate::data_bytes::FORMAT_VERSION_OLD,
            pools: vec![pool],
        };
        assert_eq!(
            history,
            History::from(from_bytes::<HistoryFromBytes>(&to_bytes(&history)?)?)
        );
        Ok(())
    }
}

#[cfg(test)]
mod text_bytes_coherency {
    use pretty_assertions::assert_eq;

    use crate::{
        data::{Sentence, Word},
        data_bytes::{SentenceFromBytes, WordFromBytes},
        data_text::{SentenceFromText, WordFromText},
        from_bytes, from_text, Result,
    };

    #[test]
    fn word() -> Result<()> {
        let word_bytes = vec![0, 0, 0, 6, 233, 159, 179, 228, 185, 144];
        let word_str = "音乐";
        let word = Word(word_str.to_string());
        let word_from_bytes = Word::from(from_bytes::<WordFromBytes>(&word_bytes)?);
        let word_from_text = Word::from(from_text::<WordFromText>(word_str.as_bytes())?);
        assert_eq!(word, word_from_bytes);
        assert_eq!(word, word_from_text);
        Ok(())
    }

    #[test]
    fn sentence() -> Result<()> {
        let sentence_bytes = vec![
            0, 0, 0, 2, 0, 0, 0, 6, 233, 159, 179, 228, 185, 144, 0, 0, 0, 6, 229, 165, 189, 229,
            144, 172,
        ];
        let sentence_str = "音乐 好听";
        let sentence = Sentence(
            sentence_str
                .split(' ')
                .map(String::from)
                .map(Word)
                .collect(),
        );
        let sentence_from_bytes = Sentence::from(from_bytes::<SentenceFromBytes>(&sentence_bytes)?);
        let sentence_from_text =
            Sentence::from(from_text::<SentenceFromText>(sentence_str.as_bytes())?);
        assert_eq!(sentence, sentence_from_bytes);
        assert_eq!(sentence, sentence_from_text);
        Ok(())
    }
}

// Author: Blurgy <gy@blurgy.xyz>
// Date:   Feb 03 2022, 11:45 [CST]
