pub mod algorithms;
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

    pub fn assist<S: Solver>(&self, mut solver: S) -> () {
        let mut game_history = Vec::new();
        println!("--------------------------------------------");
        println!("               Wordle Assistant             ");
        println!("--------------------------------------------");

        loop {
            let guess = solver.solve(&game_history);
            println!("My suggestion is to use '{}'", guess);

            println!("How was that ? (C for correct, M for misplaced, I for incorrect)");
            let mut performance = String::new();
            std::io::stdin()
                .read_line(&mut performance)
                .expect("failed to readline");

            performance = performance.trim().to_string();

            assert!(performance.len() == 5);

            let mut score = [Score::Correct; 5];
            for (i, ch) in performance.to_string().chars().enumerate() {
                let score_performance = match ch {
                    'C' => Score::Correct,
                    'M' => Score::Misplaced,
                    'I' => Score::Incorrect,
                    _ => panic!("Invalid input"),
                };
                score[i] = score_performance;
            }

            assert!(self.dictionary.contains(&*guess));
            game_history.push(Attempt { word: guess, score });

            if score.iter().all(|&x| x == Score::Correct) {
                println!(
                    "Good Game! Looks like I got it right this time. Took {} turns to guess !!",
                    game_history.len()
                );
                break;
            }
        }
    }

    pub fn game<S: Solver>(&self, answer: &'static str, mut solver: S) -> Result<usize, ()> {
        let mut game_history = Vec::new();
        print!(" Guess");
        loop {
            let guess = solver.solve(&game_history);
            print!(" -> {}", guess);
            if guess == answer {
                print!(" -> Finished !\n");
                return Ok(game_history.len() + 1);
            }

            debug_assert!(self.dictionary.contains(&*guess));
            let score = Score::color(answer, &guess);
            // println!("Score: {:?}", score);

            game_history.push(Attempt { word: guess, score });
        }
    }
}

/// A struct that represents a single attempt to guess the word
#[derive(Debug)]
pub struct Attempt {
    /// The word that was guessed in a perticular attempt
    pub word: String,
    /// The score of the guess, it is an array of 5 elements where each
    /// element represents the score of a letter in the word
    pub score: [Score; 5],
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
                final_score[index] = Score::Misplaced;
                character_map.insert(g, character_map.get(&g).unwrap() - 1);
            }
        }

        return final_score;
    }

    pub fn permutations() -> impl Iterator<Item = [Self; 5]> {
        itertools::iproduct!(
            [Score::Correct, Score::Misplaced, Score::Incorrect],
            [Score::Correct, Score::Misplaced, Score::Incorrect],
            [Score::Correct, Score::Misplaced, Score::Incorrect],
            [Score::Correct, Score::Misplaced, Score::Incorrect],
            [Score::Correct, Score::Misplaced, Score::Incorrect]
        )
        .map(|(a, b, c, d, e)| [a, b, c, d, e])
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

        #[test]
        fn test_score_color_with_some_correct_and_misplaced_words_edge_case_1() {
            let answer = "admin";
            let guess = "which";

            assert_eq!(Score::color(answer, guess), result!(I I M I I));
        }
    }
}
