use std::collections::HashSet;

use clap::{arg, Parser, Subcommand, ValueEnum};
use wordl::Wordle;

#[derive(Parser)]
#[command(version, long_about = None)]
struct Args {
    #[command(subcommand)]
    commands: Command,
}

#[derive(Debug, Clone, ValueEnum)]
enum Algorithm {
    Random,
    HighestEntropy,
    OptimizedEntropy,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(long_about = "Run the previous wordle games for benchmarking the algorithms")]
    SimulateRun {
        // Algorithm to use
        #[arg(short, long)]
        algorithm: Algorithm,

        // Number of words to guess
        #[arg(short, long)]
        count: Option<usize>,
    },
}

fn main() {
    // let wordle = Wordle::new();
    // wordle.assist(wordl::algorithms::OptimizedEntropyAlgorithm::new());

    let args: Args = Args::parse();
    match args.commands {
        Command::SimulateRun { algorithm, count } => {
            let wordle = Wordle::new();
            // not load the whole thing just load the required amount
            let wordle_answers: HashSet<&'static str> = include_str!("../dataset/wordle/ans.txt")
                .lines()
                .map(|word| word.trim())
                .take(count.unwrap_or(usize::MAX))
                .collect();

            let mut counter = 0;

            for ans in &wordle_answers {
                let result = match algorithm {
                    Algorithm::OptimizedEntropy => {
                        wordle.game(ans, wordl::algorithms::OptimizedEntropyAlgorithm::new())
                    }
                    Algorithm::Random => wordle.game(ans, wordl::algorithms::NaiveAlgorithm::new()),
                    Algorithm::HighestEntropy => {
                        wordle.game(ans, wordl::algorithms::HighestEntropyAlgorithm::new())
                    }
                };
                counter += result.unwrap();
            }

            println!("Total number of guesses attempted: {:?}", counter);

            let no_of_moves = counter as f64 / wordle_answers.len() as f64;
            println!("Average number of moves: {:?}", no_of_moves);

            // assuming an average human can solve it in 4 tries
            let efficiency = (4f64 - no_of_moves) * 2f64 / (no_of_moves + 4f64);

            println!("Efficiency over a human: {:?} %", efficiency * 100f64);
        }
    }

    // }

    // let wordle = Wordle::new();
    //
    //
    // let mut wordle_answers = std::collections::HashSet::new();
    // wordle_answers = include_str!("../dataset/wordle/ans.txt")
    //     .lines()
    //     .map(|word| word.trim())
    //     .collect();
    //
    // let mut count = 0;
    // for ans in &wordle_answers {
    //     // println!("Guessing for answer: {}", ans);
    //     let result = wordle.game(ans, wordl::algorithms::OptimizedEntropyAlgorithm::new());
    //     // println!("Guessed the ans in : {:?} moves", Some(result));
    //     count += result.unwrap();
    // }
    //
    // println!("Total number of words guessed: {:?}", count);
    // println!(
    //     "Average number of moves: {:?}",
    //     count as f64 / wordle_answers.len() as f64
    // );

    // let ans = "genre";
    // println!("Guessing for answer: {}", ans);
    // let result = wordle.game(ans, wordl::algorithms::OptimizedEntropyAlgorithm::new());
    //
    // println!("Guessed the ans in : {:?} moves", result);
}
