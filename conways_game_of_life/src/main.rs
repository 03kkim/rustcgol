
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Cell {
    pub alive: bool,
    pub row: usize,
    pub col: usize
}

#[derive(Debug, Default)]
pub struct GameBoard {
    height: usize,
    width: usize, 
    board: Vec<Vec<bool>>
    // board[i][j] - i is the row number, j is the column number
    // do we need to make board Vec<Vec<Cell>>?
}

// Do we want to wrap the return values with Enums? (for "safe Rust")
impl GameBoard {

    pub fn new(height: usize, width: usize) -> GameBoard {
        GameBoard {
            height,
            width,
            board: vec![vec![false; width]; height]
        }
    }

    // update the game board with the data in a new game board, 
    // not taking the new game board's ownership
    // need to work on error handling: try avoid using panic!()
    pub fn update(&mut self, board_from: &GameBoard) {
        if board_from.height != self.height || board_from.width != self.width {
            panic!("two boards have different sizes!");
        }
        for row in 0..board_from.height {
            for col in 0..board_from.width {
                self.board[row][col] = board_from.board[row][col];
            }
        }
    }

    // a print function for the game board
    pub fn print(&self) {
        println!("---Current Game Board---");
        println!("height: {:?}      width: {:?}", self.height, self.width);
        println!("------------------------");
        for row in 0..self.height {
            let mut row_str: String = String::from(" ");
            for col in 0..self.width {
                if self.board[row][col] {
                    row_str += "1 ";
                } else {
                    row_str += "0 ";
                }
            }
            println!("{:?}", row_str);
        }
    }

    pub fn set_cell(&mut self, alive: bool, row: usize, col: usize) {
        if row >= self.height || col >= self.width {
            panic!("invalid board position!");
        }
        self.board[row][col] = alive;
    }

    pub fn get_board(&self) -> &Vec<Vec<bool>> {
        &self.board
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_width(&self) -> usize {
        self.width
    }
}

fn main() {
    let mut b_test = GameBoard::new(10,20);
    b_test.print();
    b_test.set_cell(true, 5, 5);
    b_test.set_cell(true, 6, 6);
    b_test.set_cell(true, 7, 7);
    b_test.print();
}
