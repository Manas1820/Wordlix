use std::collections::{hash_map, HashSet};

pub struct Wordle {
    /// The dictionary of words that the game will use
    dictionary: HashSet<&'static str>,
}

impl Wordle {
    pub fn new() -> Self {
        let dictionary = include_str!("../dataset/combined_ans_guess.txt")
            .lines()
            .map(|word| word.trim())
            .collect();

        Self { dictionary }
    }

    pub fn game<S: Solver>(&self, answer: &'static str, mut solver: S) -> Result<usize, ()> {
        let mut game_history = Vec::new();

        loop {
            let guess = solver.solve(&game_history);

            if guess == answer {
                println!("Correct! The answer was: {}", answer);
                return Ok(game_history.len() + 1);
            }

            debug_assert!(self.dictionary.contains(&*guess));

            let score = Score::color(&guess, answer);
            game_history.push(Attempt { word: guess, score });
        }
    }
}

/// A struct that represents a single attempt to guess the word
#[derive(Debug)]
pub struct Attempt {
    /// The word that was guessed in a perticular attempt
    word: String,
    /// The score of the guess, it is an array of 5 elements where each
    /// element represents the score of a letter in the word
    score: [Score; 5],
}

/// A struct that represents the possible type of scoring for a word
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Score {
    /// color: green
    Correct,
    /// color: yellow
    Misplaced,
    /// color: grey
    Incorrect,
}

impl Score {
    /// This function should return the color of the attempt
    pub fn color(answer: &str, guess: &str) -> [Self; 5] {
        // Check if the length of the answer and guess is 5
        assert_eq!(answer.len(), 5);
        assert_eq!(guess.len(), 5);

        let mut final_score = [Score::Incorrect; 5];

        let mut character_map = hash_map::HashMap::new();

        for (index, (a, g)) in answer.chars().zip(guess.chars()).enumerate() {
            if a == g {
                final_score[index] = Score::Correct;
            } else {
                if character_map.contains_key(&a) {
                    character_map.insert(a, character_map.get(&a).unwrap() + 1);
                } else {
                    character_map.insert(a, 1);
                }
            }
        }

        // println!("{:?}", character_map);

        for (index, (a, g)) in answer.chars().zip(guess.chars()).enumerate() {
            if a == g {
                continue;
            } else if character_map.contains_key(&g) && character_map.get(&g).unwrap() > &0 {
                println!("{:?} {:?}", character_map, g);
                println!("Reached");
                final_score[index] = Score::Misplaced;
                character_map.insert(g, character_map.get(&g).unwrap() - 1);
            }
        }

        return final_score;
    }
}

pub trait Solver {
    /// This function should return the word that the solver thinks is the correct answer
    fn solve(&mut self, history: &[Attempt]) -> String;
}

impl Solver for fn(attempts: &[Attempt]) -> String {
    fn solve(&mut self, history: &[Attempt]) -> String {
        (*self)(history)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod wordle {
        use super::*;

        macro_rules! solver {
            (|$history: ident| $impl:block ) => {{
                struct S;
                impl Solver for S {
                    fn solve(&mut self, $history: &[Attempt]) -> String {
                        $impl
                    }
                }
                S
            }};
        }

        #[test]
        fn verify_dictionary() {
            let wordle = Wordle::new();
            assert_eq!(wordle.dictionary.len(), 12972);
        }

        #[test]
        fn test_game_success_on_attempt_1() {
            let wordle = Wordle::new();
            let answer = "hello";

            let solver = solver!(|_history| { "hello".to_string() });
            assert_eq!(wordle.game(answer, solver), Ok(1));
        }

        #[test]
        fn test_game_success_on_attempt_2() {
            let wordle = Wordle::new();
            let answer = "hello";
            let solver = solver!(|history| {
                if history.len() == 1 {
                    return "hello".to_string();
                } else {
                    return "world".to_string();
                }
            });
            assert_eq!(wordle.game(answer, solver), Ok(2));
        }
    }

    mod score_calculation {
        use super::*;

        macro_rules! result {
            (C) => {
                Score::Correct
            };
            (M) => {
                Score::Misplaced
            };
            (I) => {
                Score::Incorrect
            };
            ($($c:tt)+) => {
                [$(result!($c)),+]
            };
        }

        #[test]
        fn test_score_color_for_same_words() {
            let answer = "hello";
            let guess = "hello";
            assert_eq!(Score::color(answer, guess), result!(C C C C C));
        }

        #[test]
        fn test_score_color_for_totally_different_letters() {
            let answer = "abcde";
            let guess = "fghij";
            assert_eq!(Score::color(answer, guess), result!(I I I I I));
        }

        #[test]
        fn test_score_color_with_some_different_words() {
            let answer = "helro";
            let guess = "world";

            assert_eq!(Score::color(answer, guess), result!(I M M M I));
        }

        #[test]
        fn test_score_color_with_some_correct_and_misplaced_words() {
            let answer = "azzaz";
            let guess = "aaabb";

            assert_eq!(Score::color(answer, guess), result!(C M I I I));
        }

        #[test]
        fn test_score_color_with_some_correct_and_misplaced_words_edge_case() {
            let answer = "baccc";
            let guess = "aaddd";

            assert_eq!(Score::color(answer, guess), result!(I C I I I));
        }
    }
}
