use puzzle::{Direction, Metric, Puzzle, Strategy};
use std::env;

mod puzzle;

enum ArgsError {
    NotEnoughArguments,
    InvalidStrategy,
    InvalidOrder,
}

#[derive(Debug)]
struct Config {
    pub strategy: Strategy,
    pub input_file: String,
    pub solution_file: String,
    pub stats_file: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, ArgsError> {
        if args.len() < 5 {
            return Err(ArgsError::NotEnoughArguments);
        }

        let strategy = args[1].as_str();
        let order = args[2].as_str();

        let input_file = args[3].clone();
        let solution_file = args[4].clone();
        let stats_file = args[5].clone();

        let strategy = match strategy {
            "bfs" | "dfs" => {
                let mut directions = [Direction::Up; 4];
                for (i, direction) in order.to_uppercase().chars().enumerate() {
                    match direction {
                        'U' => directions[i] = Direction::Up,
                        'D' => directions[i] = Direction::Down,
                        'L' => directions[i] = Direction::Left,
                        'R' => directions[i] = Direction::Right,
                        _ => return Err(ArgsError::InvalidOrder),
                    }
                }
                if strategy == "bfs" {
                    Strategy::BFS(directions)
                } else {
                    Strategy::DFS(directions)
                }
            }
            "astr" => {
                let metric = match order {
                    "manh" => Metric::Manhattan,
                    "hamm" => Metric::Hamming,
                    _ => return Err(ArgsError::InvalidOrder),
                };
                Strategy::AStar(metric)
            }
            _ => return Err(ArgsError::InvalidStrategy),
        };

        Ok(Config {
            strategy,
            input_file,
            solution_file,
            stats_file,
        })
    }
}

fn main() {
    // Get the arguments from the command line and parse them into the config.
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        print!("Problem parsing arguments: ");
        match err {
            ArgsError::NotEnoughArguments => println!("Not enough arguments"),
            ArgsError::InvalidStrategy => println!("Invalid strategy"),
            ArgsError::InvalidOrder => println!("Invalid order"),
        }
        std::process::exit(1);
    });

    let mut puzzle = Puzzle::from_file(&config.input_file).unwrap_or_else(|err| {
        match err {
            puzzle::FileReadError::FileNotFound => {
                println!("File not found: {}", config.input_file);
            }
            puzzle::FileReadError::FileIsEmpty => {
                println!("File is empty: {}", config.input_file);
            }
            puzzle::FileReadError::FileIsCorrupt => {
                println!("File is corrupted: {}", config.input_file);
            }
        }
        std::process::exit(1);
    });

    let solution = puzzle.solve(&config.strategy);

    println!("{}", solution);
}
