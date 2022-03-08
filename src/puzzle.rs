use std::collections::{HashSet, VecDeque};
use std::fs;
use std::hash::Hash;
use std::time::Instant;

const MAX_DEPTH: usize = 20;

#[derive(Debug)]
pub enum FileReadError {
    FileNotFound,
    FileIsEmpty,
    FileIsCorrupt,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub enum Strategy {
    BFS([Direction; 4]),
    DFS([Direction; 4]),
    AStar(Metric),
}

#[derive(Debug)]
pub enum Metric {
    Hamming,
    Manhattan,
}

/// Puzzle contains a single state of the game.
/// Width and height represent the dimensions of the grid.
#[derive(Clone)]
pub struct Puzzle {
    /// The cells of the puzzle.
    // Right now we're using u8 for representing the cells, if width * height > 255, we'll need to change this.
    grid: Vec<Vec<u8>>,
    /// Series of moves that led to this state.
    path: Vec<Direction>,
}

/// Result of solving the puzzle.
pub struct SolveResult {
    /// Solution of puzzle or none if puzzle is unsolvable.
    path: Option<Vec<Direction>>,
    /// Number of visited states.
    visited_states: usize,
    /// Number of processed states.
    processed_states: usize,
    /// Maximum depth of the search tree.
    max_depth: usize,
    /// Time spent in milliseconds.
    time_spent: u128,
}

impl PartialEq for Puzzle {
    fn eq(&self, other: &Puzzle) -> bool {
        self.grid == other.grid
    }
}

impl Eq for Puzzle {}

impl Hash for Puzzle {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.grid.hash(state);
    }
}

impl Puzzle {
    /// Returns solved puzzle with the given dimensions.
    pub fn new(width: usize, height: usize) -> Puzzle {
        let mut grid = vec![vec![0; width]; height];
        for y in 0..height {
            for x in 0..width {
                grid[y][x] = (y * height + x + 1) as u8;
            }
        }
        grid[width - 1][height - 1] = 0;
        Puzzle { grid, path: vec![] }
    }

    /// Returns a puzzle from a file in which first line contains height and width
    /// and the next ones values of cells seperated by spaces.
    pub fn from_file(path: &str) -> Result<Puzzle, FileReadError> {
        // Read contents of file, if we fail to do that, the file probably doesn't exist.
        let contents = fs::read_to_string(path).map_err(|_err| FileReadError::FileNotFound)?;
        // Get first line of file, if we fail to do that, file is empty.
        let first_line = contents.lines().next().ok_or(FileReadError::FileIsEmpty)?;

        // First line of file should contain the dimensions of the puzzle.
        // We're splitting first line by whitespace, and parse the first two elements from &str to usize.
        let mut dimensions = first_line.split_whitespace().map(|s| s.parse::<usize>());

        // If these two elements were valid, we pull them out of Option<Result<>>, otherwise the file is corrupted.
        let height = match dimensions.next() {
            Some(Ok(height)) => height,
            _ => return Err(FileReadError::FileIsCorrupt),
        };
        let width = match dimensions.next() {
            Some(Ok(width)) => width,
            _ => return Err(FileReadError::FileIsCorrupt),
        };

        // Create a new grid of cells with the given dimensions.
        let mut grid = vec![vec![0; width]; height];

        // Iterate over the lines of the file, starting from the second line.
        for (y, line) in contents.lines().skip(1).enumerate() {
            // Split the line by whitespace, and parse the elements from &str to u32.
            let line_elements = line.split_whitespace().map(|s| s.parse::<u32>());
            // Iterate over the elements of the line, and set the cell at the given coordinates to the value.
            for (x, value) in line_elements.enumerate() {
                let value = value.map_err(|_err| FileReadError::FileIsCorrupt)?;
                grid[y][x] = value as u8;
            }
        }

        Ok(Puzzle { grid, path: vec![] })
    }

    fn is_solved(&self) -> bool {
        let height = self.grid.len();
        let width = self.grid[0].len();

        // Check if 0 is on the last place.
        if self.grid[height - 1][width - 1] != 0 {
            return false;
        }

        // Check if the numbers from all but last row are in order.
        for y in 0..(height - 1) {
            for x in 0..width {
                if self.grid[y][x] != (y * width + x + 1) as u8 {
                    return false;
                }
            }
        }

        // Check last row (without the last number, which should be 0).
        for x in 0..(width - 1) {
            if self.grid[height - 1][x] != ((height - 1) * width + x + 1) as u8 {
                return false;
            }
        }

        // If we got here, the puzzle is valid.
        return true;
    }

    fn empty_position(&self) -> (usize, usize) {
        let height = self.grid.len();
        let width = self.grid[0].len();

        for y in 0..height {
            for x in 0..width {
                if self.grid[y][x] == 0 {
                    return (y, x);
                }
            }
        }

        panic!("Puzzle is not solvable!");
    }

