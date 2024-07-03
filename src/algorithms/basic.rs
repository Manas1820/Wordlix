use crate::{Attempt, Score, Solver};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BasicAlgorithm {
    remaining_words: HashMap<&'static str, usize>,
}

impl BasicAlgorithm {
    pub fn new() -> BasicAlgorithm {
        let n_gram_frequency = include_str!("./../../dataset/wordle_words_x_n_gram.txt");
        let remaining_words = n_gram_frequency
            .lines()
            .map(|line| {
                let mut parts = line.split_whitespace();
                let word = parts.next().unwrap();
                let freq = parts.next().unwrap().parse().unwrap();
                (word, freq)
            })
            .collect();
        BasicAlgorithm { remaining_words }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Candidate {
    word: &'static str,
    count: usize,
    similarity: f64,
}

impl Solver for BasicAlgorithm {
    fn solve(&mut self, history: &[Attempt]) -> String {
        let mut best_word: Option<Candidate> = None;
        for (&word, &count) in &self.remaining_words {
            let similarity = 0.0;
            if let Some(possible_ans) = best_word {
                if similarity > possible_ans.similarity {
                    best_word = Some(Candidate {
                        word,
                        count,
                        similarity,
                    });
                }
            } else {
                best_word = Some(Candidate {
                    word,
                    count,
                    similarity,
                });
            }
        }
        return best_word.unwrap().word.to_string();
    }
}
