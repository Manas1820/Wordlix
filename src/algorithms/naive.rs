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
        let last_score = history.score;

        // Remove the last word from the available options
        // since it is not the correct answer

        self.available_options.remove(last_word);

        // keeping it to manage the frequency of the characters specially the misplaced ones
        // planning to use it in cases of multiple repeated charater it might mark a few extra characters
        // as Incorrect.

        let mut current_frequency_character_map = HashMap::new();

        for (i, (lw, ls)) in last_word.chars().zip(last_score.iter()).enumerate() {
            match ls {
                Score::Correct => {
                    if self.restriction[i] {
                        continue;
                    }
                    self.restriction[i] = true;
                    self.possibility_grid[i] = [false; 26];
                    self.possibility_grid[i][lw as usize - 'a' as usize] = true;
                }
                Score::Misplaced => {
                    // since in the possibility grid we already have true for the correct character
                    current_frequency_character_map
                        .entry(lw)
                        .and_modify(|e| *e += 1)
                        .or_insert(1);
                }
                Score::Incorrect => {
                    if current_frequency_character_map.contains_key(&lw) {
                        continue;
                    } else {
                        for num in 0..5 {
                            if !self.restriction[num] {
                                self.possibility_grid[num][lw as usize - 'a' as usize] = false;
                            }
                        }
                    }
                }
            }
        }

        self.available_options.retain(|word, _| {
            let mut current_frequency_character_map = current_frequency_character_map.clone();

            for (i, w) in word.chars().enumerate() {
                if self.possibility_grid[i][w as usize - 'a' as usize] {
                    if self.restriction[i] {
                        continue;
                    } else {
                        if current_frequency_character_map.contains_key(&w) {
                            if current_frequency_character_map.get(&w).unwrap() > &0 {
                                current_frequency_character_map
                                    .entry(w)
                                    .and_modify(|e| *e -= 1);
                            }
                        }
                    }
                } else {
                    return false;
                };
            }

            // verifying if all the misplaced characters are used
            let misplaced_chars = current_frequency_character_map.values().sum::<u32>();
            if misplaced_chars > 0 {
                return false;
            }

            return true;
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

    #[test]
    fn test_naive_algorithm_update_possible_answers() {
        let mut naive_algorithm = NaiveAlgorithm::new();
        naive_algorithm.update_possible_answers(None);
        assert_eq!(naive_algorithm.possibility_grid, [[true; 26]; 5]);
        assert_eq!(naive_algorithm.restriction, [false; 5]);
        assert_eq!(naive_algorithm.available_options.len(), 12972);
        naive_algorithm.update_possible_answers(Some(&Attempt {
            word: "ddddd".to_string(),
            score: [
                Score::Incorrect,
                Score::Incorrect,
                Score::Incorrect,
                Score::Incorrect,
                Score::Correct,
            ],
        }));
        assert_eq!(
            naive_algorithm.restriction,
            [false, false, false, false, true]
        );
        assert_eq!(naive_algorithm.available_options.len(), 746);
        naive_algorithm.update_possible_answers(Some(&Attempt {
            word: "qdddd".to_string(),
            score: [
                Score::Correct,
                Score::Incorrect,
                Score::Incorrect,
                Score::Incorrect,
                Score::Correct,
            ],
        }));

        println!("{:?}", naive_algorithm.available_options.keys());
        assert_eq!(naive_algorithm.available_options.len(), 2);
    }

    #[test]
    fn test_naive_algorithm_initialization() {
        let naive_algorithm = NaiveAlgorithm::new();
        assert_eq!(naive_algorithm.available_options.len(), 12972);
    }
}