    fn move_empty(&self, direction: &Direction) -> Option<Puzzle> {
        let (y, x) = self.empty_position();
        let height = self.grid.len();
        let width = self.grid[0].len();

        let mut new_x = x;
        let mut new_y = y;

        // Check if the direction is valid.
        match direction {
            Direction::Up => {
                if y == 0 {
                    return None;
                }
                new_y = y - 1;
            }
            Direction::Down => {
                if y == height - 1 {
                    return None;
                }
                new_y = y + 1;
            }
            Direction::Left => {
                if x == 0 {
                    return None;
                }
                new_x = x - 1;
            }
            Direction::Right => {
                if x == width - 1 {
                    return None;
                }
                new_x = x + 1;
            }
        }

        let mut new_puzzle = self.clone();

        // Swap the empty cell with the cell in the given direction.
        new_puzzle.grid[y][x] = new_puzzle.grid[new_y][new_x];
        new_puzzle.grid[new_y][new_x] = 0;

        // Push the direction to the path which lead to this new state.
        new_puzzle.path.push(direction.clone());

        return Some(new_puzzle);
    }

    /// Returns vector of all possible moves from the current state in the given order.
    fn get_neighbour_states(&mut self, order: &[Direction; 4]) -> Vec<Puzzle> {
        let mut neighbours = Vec::new();

        for direction in order {
            if let Some(new_puzzle) = self.move_empty(direction) {
                neighbours.push(new_puzzle);
            }
        }

        neighbours
    }

    pub fn solve(&self, strategy: &Strategy) -> SolveResult {
        match strategy {
            Strategy::BFS(order) => self.solve_basic(order, false),
            Strategy::DFS(order) => self.solve_basic(order, true),
            Strategy::AStar(metric) => self.solve_priority(metric),
        }
    }

    fn solve_basic(&self, order: &[Direction; 4], is_dfs: bool) -> SolveResult {
        // Queue of puzzles to be solved.
        let mut queue = VecDeque::new();
        // HashSet of already visited puzzles. We use it to check if we've already visited a puzzle.
        let mut visited = HashSet::new();

        // Push the initial state to the queue and visited.
        queue.push_back(self.clone());
        visited.insert(self.clone());

        // Max depth of search tree.
        let mut max_depth = 0;

        let mut processed_states = 0;

        // Start timer from now to either finding the solution or processing all possible states.
        let start_time = Instant::now();

        // If we're doing DFS, we need to reverse the order of the moves.
        let mut order = order.to_vec();
        if is_dfs {
            order.reverse();
        }
        let order: &[Direction; 4] = &[order[0], order[1], order[2], order[3]];

        // While the queue is not empty, we keep iterating.
        while !queue.is_empty() {
            let mut current_state: Puzzle;
            // Depending on whetever we're doing BFS or DFS, we pop the first or last element.
            if is_dfs {
                current_state = queue.pop_back().unwrap();
            } else {
                current_state = queue.pop_front().unwrap();
            }

            // Insert current state into already visited states so that we don't visit it again.
            // visited.insert(current_state.clone());

            processed_states += 1;

            // Update the max depth of the search tree.
            if current_state.path.len() > max_depth {
                max_depth = current_state.path.len();
            }

            // If the current state is solved, we've found the solution.
            if current_state.is_solved() {
                return SolveResult {
                    path: Some(current_state.path.clone()),
                    max_depth,
                    visited_states: visited.len(),
                    processed_states,
                    time_spent: start_time.elapsed().as_nanos(),
                };
            }

            // For DFS skip generating neighbour states if we're at MAX_DEPTH depth.
            if is_dfs && current_state.path.len() == MAX_DEPTH {
                continue;
            }

            // Get the neighbour states of the current state.
            let neighbour_states = current_state.get_neighbour_states(order);

            // Iterate over the neighbours.
            for neighbour in neighbour_states {
                if let Some(previous) = visited.get(&neighbour) {
                    if previous.path.len() > neighbour.path.len() {
                        // If the state is already visited, but the path to it is shorter this time, we add it anyway,
                        // because maybe this time it'll be able to reach the solution.
                        visited.replace(neighbour.clone());
                        queue.push_back(neighbour.clone());
                    }
                } else {
                    // If the neighbour is not visited, we push him to the queue and mark him as visited.
                    visited.insert(neighbour.clone());
                    queue.push_back(neighbour.clone());
                }
            }
        }

        // If we got here, the puzzle is unsolvable.
        SolveResult {
            path: None,
            max_depth,
            visited_states: visited.len(),
            processed_states,
            time_spent: start_time.elapsed().as_nanos(),
        }
    }

    fn solve_priority(&self, metric: &Metric) -> SolveResult {
        todo!()
    }
}

impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.grid {
            for cell in row {
                write!(f, "{:3} ", cell)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl std::fmt::Display for SolveResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Path: {:?}\n", self.path)?;
        write!(f, "Max depth: {}\n", self.max_depth)?;
        write!(f, "Visited states: {}\n", self.visited_states)?;
        write!(f, "Processed states: {}\n", self.processed_states)?;
        write!(
            f,
            "Time spent: {:.3}\n",
            self.time_spent as f32 * f32::powi(10.0, -6)
        )?;
        Ok(())
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
            Direction::Left => write!(f, "Left"),
            Direction::Right => write!(f, "Right"),
        }
    }
}
