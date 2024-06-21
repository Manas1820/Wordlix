use std::io;
use wordl::Wordle;

fn main() {
    let mut wordle_answers = std::collections::HashSet::new();
    wordle_answers = include_str!("../dataset/wordle/ans.txt")
        .lines()
        .map(|word| word.trim())
        .collect();

    let wordle = Wordle::new();
    for ans in wordle_answers {
        let result = todo!();
        // println!("The result is: {:?}", result);
    }
}
