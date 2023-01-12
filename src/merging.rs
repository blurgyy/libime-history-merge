use crate::data::{History, Pool, Sentence};
use crate::utils::{gcd, split_vec};
use crate::{Error, Result};

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

// Author: Blurgy <gy@blurgy.xyz>
// Date:   Feb 04 2022, 21:26 [CST]
