use super::Utils;
use crate::{Attempt, Score, Solver};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NaiveAlgorithm {
    pub available_options: HashMap<&'static str, u32>,
    pub possibility_grid: [[bool; 26]; 5],
    pub restriction: [bool; 5],
}

impl NaiveAlgorithm {
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

        let possibility_grid = [[true; 26]; 5];
        let restriction = [false; 5];
        Self {
            available_options,
            possibility_grid,
            restriction,
        }
    }

    pub fn update_possible_answers(&mut self, last_attempt: Option<&Attempt>) {
        if last_attempt.is_none() {
            return;
        }
        let history = last_attempt.unwrap();
        let last_word = history.word.as_str();

        // Remove the last word from the available options
        // since it is not the correct answer

        self.available_options.remove(last_word);
        self.available_options.retain(|word, _| {
            return Utils::if_attempt_is_similar_to_word(history, word);
        });
    }
}

impl Solver for NaiveAlgorithm {
    fn solve(&mut self, history: &[Attempt]) -> String {
        self.update_possible_answers(history.last());
        let possible_ans = &self
            .available_options
            .iter()
            .max_by_key(|&(_, count)| count)
            .unwrap()
            .0;

        // removing the word from the available options as it is already used
        return possible_ans.to_string();
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_naive_algorithm_solve() {
        let mut naive_algorithm = NaiveAlgorithm::new();
        naive_algorithm.update_possible_answers(None);
        assert_eq!(naive_algorithm.possibility_grid, [[true; 26]; 5]);
        assert_eq!(naive_algorithm.restriction, [false; 5]);
        assert_eq!(naive_algorithm.available_options.len(), 12972);

        let attempt = Attempt {
            word: "which".to_string(),
            score: [
                Score::Incorrect,
                Score::Misplaced,
                Score::Incorrect,
                Score::Misplaced,
                Score::Incorrect,
            ],
        };

        naive_algorithm.update_possible_answers(Some(&attempt));
        assert_eq!(
            naive_algorithm.restriction,
            [false, false, false, false, false]
        );
    }

    // #[test]
    // fn test_naive_algorithm_update_possible_answers() {
    //     let mut naive_algorithm = NaiveAlgorithm::new();
    //     naive_algorithm.update_possible_answers(None);
    //     assert_eq!(naive_algorithm.possibility_grid, [[true; 26]; 5]);
    //     assert_eq!(naive_algorithm.restriction, [false; 5]);
    //     assert_eq!(naive_algorithm.available_options.len(), 12972);
    //     naive_algorithm.update_possible_answers(Some(&Attempt {
    //         word: "ddddd".to_string(),
    //         score: [
    //             Score::Incorrect,
    //             Score::Incorrect,
    //             Score::Incorrect,
    //             Score::Incorrect,
    //             Score::Correct,
    //         ],
    //     }));
    //     assert_eq!(
    //         naive_algorithm.restriction,
    //         [false, false, false, false, true]
    //     );
    //     assert_eq!(naive_algorithm.available_options.len(), 746);
    //     naive_algorithm.update_possible_answers(Some(&Attempt {
    //         word: "qdddd".to_string(),
    //         score: [
    //             Score::Correct,
    //             Score::Incorrect,
    //             Score::Incorrect,
    //             Score::Incorrect,
    //             Score::Correct,
    //         ],
    //     }));

    //     println!("{:?}", naive_algorithm.available_options.keys());
    //     assert_eq!(naive_algorithm.available_options.len(), 2);
    // }

    #[test]
    fn test_naive_algorithm_initialization() {
        let naive_algorithm = NaiveAlgorithm::new();
        assert_eq!(naive_algorithm.available_options.len(), 12972);
    }
}
