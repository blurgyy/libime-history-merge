use serde::Deserialize;

use crate::{
    data::{History, Pool, Sentence, Word},
    //de_text::{SequenceVisitor, StringVisitor},
    de::StringVisitor,
    de_text::{SpaceSeparatedVisitor, TextDeserializer},
    Result,
};

impl History {
    /// Load a history object from a newline-separated text buffer.  Each line should be a
    /// space-separated collection of words.  Empty lines are ignored.
    pub fn load_from_text(content: &[u8]) -> Result<Self> {
        let mut deserializer = TextDeserializer::new(content);
        let mut sentences: Vec<SentenceFromText> = Vec::new();
        while !deserializer.ended() {
            let sft = SentenceFromText::deserialize(&mut deserializer)?;
            sentences.push(sft);
        }
        let sentences = sentences
            .into_iter()
            .map(Sentence::from)
            .filter(|sentence| !sentence.is_empty())
            .collect();
        let pools = vec![Pool(sentences)];
        Ok(History::new(pools))
    }
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct WordFromText(
    /// Use `String` here because it is read from dumped `user.history` so it must be valid UTF-8.
    pub String,
);

impl From<WordFromText> for Word {
    fn from(wft: WordFromText) -> Self {
        Word(wft.0)
    }
}

impl<'de> Deserialize<'de> for WordFromText {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(WordFromText(
            deserializer.deserialize_string(StringVisitor)?,
        ))
    }
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SentenceFromText(pub Vec<WordFromText>);

impl From<SentenceFromText> for Sentence {
    fn from(sft: SentenceFromText) -> Self {
        Sentence(sft.0.into_iter().map(Word::from).collect())
    }
}

impl<'de> Deserialize<'de> for SentenceFromText {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(SentenceFromText(
            deserializer.deserialize_seq(SpaceSeparatedVisitor::new())?,
        ))
    }
}
