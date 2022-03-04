use std::fs;

#[derive(Debug)]
pub enum FileReadError {
    FileNotFound,
    FileIsEmpty,
    FileIsCorrupt,
}

/// Puzzle contains a single state of the game.
/// Width and height represent the dimensions of the grid.
pub struct Puzzle {
    /// The cells of the puzzle.
    // Right now we're using u8 for representing the cells, if Width * Height > 255, we'll need to change this.
    grid: Vec<Vec<u8>>,
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
        Puzzle { grid }
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
            // Split the line by whitespace, and parse the elements from &str to usize.
            let line_elements = line.split_whitespace().map(|s| s.parse::<usize>());
            // Iterate over the elements of the line, and set the cell at the given coordinates to the value.
            for (x, value) in line_elements.enumerate() {
                let value = value.map_err(|_err| FileReadError::FileIsCorrupt)?;
                grid[y][x] = value as u8;
            }
        }

        Ok(Puzzle { grid })
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
