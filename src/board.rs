use cell::Cell;

use std::fmt::Display;
use std::fmt::Result;
use std::fmt::Formatter;

const SIZES: [(u16,u16); 4] = [(16,16), (50,25), (30,16), (30,16)];

// The information panel
pub const HUD: &'static str = "╔═════════════════════════╗\n\r\
                               ║                         ║\n\r\
                               ║ LV:{} HP:{} EX:{} NE:{} ║\n\r\
                               ╚═════════════════════════╝";


#[derive(Copy, Clone)]
pub enum Difficulty {
    EASY,
    //NORMAL,
    HUGE,
    EXTREME,
    BLIND,
}

pub enum Window {
    // The string printed for concealed cells.
    Concealed,
    // The upper and lower boundary char.
    HorzBoundary,
    // The left and right boundary char.
    VertBoundary,
    
    // The four corners
    TopLeftCorner,
    TopRightCorner,
    BottomLeftCorner,
    BottomRightCorner,
}

impl Display for Window {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let printable = match *self {
            Window::Concealed => "▒",
            Window::HorzBoundary => "─",
            Window::VertBoundary => "│",
            Window::TopLeftCorner => "┌",
            Window::TopRightCorner => "┐",
            Window::BottomLeftCorner => "└",
            Window::BottomRightCorner => "┘",
        };
        write!(f, "{}", printable)
    }
}

pub struct Board {
    /// Height of the grid.
    pub height: u16,
    /// Width of the grid.
    pub width: u16,
    /// The grid.
    ///
    /// The cells are enumerated like you would read a book. Left to right, until you reach the
    /// line ending.
    pub grid: Vec<Cell>,
    /// The difficulty of the game.
    pub difficulty: Difficulty,
}

impl Board {
    pub fn new(difficulty: Difficulty) -> Board {
        let width = SIZES[difficulty as usize].0;
        let height = SIZES[difficulty as usize].1;
        let board = Board {
            width,
            height,
            difficulty,
            grid: vec![
                Cell::new();
                width as usize * height as usize
            ],
        };
        board.place_monsters(difficulty);
        board
    }

    pub fn reset(&mut self) {
        self.grid.iter_mut()
            .for_each(|c| *c = Cell::new());
        self.place_monsters(self.difficulty);
    }

    fn place_monsters(&self, difficulty: Difficulty) {
        unimplemented!("Place monsters according to current board difficulty")       
    }
}

