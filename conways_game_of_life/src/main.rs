// i think this is for display/UI right? i did not need it for backend

use winit::{
    dpi::LogicalSize,
<<<<<<< HEAD
    event::{Event, WindowEvent, VirtualKeyCode},
=======
    event::{Event, VirtualKeyCode, WindowEvent},
>>>>>>> bruh
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use pixels::{Error, Pixels, SurfaceTexture};

const WIDTH: usize = 10;
const HEIGHT: usize = 10;


fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = WindowBuilder::new()
                .with_title("KAL Seagull")
                .build(&event_loop).unwrap();

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)
    };
    
    let mut life = GameBoard::new(HEIGHT, WIDTH);
    // life.print(); // display empty board
    life.set_cell(true, 5, 5);
    life.set_cell(true, 6, 5);
    life.set_cell(true, 6, 6);
    life.set_cell(true, 7, 7);
    life.set_cell(true, 8, 8);
    life.print(); // display starting board

    // // let it evolve a bit
    // life.evolve(); // 1
    // life.print();
    // life.evolve(); // 2
    // life.print();
    // life.evolve(); // 3
    // life.print(); // (same as 2 because it is a stagnant square pattern)


    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;


        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}

use winit_input_helper::WinitInputHelper;

use pixels::{
    Error, 
    Pixels, 
    SurfaceTexture,
    wgpu::Color,
};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Cell {
    pub alive: bool,
    pub row: usize,
    pub col: usize
} // actually this class is not necessary?

#[derive(Debug, Default)]
pub struct GameBoard {
    height: usize,
    width: usize, 
    board: Vec<Vec<bool>>
    // board[i][j] - i is the row number, j is the column number
    // do we need to make board Vec<Vec<Cell>>?
}

const WIN_WIDTH: f64 = 900.0;
const WIN_HEIGHT: f64 = 600.0;
const PIX_WIDTH: u32 = 150;
const PIX_HEIGHT: u32 = 100;

// Do we want to wrap the return values with Enums? (for "safe Rust")
impl GameBoard {

