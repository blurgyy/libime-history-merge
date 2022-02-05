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
        log::trace!("Got {} sentences", size);
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

fn gen_mixed_sentences(
    sorted_weighted_histories: &mut [WeightedHistory],
) -> Vec<Sentence> {
    let len = sorted_weighted_histories.len();
    let target_size = POOL_SIZE.iter().sum();
    match len {
        0 => Vec::new(),
        1 => sorted_weighted_histories[0].sentences.to_vec(),
        _ => {
            let mut sentences: Vec<Sentence> =
                Vec::with_capacity(target_size);

            let partition: Vec<usize> = {
                let g = sorted_weighted_histories
                    .iter()
                    .map(|x| x.weight)
                    .reduce(|lhs, rhs| {
                        gcd(std::cmp::max(lhs, rhs), std::cmp::min(lhs, rhs))
                    })
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
                    sentences.append(
                        &mut sorted_weighted_histories[i]
                            .next_exact(*part % (min_part as usize)) // mod here
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
                        sentences.append(
                            &mut sorted_weighted_histories[i]
                                .next_exact(*part / (min_part as usize)) // divide here
                                .to_owned(),
                        );
                    }
                }
            }

            assert!(
                sentences.len() <= target_size,
                "Generated vector of length {} but expected length to be less than or equal to {}",
                sentences.len(),
                target_size,
            );

            sentences
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
        return Err(Error::LogicError(
            "Zero weight is not allowed".to_string(),
        ));
    }

    let histories: Vec<Vec<Sentence>> =
        histories.iter().map(|hist| hist.get_sentences()).collect();

    let mut weighted_histories: Vec<WeightedHistory> = Vec::new();
    for (i, hist) in histories.iter().enumerate() {
        weighted_histories.push(WeightedHistory {
            sentences: hist,
            weight: weights[i],
        })
    }

    // Sort decending by weight
    weighted_histories
        .sort_by(|lhs, rhs| rhs.weight.partial_cmp(&lhs.weight).unwrap());

    let pools =
        split_vec(gen_mixed_sentences(&mut weighted_histories), POOL_SIZE)
            .iter()
            .map(|vec_sentence| Pool(vec_sentence.to_owned()))
            .collect();

    Ok(History::new(pools))
}

// Author: Blurgy <gy@blurgy.xyz>
// Date:   Feb 04 2022, 21:26 [CST]
