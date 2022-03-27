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
                    Strategy::Bfs(directions)
                } else {
                    Strategy::Dfs(directions)
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

    let puzzle = Puzzle::from_file(&config.input_file).unwrap_or_else(|err| {
        match err {
            puzzle::FileReadError::NotFound => {
                println!("File not found: {}", config.input_file);
            }
            puzzle::FileReadError::IsEmpty => {
                println!("File is empty: {}", config.input_file);
            }
            puzzle::FileReadError::IsCorrupt => {
                println!("File is corrupted: {}", config.input_file);
            }
        }
        std::process::exit(1);
    });

    let solution = puzzle.solve(&config.strategy);

    let solution_file_content = match &solution.path {
        Some(path) => {
            let mut steps = String::new();
            for step in path {
                steps.push(match step {
                    Direction::Up => 'U',
                    Direction::Down => 'D',
                    Direction::Left => 'L',
                    Direction::Right => 'R',
                    Direction::None => panic!(),
                });
            }
            format!("{}\n{}", &path.len(), steps)
        }
        None => String::from("-1"),
    };

    let path_len = match solution.path {
        Some(path) => path.len().to_string(),
        None => "-1".to_string(),
    };

    let stats_file_content = format!(
        "{}\n{}\n{}\n{}\n{:.3}",
        path_len,
        solution.visited_states,
        solution.processed_states,
        solution.max_depth,
        solution.time_spent as f32 * 10.0_f32.powi(-6)
    );

    std::fs::write(&config.solution_file, solution_file_content)
        .expect(format!("Error writing solution to file: {}", &config.solution_file).as_str());

    std::fs::write(&config.stats_file, stats_file_content)
        .expect(format!("Error writing stats to file: {}", &config.stats_file).as_str());
}