    pub fn new(height: usize, width: usize) -> GameBoard {
        GameBoard {
            height,
            width,
            board: vec![vec![false; width]; height]
        }
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

    // sets the board element at (row, col) to the alive parameter (true/false)
    pub fn set_cell(&mut self, alive: bool, row: usize, col: usize) {
        if row >= self.height || col >= self.width {
            panic!("invalid board position!");
        }
        self.board[row][col] = alive;
    }

    // evolves the elements of the board
    pub fn evolve(&mut self) {
        // copy of self's board to evolve
        let mut future = GameBoard::new(self.height,self.width);
        future.update(self);
        // iterate through future copy (starts out same as self's)
        for i in 0..future.height {
            for j in 0..future.width {
                if self.board[i][j] == true { // if cell at location is live
                    future.evolve_live_cell(i, j);
                } else { // else if dead
                    future.evolve_dead_cell(i, j);
                }
            }
        }
    
        // copy board back into self
        self.update(&future);
    }

    // counts the number of neighbors around board element at (row,col) that are alive
    fn count_live_neighbors(&mut self, row: usize, col: usize) -> usize {
        // live nieghbor count
        let mut n: usize = 0;
    
        // negatives are not allowed in ranges in rust :/
        // so i have to shift things and make i32 copies of the usizes
        let mut i: i32 = -1;
        let mut j: i32 = -1;

        let r: i32 = row.try_into().unwrap();
        let c: i32 = col.try_into().unwrap();
        let height: i32 = self.height.try_into().unwrap();
        let width: i32 = self.width.try_into().unwrap();

        // iterate through neighbors
        while i < 2 {
            while j < 2 {
                if i == 0 && j == 0 { // if at reference element
                    // do nothing (quick but lazy sorry)
                } else {
                    // if location is in board
                    if r + i >= 0 && r + i < height && 
                    c + j >= 0 && c + j < width {
                        // if live
                        // can only index with usize so must typecast
                        if self.board[(r + i) as usize][(c + j) as usize] == true {
                            n += 1; // increment live count
                        }
                    }
                }

                j += 1;
            }

            i += 1;
            j = -1;
        }
        
    
        // return live count
        n
    }

    fn evolve_live_cell(&mut self, row: usize, col: usize) {
        let n: usize = self.count_live_neighbors(row, col);
        if n < 2 || n > 3 { // if less than 2 or greater than 3 live neighbors
            self.set_cell(false, row, col); // set to dead
        }
    }

    fn evolve_dead_cell(&mut self, row: usize, col: usize) {
        let n: usize = self.count_live_neighbors(row, col);
        if n == 3 { // if exactly 3 live neighbors
            self.set_cell(true, row, col); // set to live
        }
    }

    fn draw(&self, frame: &mut [u8]) {
        debug_assert_eq!(frame.len(), 4 * self.width * self.height);
        let mut count: usize = 0;
        for cell in frame.chunks_exact_mut(4) {
            let col: usize = count % (PIX_WIDTH as usize);
            let row: usize = count / (PIX_WIDTH as usize);

            let color = if self.board[row][col] {
                // light blue if alive
                [0, 0xff, 0xff, 0xff]
            } else {
                // black if not alive
                [0, 0, 0, 0xff]
            };

            cell.copy_from_slice(&color);
            count += 1;
        }
    }
}
<<<<<<< HEAD

fn main() -> Result<(), Error> {
    let mut b_test = GameBoard::new(10,20);
    b_test.print(); // display empty board
    b_test.set_cell(true, 5, 5);
    b_test.set_cell(true, 6, 5);
    b_test.set_cell(true, 6, 6);
    b_test.set_cell(true, 7, 7);
    b_test.set_cell(true, 8, 8);
    b_test.print(); // display starting board

    // let it evolve a bit
    b_test.evolve(); // 1
    b_test.print();
    b_test.evolve(); // 2
    b_test.print();
    b_test.evolve(); // 3
    b_test.print(); // (same as 2 because it is a stagnant square pattern)
    
    let event_loop = EventLoop::new();
    let mut win_input = WinitInputHelper::new();
    let window = WindowBuilder::new()
                .with_title("KAL Seagull")
                .with_inner_size(LogicalSize::new(WIN_WIDTH, WIN_HEIGHT))
                .with_min_inner_size(LogicalSize::new(WIN_WIDTH, WIN_HEIGHT))
                .build(&event_loop).unwrap();
    
    // display GameBoard with pixels            
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(PIX_WIDTH, PIX_HEIGHT, surface_texture)?
    };

    // before displaying the color of the window should be white 
    // although it's never white because the program displays the gameboard instantly
    pixels.set_clear_color(Color::WHITE);

    let mut game_board = GameBoard::new(PIX_HEIGHT as usize, PIX_WIDTH as usize);
    game_board.set_cell(true, 10, 10);
    game_board.set_cell(true, 10, 11);
    game_board.set_cell(true, 10, 12);
    game_board.set_cell(true, 10, 13);
    game_board.set_cell(true, 10, 10);
    game_board.set_cell(true, 11, 11);
    game_board.set_cell(true, 12, 12);
    game_board.set_cell(true, 13, 13);

    let mut paused = false;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(_) => {
                game_board.draw(pixels.get_frame());
                pixels.render();
            },
            _ => (),
        }

        // if keys are pressed
        if win_input.update(&event) {
            // Close the window
            if win_input.key_pressed(VirtualKeyCode::Escape) || win_input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            if win_input.key_pressed(VirtualKeyCode::Space) {
                // use the space key to change the state of paused or not paused
                paused = !paused;
            }
        }

        game_board.evolve();
        /*
            we still need:
            allow user to add evolution seed in the window with mouse
            dealing with different states: paused & not paused
                - if not paused, don't detect mouse clicks
                - if it is paused, allow player to add seeds
            - make the board evolve itself regularly
            - make sure between each evolution there is a period of stopping at that state
                - so that player can see the current state 
        */
    });
}
=======
>>>>>>> bruh
