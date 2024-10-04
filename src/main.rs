use std::io::{self, Write, BufRead};

const BOARD_SIZE: usize = 8;

// Define a Cell enum to represent the state of each cell on the board
#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Empty,
    Black,
    White,
}

impl Cell {
    // Method to get the opposite color
    fn opposite(&self) -> Cell {
        match self {
            Cell::Black => Cell::White,
            Cell::White => Cell::Black,
            Cell::Empty => Cell::Empty, // Default case; shouldn't be used for valid moves
        }
    }

    // Convert the cell to a character
    fn to_char(&self) -> char {
        match self {
            Cell::Empty => '.',
            Cell::Black => 'B',
            Cell::White => 'W',
        }
    }
}

// Define a Board struct to represent the game board
struct Board {
    grid: [[Cell; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    // Initialize a new board with starting positions
    fn new() -> Self {
        let mut grid = [[Cell::Empty; BOARD_SIZE]; BOARD_SIZE];
        grid[3][3] = Cell::White;
        grid[3][4] = Cell::Black;
        grid[4][3] = Cell::Black;
        grid[4][4] = Cell::White;
        Board { grid }
    }

    // Print the current state of the board
    fn print(&self) {
        println!("  abcdefgh");
        for (i, row) in self.grid.iter().enumerate() {
            print!("{} ", (b'a' + i as u8) as char);
            for &cell in row.iter() {
                print!("{}", cell.to_char());
            }
            println!();
        }
    }

    // Check if a move is valid for the given color
    fn is_valid_move(&self, row: usize, col: usize, color: Cell) -> bool {
        // Check if the position is out of bounds or already occupied
        if row >= BOARD_SIZE || col >= BOARD_SIZE || self.grid[row][col] != Cell::Empty {
            return false;
        }

        // Define all possible directions to check
        let directions = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1), (1, 0), (1, 1),
        ];

        // Iterate over each direction to see if it's a valid capturing move
        for &(dr, dc) in directions.iter() {
            let mut r = row as isize + dr;
            let mut c = col as isize + dc;
            let mut found_opposite = false;

            while r >= 0 && r < BOARD_SIZE as isize && c >= 0 && c < BOARD_SIZE as isize {
                match self.grid[r as usize][c as usize] {
                    x if x == color.opposite() => found_opposite = true,
                    x if x == color && found_opposite => return true, // Only valid if an opposite color is found first
                    _ => break,
                }
                r += dr;
                c += dc;
            }
        }
        false
    }

    // Apply a move and update the board state
    fn apply_move(&mut self, row: usize, col: usize, color: Cell) {
        self.grid[row][col] = color;

        let directions = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1), (1, 0), (1, 1),
        ];

        for &(dr, dc) in directions.iter() {
            let mut r = row as isize + dr;
            let mut c = col as isize + dc;
            let mut to_flip = Vec::new();

            while r >= 0 && r < BOARD_SIZE as isize && c >= 0 && c < BOARD_SIZE as isize {
                match self.grid[r as usize][c as usize] {
                    x if x == color.opposite() => to_flip.push((r as usize, c as usize)),
                    x if x == color => {
                        for &(fr, fc) in to_flip.iter() {
                            self.grid[fr][fc] = color; // Flip all in-between pieces to current color
                        }
                        break;
                    }
                    _ => break,
                }
                r += dr;
                c += dc;
            }
        }
    }

    // Check if the player has any valid moves
    fn has_valid_moves(&self, color: Cell) -> bool {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if self.is_valid_move(row, col, color) {
                    return true;
                }
            }
        }
        false
    }

    // Count the number of black and white pieces on the board
    fn count_pieces(&self) -> (usize, usize) {
        let mut black_count = 0;
        let mut white_count = 0;
        for row in self.grid.iter() {
            for &cell in row.iter() {
                match cell {
                    Cell::Black => black_count += 1,
                    Cell::White => white_count += 1,
                    _ => {}
                }
            }
        }
        (black_count, white_count)
    }
}

fn main() {
    let mut board = Board::new();
    let mut current_player = Cell::Black;
    let stdin = io::stdin();

    loop {
        board.print();
        let (black_count, white_count) = board.count_pieces();

        // Check if the game has ended: both players have no valid move
        if !board.has_valid_moves(Cell::Black) && !board.has_valid_moves(Cell::White) {
            // Check if current player has any valid moves
            if !board.has_valid_moves(current_player) {
                println!("B player has no valid move.");
                println!("W player has no valid move.");
            }
            // results
            let result = match black_count.cmp(&white_count) {
                std::cmp::Ordering::Greater => format!("Black wins by {} points!", black_count - white_count),
                std::cmp::Ordering::Less => format!("White wins by {} points!", white_count - black_count),
                std::cmp::Ordering::Equal => "Draw!".to_string(),
            };
            println!("{}", result);
            break;
        }

        // Get input move from the player
        let mut input = String::new();
        print!("Enter move for colour {} (RowCol): ", current_player.to_char());
        io::stdout().flush().expect("Failed to flush stdout.");

        stdin.lock().read_line(&mut input).expect("Failed to read line");
        let move_input = input.trim();

        if move_input.len() != 2 {
            println!("Invalid input. Try again.");
            continue;
        }

        let row = (move_input.chars().nth(0).unwrap() as usize) - ('a' as usize);
        let col = (move_input.chars().nth(1).unwrap() as usize) - ('a' as usize);

        // Check if the entered move is valid
        if row >= BOARD_SIZE || col >= BOARD_SIZE || !board.is_valid_move(row, col, current_player) {
            println!("Invalid move. Try again.");
            continue;
        }

        // Apply the move
        board.apply_move(row, col, current_player);

        // Switch to the other player
        current_player = current_player.opposite();
    }
}
