use crate::{Attempt, Score};
use std::collections::HashMap;

pub struct Utils;

impl Utils {
    // function will verify if the word can be a potential candidate or not
    pub fn if_attempt_is_similar_to_word(attempt: &Attempt, word: &'static str) -> bool {
        if attempt.word.len() != word.len() {
            return false;
        }

        let mut word_frequency_character_map = HashMap::new();

        for c in word.chars() {
            *word_frequency_character_map.entry(c).or_insert(0) += 1;
        }

        let mut attempted_mismatched_character = HashMap::new();
        let mut attempted_incorrect_character = HashMap::new();

        for (i, (c, sc)) in attempt.word.chars().zip(attempt.score).enumerate() {
            match sc {
                Score::Correct => {
                    if c != word.chars().nth(i).unwrap() {
                        return false;
                    }
                    word_frequency_character_map
                        .entry(c)
                        .and_modify(|e| *e -= 1);
                }
                Score::Misplaced => {
                    // if the misplced character is already at the position
                    if c == word.chars().nth(i).unwrap() {
                        return false;
                    }
                    *attempted_mismatched_character.entry(c).or_insert(0) += 1
                }
                Score::Incorrect => *attempted_incorrect_character.entry(c).or_insert(0) += 1,
            }
        }

        for (c, count) in attempted_mismatched_character {
            *word_frequency_character_map.entry(c).or_insert(0) -= count;
            if word_frequency_character_map.get(&c) == None
                || *word_frequency_character_map.get(&c).unwrap() < 0
            {
                return false;
            }
        }

        for (c, _) in attempted_incorrect_character {
            if word_frequency_character_map.get(&c) != None
                && *word_frequency_character_map.get(&c).unwrap() > 0
            {
                return false;
            }
        }

        true
    }
}
