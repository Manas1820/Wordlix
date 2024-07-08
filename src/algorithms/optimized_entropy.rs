use super::HighestEntropyAlgorithm;
use crate::{Attempt, Score, Solver};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct WordScore {
    pub word: &'static str,
    pub score: f64,
}

impl WordScore {
    pub fn new(word: &'static str, score: f64) -> Self {
        Self { word, score }
    }
}

// Maximum entropy calculation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptimizedEntropyAlgorithm {
    pub available_options: HashMap<&'static str, u32>,
}

impl OptimizedEntropyAlgorithm {
    pub fn new() -> Self {
        let n_gram_frequency = include_str!("./../../dataset/wordle_words_x_n_gram.txt");
        let available_options = n_gram_frequency
            .lines()
            .map(|line| {
                let mut parts = line.split_whitespace();
                let word = parts.next().unwrap();
                let freq = parts.next().unwrap().parse().unwrap();
                (word, freq)
            })
            .collect();

        Self { available_options }
    }

    pub fn calculate_score(
        word: &'static str,
        available_options: HashMap<&'static str, u32>,
    ) -> WordScore {
        let mut entropy = 0.0;
        let count = available_options.len();
        let total_freq: u32 = available_options.values().sum();

        let frequency = *available_options.get(word).unwrap_or(&0) as f64 / total_freq as f64;

        // println!("Total Count For Entropy Calculations : {}", count);

        // Calculate the entropy of the word
        // using the formula: -p * log2(p)
        // where p is the probability of the word

        for possibility in Score::permutations() {
            // if possibility == [Score::Correct; 5] {
            //     println!("Skipping as all the letters are correct.");
            //     continue;
            // }

            let mut possible_options = available_options.clone();

            let attempt = Attempt {
                word: word.to_string(),
                score: possibility,
            };

            let count_after_chosing = HighestEntropyAlgorithm::fetch_length_for_updated_dictionary(
                &attempt,
                &mut possible_options,
            ) as f64;

            if count_after_chosing == 0.0 {
                continue;
            }

            let probability = count_after_chosing / count as f64;

            entropy += (probability * probability.log2()) * -1 as f64;
        }

        // println!(
        //     "Word: {} Entropy: {} Frequency : {}",
        //     word, entropy, frequency
        // );
        WordScore::new(word, entropy + frequency)
    }
}

impl Solver for OptimizedEntropyAlgorithm {
    fn solve(&mut self, history: &[Attempt]) -> String {
        let last_attempt = history.last();

        // used to reduce the time on first attempt
        if last_attempt.is_none() {
            return "tares".to_string();
        }

        let last_try = last_attempt.unwrap();
        let last_word = last_try.word.as_str();

        // println!("Last {:?}", last_try);

        self.available_options.remove(last_word);
        HighestEntropyAlgorithm::fetch_length_for_updated_dictionary(
            &last_try,
            &mut self.available_options,
        );

        let mut word_entropies: Vec<WordScore> = vec![];

        for (word, _) in self.available_options.iter() {
            let word_entropy =
                OptimizedEntropyAlgorithm::calculate_score(*word, self.available_options.clone());

            word_entropies.push(word_entropy);
        }

        word_entropies.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());

        let result = word_entropies.last().unwrap().word.to_string();
        // println!("{:?}", result);
        return result;
    }
}
