use super::Utils;
use crate::{Attempt, Score, Solver};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct WordEntropy {
    pub word: &'static str,
    pub entropy: f64,
}

impl WordEntropy {
    pub fn new(word: &'static str, entropy: f64) -> Self {
        Self { word, entropy }
    }
}

// Maximum entropy calculation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HighestEntropyAlgorithm {
    pub available_options: HashMap<&'static str, u32>,
}

impl HighestEntropyAlgorithm {
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

    pub fn calculate_entropy(
        word: &'static str,
        available_options: HashMap<&'static str, u32>,
    ) -> WordEntropy {
        let mut entropy = 0.0;
        let count = available_options.len();

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

        WordEntropy::new(word, entropy)
    }

    fn fetch_length_for_updated_dictionary(
        attempt: &Attempt,
        available_options: &mut HashMap<&'static str, u32>,
    ) -> usize {
        // Remove the last word from the available options
        // since it is not the correct answer
        available_options.retain(|word, _| {
            return Utils::if_attempt_is_similar_to_word(&attempt, word);
        });

        return available_options.len();
    }
}

impl Solver for HighestEntropyAlgorithm {
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

        let mut word_entropies: Vec<WordEntropy> = vec![];

        for (word, _) in self.available_options.iter() {
            let word_entropy =
                HighestEntropyAlgorithm::calculate_entropy(*word, self.available_options.clone());
            word_entropies.push(word_entropy);
        }

        word_entropies.sort_by(|a, b| a.entropy.partial_cmp(&b.entropy).unwrap());

        let result = word_entropies.last().unwrap().word.to_string();
        // println!("{:?}", result);
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Attempt;
    use crate::Score;

    #[test]
    fn test_if_attempt_is_similar_to_word_ideal_case() {
        let attempt = Attempt {
            word: "hello".to_string(),
            score: [Score::Correct; 5],
        };
        let word = "hello";
        let result = Utils::if_attempt_is_similar_to_word(&attempt, word);
        assert_eq!(result, true);
    }

    #[test]
    fn test_if_attempt_is_similar_to_word_ideal_case_sample_1() {
        let attempt = Attempt {
            word: "weary".to_string(),
            score: [
                Score::Correct,
                Score::Incorrect,
                Score::Misplaced,
                Score::Incorrect,
                Score::Incorrect,
            ],
        };
        let word = "wages";
        let result = Utils::if_attempt_is_similar_to_word(&attempt, word);
        assert_eq!(result, false);
    }

    #[test]
    fn test_update_available_options_after_word_selection() {
        let mut highest_entropy_algorithm = HighestEntropyAlgorithm::new();
        let attempt = Attempt {
            word: "weary".to_string(),
            score: [
                Score::Correct,
                Score::Incorrect,
                Score::Misplaced,
                Score::Incorrect,
                Score::Incorrect,
            ],
        };
        let result = HighestEntropyAlgorithm::fetch_length_for_updated_dictionary(
            &attempt,
            &mut highest_entropy_algorithm.available_options,
        );
        assert_eq!(result, 58);
    }

    #[test]
    fn test_update_available_options_after_word_selection_sample_2() {
        let mut highest_entropy_algorithm = HighestEntropyAlgorithm::new();
        let attempt = Attempt {
            word: "weary".to_string(),
            score: [
                Score::Incorrect,
                Score::Misplaced,
                Score::Incorrect,
                Score::Incorrect,
                Score::Incorrect,
            ],
        };
        let result = HighestEntropyAlgorithm::fetch_length_for_updated_dictionary(
            &attempt,
            &mut highest_entropy_algorithm.available_options,
        );
        assert_eq!(result, 1419);
    }

    #[test]
    fn test_calculate_entropy() {
        let highest_entropy_algorithm = HighestEntropyAlgorithm::new();
        let result = HighestEntropyAlgorithm::calculate_entropy(
            "corms",
            highest_entropy_algorithm.available_options,
        );
        assert_eq!(result.entropy, 5.137219911431635);
    }
}
