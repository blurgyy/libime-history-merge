use crate::data::{History, Pool, Sentence};
use crate::{Error, Result};

const POOL_SIZE: &[usize] = &[128, 8192, 65536];

#[derive(Debug)]
struct WeightedHistory<'a> {
    sentences: &'a [Sentence],
    weight: usize,
}
impl<'a> WeightedHistory<'a> {
    fn next_exact(&mut self, size: usize) -> &'a [Sentence] {
        let size = std::cmp::min(size, self.sentences.len());
        log::trace!("Got {} sentences", size);
        let ret = &self.sentences[..size];
        self.sentences = &self.sentences[size..];
        ret
    }
}

fn gen_pool(
    target_size: usize,
    sorted_weighted_histories: &mut Vec<WeightedHistory>,
) -> Pool {
    let len = sorted_weighted_histories.len();
    match len {
        0 => Pool::default(),
        1 => Pool(
            sorted_weighted_histories[0]
                .next_exact(target_size)
                .to_owned(),
        ),
        _ => {
            let mut sentences: Vec<Sentence> =
                Vec::with_capacity(target_size);

            let sum_of_weights: usize =
                sorted_weighted_histories.iter().map(|x| x.weight).sum();

            // Iterate untill last entry
            for hist in &mut sorted_weighted_histories[..len - 1] {
                let target_size = ((target_size as f32)
                    * (hist.weight as f32 / sum_of_weights as f32))
                    as usize;
                log::trace!("Requesting {} sentences", target_size);
                sentences.append(&mut hist.next_exact(target_size).to_owned())
            }

            // Try to fill the vec's capacity
            let remaining_size = target_size - sentences.len();
            log::trace!("Requesting {} sentences", remaining_size);
            sentences.append(
                &mut sorted_weighted_histories[len - 1]
                    .next_exact(remaining_size)
                    .to_owned(),
            );

            assert!(
                sentences.len() <= target_size,
                "Generated vector of length {} but expected length to be less or equal to {}",
                sentences.len(),
                target_size,
            );
            Pool(sentences)
        }
    }
}

fn gen_pool_mixed(
    target_size: usize,
    sorted_weighted_histories: &mut Vec<WeightedHistory>,
) -> Pool {
    let len = sorted_weighted_histories.len();
    match len {
        0 => Pool::default(),
        1 => Pool(
            sorted_weighted_histories[0]
                .next_exact(target_size)
                .to_owned(),
        ),
        _ => {
            let mut sentences: Vec<Sentence> =
                Vec::with_capacity(target_size);

            let min_weight = sorted_weighted_histories
                .iter()
                .map(|x| x.weight)
                .min()
                .unwrap();
            let partition: Vec<usize> = sorted_weighted_histories
                .iter()
                .map(|x| {
                    (x.weight as f32 / min_weight as f32).ceil() as usize
                })
                .collect();
            let chunk_size: usize = partition.iter().sum();
            let whole_chunks: usize =
                (target_size as f32 / chunk_size as f32) as usize;

            for _ in 0..whole_chunks {
                for (i, part) in partition.iter().enumerate() {
                    sentences.append(
                        &mut sorted_weighted_histories[i]
                            .next_exact(*part)
                            .to_owned(),
                    );
                }
            }

            let mut remaining_size = target_size - sentences.len();
            for (i, part) in partition.iter().enumerate() {
                let next_size = std::cmp::min(*part, remaining_size);
                sentences.append(
                    &mut sorted_weighted_histories[i]
                        .next_exact(next_size)
                        .to_owned(),
                );
                remaining_size -= next_size;
                if remaining_size == 0 {
                    break;
                }
            }

            assert!(
                sentences.len() <= target_size,
                "Generated vector of length {} but expected length to be less or equal to {}",
                sentences.len(),
                target_size,
            );
            Pool(sentences)
        }
    }
}

/// Merge given `histories` with corresponding weights.
pub fn merge(
    histories: Vec<History>,
    weights: Vec<usize>,
    mixed: bool,
) -> Result<History> {
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
    if weights.iter().any(|w| *w == 0usize) {
        return Err(Error::LogicError("Zero weight encountered".to_string()));
    }

    let histories: Vec<Vec<Sentence>> =
        histories.iter().map(|hist| hist.get_sentences()).collect();

    let mut weighted_histories: Vec<WeightedHistory> = Vec::new();
    for (i, hist) in histories.iter().enumerate() {
        weighted_histories.push(WeightedHistory {
            sentences: &hist,
            weight: weights[i],
        })
    }

    // Sort decending
    weighted_histories
        .sort_by(|lhs, rhs| rhs.weight.partial_cmp(&lhs.weight).unwrap());

    let mut pools: Vec<Pool> = Vec::with_capacity(3);
    for size in POOL_SIZE {
        pools.push(if mixed {
            gen_pool_mixed(*size, &mut weighted_histories)
        } else {
            gen_pool(*size, &mut weighted_histories)
        });
    }

    Ok(History::new(pools))
}

// Author: Blurgy <gy@blurgy.xyz>
// Date:   Feb 04 2022, 21:26 [CST]
