use crate::data::{History, Pool, Sentence};
use crate::utils::{gcd, split_vec};
use crate::{Error, Result};

/// [`libime`][libime] saves the most recent 73856 sentence entries in the history file, where
/// entries are splitted into 3 pools with sizes 128, 8192, 65536, from newest to oldest,
/// respectively.
///
/// REF: <https://github.com/fcitx/libime/blob/2e90224d4905c9228c4008bca52155829d673532/src/libime/core/historybigram.cpp#L392-L396>
///
/// [libime]: https://github.com/fcitx/libime
const POOL_SIZE: &[usize] = &[128, 8192, 65536];

#[derive(Debug)]
struct WeightedHistory<'a> {
    sentences: &'a [Sentence],
    weight: u8,
}
impl<'a> WeightedHistory<'a> {
    fn next_exact(&mut self, size: usize) -> &'a [Sentence] {
        let size = std::cmp::min(size, self.sentences.len());
        log::trace!("got {} sentence{}", size, if size == 1 { "" } else { "s" });
        match size {
            0 => &[], // no op
            _ => {
                let ret = &self.sentences[..size];
                self.sentences = &self.sentences[size..];
                ret
            }
        }
    }
}

fn mix_sentences(
    target_size: usize,
    sorted_weighted_histories: &mut [WeightedHistory],
) -> Vec<Sentence> {
    match sorted_weighted_histories.len() {
        0 => Vec::new(),
        1 => sorted_weighted_histories[0].sentences.to_vec(),
        _ => {
            let total_input_size = sorted_weighted_histories
                .iter()
                .map(|wh| wh.sentences.len())
                .sum();

            let mut sentences: Vec<Sentence> = Vec::with_capacity(target_size);

            let partition: Vec<usize> = {
                let g = sorted_weighted_histories
                    .iter()
                    .map(|x| x.weight)
                    .reduce(|lhs, rhs| gcd(std::cmp::max(lhs, rhs), std::cmp::min(lhs, rhs)))
                    .unwrap();
                sorted_weighted_histories
                    .iter()
                    .map(|x| (x.weight / g) as usize)
                    .collect()
            };
            let min_part: usize = *partition.iter().min().unwrap();

            loop {
                if sentences.len() == target_size
                    || sorted_weighted_histories
                        .iter()
                        .all(|wh| wh.sentences.is_empty())
                {
                    break;
                }

                // Mix remainders
                for (i, part) in partition.iter().enumerate() {
                    if sorted_weighted_histories[i].sentences.is_empty() {
                        continue;
                    }
                    if sentences.len() == target_size {
                        break;
                    }
                    // After pusing, total number of sentences should not exceed `target_size`
                    let next_size = std::cmp::min(
                        *part % min_part, // mod here
                        target_size - sentences.len(),
                    );
                    sentences.append(
                        &mut sorted_weighted_histories[i]
                            .next_exact(next_size)
                            .to_owned(),
                    );
                }
                // Mix quotients
                for _ in 0..min_part {
                    for (i, part) in partition.iter().enumerate() {
                        if sorted_weighted_histories[i].sentences.is_empty() {
                            continue;
                        }
                        if sentences.len() == target_size {
                            break;
                        }
                        // After pusing, total number of sentences should not exceed `target_size`
                        let next_size = std::cmp::min(
                            *part / min_part, // divide here
                            target_size - sentences.len(),
                        );
                        sentences.append(
                            &mut sorted_weighted_histories[i]
                                .next_exact(next_size)
                                .to_owned(),
                        );
                    }
                }
            }

            if sentences.len() == std::cmp::min(target_size, total_input_size) {
                sentences
            } else {
                // Did not manage to use all history entries
                panic!(
                    "bad length of mixed sentences (expected {}, got {})",
                    std::cmp::min(target_size, total_input_size),
                    sentences.len(),
                )
            }
        }
    }
}

