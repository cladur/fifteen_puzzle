use puzzle::Puzzle;
use std::env;

mod puzzle;

#[derive(Clone, Copy, Debug)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum Strategy {
    BFS,
    DFS,
    AStar,
}

#[derive(Debug)]
enum Order {
    Neighbourhood([Directions; 4]),
    Hamming,
    Manhattan,
}

enum ArgsError {
    NotEnoughArguments,
    InvalidStrategy,
    InvalidOrder,
}

#[derive(Debug)]
struct Config {
    strategy: Strategy,
    order: Order,
    input_file: String,
    solution_file: String,
    stats_file: String,
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
            "bfs" => Strategy::BFS,
            "dfs" => Strategy::DFS,
            "astr" => Strategy::AStar,
            _ => return Err(ArgsError::InvalidStrategy),
        };

        let order = match order {
            "hamm" => Order::Hamming,
            "manh" => Order::Manhattan,
            _ => {
                let mut directions = [Directions::Up; 4];
                for (i, direction) in order.to_uppercase().chars().enumerate() {
                    match direction {
                        'U' => directions[i] = Directions::Up,
                        'D' => directions[i] = Directions::Down,
                        'L' => directions[i] = Directions::Left,
                        'R' => directions[i] = Directions::Right,
                        _ => return Err(ArgsError::InvalidOrder),
                    }
                }
                Order::Neighbourhood(directions)
            }
        };

        Ok(Config {
            strategy,
            order,
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

    println!("{:?}", config);
}
