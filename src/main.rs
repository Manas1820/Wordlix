use wordl::Wordle;

fn main() {
    let mut wordle_answers = std::collections::HashSet::new();
    wordle_answers = include_str!("../dataset/wordle/ans.txt")
        .lines()
        .map(|word| word.trim())
        .collect();

    let wordle = Wordle::new();

    let mut count = 0;
    for ans in &wordle_answers {
        println!("Guessing for answer: {}", ans);
        let result = wordle.game(ans, wordl::algorithms::NaiveAlgorithm::new());
        println!("Guessed the ans in : {:?} moves", Some(result));
        count += result.unwrap();
    }

    println!("Total number of words guessed: {:?}", count);
    println!(
        "Average number of moves: {:?}",
        count as f64 / wordle_answers.len() as f64
    );
    // let ans = "cease";
    // println!("Guessing for answer: {}", ans);
    // let result = wordle.game(ans, wordl::algorithms::NaiveAlgorithm::new());
    // println!("Guessed the ans in : {:?} moves", Some(result));
}