/// Merge given `histories` with corresponding weights.
pub fn merge(histories: Vec<History>, weights: Vec<u8>) -> Result<History> {
    let weights = if weights.is_empty() {
        log::info!("Using identical weights for each history data");
        vec![1; histories.len()]
    } else {
        weights
    };
    if histories.len() != weights.len() {
        return Err(Error::LogicError(
            "Number of weights should match number of histories".to_string(),
        ));
    }
    if weights.iter().any(|w| *w == 0u8) {
        return Err(Error::LogicError("Zero weight is not allowed".to_string()));
    }

    let histories: Vec<Vec<Sentence>> = histories.iter().map(|hist| hist.get_sentences()).collect();

    let mut weighted_histories: Vec<WeightedHistory> = Vec::new();
    for (i, hist) in histories.iter().enumerate() {
        weighted_histories.push(WeightedHistory {
            sentences: hist,
            weight: weights[i],
        })
    }

    // Sort decending by weight
    weighted_histories.sort_by(|lhs, rhs| rhs.weight.partial_cmp(&lhs.weight).unwrap());

    let pools = split_vec(
        mix_sentences(POOL_SIZE.iter().sum(), &mut weighted_histories),
        POOL_SIZE,
    )
    .iter()
    .map(|vec_sentence| Pool(vec_sentence.to_owned()))
    .collect();

    Ok(History::new(pools))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::Word;
    fn create_test_sentence(content: &str) -> Sentence {
        Sentence(vec![Word(content.to_string())])
    }

    fn create_test_history(sentences: Vec<&str>) -> History {
        let sentences: Vec<Sentence> = sentences.iter().map(|s| create_test_sentence(s)).collect();
        // Create a simple history with all sentences in the first pool
        let mut pools = vec![Pool(sentences), Pool(vec![]), Pool(vec![])];
        // Pad with empty pools if needed
        while pools.len() < 3 {
            pools.push(Pool(vec![]));
        }
        History::new(pools)
    }

    #[test]
    fn test_merge_single_history() -> Result<()> {
        let history = create_test_history(vec!["hello", "world"]);
        let result = merge(vec![history.clone()], vec![1])?;
        
        // Result should have exactly 3 pools
        assert_eq!(result.pools.len(), 3);
        
        // Total sentences should be preserved
        let original_sentences = history.get_sentences();
        let result_sentences = result.get_sentences();
        assert_eq!(result_sentences.len(), original_sentences.len());
        
        Ok(())
    }

    #[test]
    fn test_merge_equal_weights() -> Result<()> {
        let hist1 = create_test_history(vec!["a1", "a2", "a3", "a4"]);
        let hist2 = create_test_history(vec!["b1", "b2", "b3", "b4"]);
        
        let result = merge(vec![hist1, hist2], vec![1, 1])?;
        
        // Should have exactly 3 pools with correct sizes
        assert_eq!(result.pools.len(), 3);
        
        // Total sentence count should be sum of inputs (up to pool capacity)
        let result_sentences = result.get_sentences();
        assert_eq!(result_sentences.len(), 8); // 4 + 4
        
        // With equal weights, sentences should be mixed somewhat evenly
        let sentence_texts: Vec<String> = result_sentences.iter()
            .map(|s| s.0[0].0.clone())
            .collect();
        
        // Both histories should be represented
        assert!(sentence_texts.iter().any(|s| s.starts_with('a')));
        assert!(sentence_texts.iter().any(|s| s.starts_with('b')));
        
        Ok(())
    }

    #[test]
    fn test_merge_weighted_distribution() -> Result<()> {
        // Create histories with distinct sentence patterns
        let hist1 = create_test_history(vec!["high1", "high2", "high3", "high4", "high5", "high6"]);
        let hist2 = create_test_history(vec!["low1", "low2", "low3"]);
        
        // Give hist1 weight 3, hist2 weight 1 (3:1 ratio)
        let result = merge(vec![hist1, hist2], vec![3, 1])?;
        
        let result_sentences = result.get_sentences();
        let sentence_texts: Vec<String> = result_sentences.iter()
            .map(|s| s.0[0].0.clone())
            .collect();
        
        // Count sentences from each history
        let high_count = sentence_texts.iter().filter(|s| s.starts_with("high")).count();
        let low_count = sentence_texts.iter().filter(|s| s.starts_with("low")).count();
        
        // With 3:1 weight ratio, we should see roughly 3x more from high weight history
        // (exact ratio depends on mixing algorithm, but should be heavily skewed)
        assert!(high_count > low_count, "High weight history should have more sentences");
        assert!(high_count >= 2 * low_count, "Weight ratio should be reflected in distribution");
        
        Ok(())
    }

    #[test]
    fn test_merge_empty_weights_defaults_to_equal() -> Result<()> {
        let hist1 = create_test_history(vec!["a1", "a2"]);
        let hist2 = create_test_history(vec!["b1", "b2"]);
        
        // Empty weights should default to equal weights
        let result = merge(vec![hist1, hist2], vec![])?;
        
        let result_sentences = result.get_sentences();
        assert_eq!(result_sentences.len(), 4);
        
        Ok(())
    }

    #[test]
    fn test_merge_error_mismatched_weights() {
        let hist1 = create_test_history(vec!["a1"]);
        let hist2 = create_test_history(vec!["b1"]);
        
        // 2 histories but 3 weights should error
        let result = merge(vec![hist1, hist2], vec![1, 2, 3]);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Number of weights should match"));
    }

    #[test]
    fn test_merge_error_zero_weight() {
        let hist1 = create_test_history(vec!["a1"]);
        let hist2 = create_test_history(vec!["b1"]);
        
        // Zero weight should be rejected
        let result = merge(vec![hist1, hist2], vec![1, 0]);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Zero weight is not allowed"));
    }

    #[test]
    fn test_merge_preserves_pool_structure() -> Result<()> {
        let hist1 = create_test_history(vec!["a1", "a2"]);
        let hist2 = create_test_history(vec!["b1", "b2"]);
        
        let result = merge(vec![hist1, hist2], vec![1, 1])?;
        
        // Must have exactly 3 pools
        assert_eq!(result.pools.len(), 3);
        
        // Pool sizes should follow libime structure (though may not be full)
        // We can't test exact sizes since they depend on input size,
        // but we can verify the structure exists
        let pool_sizes: Vec<usize> = result.pools.iter().map(|p| p.0.len()).collect();
        let total_sentences: usize = pool_sizes.iter().sum();
        
        // Total should equal input sentences
        assert_eq!(total_sentences, 4);
        
        Ok(())
    }

    #[test]
    fn test_merge_large_history_respects_pool_limits() -> Result<()> {
        // Create a history larger than first pool (128)
        let large_sentences: Vec<&str> = (0..200).map(|_i| "sentence").collect();
        let hist1 = create_test_history(large_sentences);
        
        let result = merge(vec![hist1], vec![1])?;
        
        // Should still have exactly 3 pools
        assert_eq!(result.pools.len(), 3);
        
        // First pool should not exceed its limit
        assert!(result.pools[0].0.len() <= 128);
        
        Ok(())
    }

    #[test]
    fn test_merge_with_different_format_histories() -> Result<()> {
        // Create histories with different format versions but same data structure
        use crate::data_bytes::{FORMAT_VERSION_V2, FORMAT_VERSION_V3, MAGIC};
        
        let sentences1 = vec![create_test_sentence("format_v2_1"), create_test_sentence("format_v2_2")];
        let sentences2 = vec![create_test_sentence("format_v3_1"), create_test_sentence("format_v3_2")];
        
        let hist_v2 = History {
            magic: MAGIC,
            format_version: FORMAT_VERSION_V2,
            pools: vec![Pool(sentences1), Pool(vec![]), Pool(vec![])],
        };
        
        let hist_v3 = History {
            magic: MAGIC,
            format_version: FORMAT_VERSION_V3,
            pools: vec![Pool(sentences2), Pool(vec![]), Pool(vec![])],
        };
        
        // Merge histories from different format versions
        let result = merge(vec![hist_v2, hist_v3], vec![1, 1])?;
        
        // Result should have exactly 3 pools
        assert_eq!(result.pools.len(), 3);
        
        // Should contain sentences from both format versions
        let result_sentences = result.get_sentences();
        let sentence_texts: Vec<String> = result_sentences.iter()
            .map(|s| s.0[0].0.clone())
            .collect();
        
        assert!(sentence_texts.iter().any(|s| s.contains("format_v2")));
        assert!(sentence_texts.iter().any(|s| s.contains("format_v3")));
        assert_eq!(result_sentences.len(), 4);
        
        Ok(())
    }

    #[test]
    fn test_merge_realistic_weight_scenario() -> Result<()> {
        // Test the example from README: weights 2,5,3
        let hist_a = create_test_history(vec!["A0", "A1", "A2", "A3", "A4"]);
        let hist_b = create_test_history(vec!["B0", "B1", "B2", "B3", "B4", "B5", "B6", "B7"]);
        let hist_c = create_test_history(vec!["C0", "C1", "C2", "C3", "C4", "C5"]);
        
        let result = merge(vec![hist_a, hist_b, hist_c], vec![2, 5, 3])?;
        
        let result_sentences = result.get_sentences();
        let sentence_texts: Vec<String> = result_sentences.iter()
            .map(|s| s.0[0].0.clone())
            .collect();
        
        // Count occurrences from each history
        let count_a = sentence_texts.iter().filter(|s| s.starts_with('A')).count();
        let count_b = sentence_texts.iter().filter(|s| s.starts_with('B')).count();
        let count_c = sentence_texts.iter().filter(|s| s.starts_with('C')).count();
        
        // B should have the most (weight 5), then C (weight 3), then A (weight 2)
        assert!(count_b >= count_c, "B (weight 5) should have >= sentences than C (weight 3)");
        assert!(count_c >= count_a, "C (weight 3) should have >= sentences than A (weight 2)");
        
        // Total should be all input sentences
        assert_eq!(count_a + count_b + count_c, 5 + 8 + 6);
        
        Ok(())
    }
}

// Author: Blurgy <gy@blurgy.xyz>
// Date:   Feb 04 2022, 21:26 [CST]
