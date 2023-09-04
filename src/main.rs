extern crate termion;

mod cell;
mod board;

use board::Difficulty;

use termion::{clear, cursor, color, style};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;
use termion::event::Key::Char;

use std::env;
use std::io::{self, Read, Write};
use std::process;

// The help page.
const HELP: &'static str = r#"
Description:
    This is a cross between Minesweeper and an RPG.
    You gain levels by killing weak monsters and win when you defeat them all.
    It's a bit different than Minesweeper in that the number you reveal when
    you click on a square is the total of the levels of the monsters
    in adjacent squares. 

    When you select a tile with a monster in it, a battle begins.
    First, you attack and deal damage (equal to your level) to the monster.
    On the next turn, if the monster is still alive, it attacks you back.
    Your attacks alternate until one of you dies.

Flags:
    -d | --difficulty <EASY, HUGE, EXTREME, BLIND> ~ set the difficulty
    -h | --help                     ~ this help page.

Legend:
    HP: Hit points. If these drop to 0, you lose.
    LV: Level. This is how much damage you deal to monsters.
    EX: Experience. Collect enough to level up.
    NE: Remaining experience required to reach the next level.

Controls:
    ---selection--------------------
    space ~ open the selected cell
    space ~ switch between already defeated monster and the number for its square
    ---movement---------------------
    h | a ~ move left.
    j | s ~ move down.
    k | w ~ move up.
    l | d ~ move right.
    ---flags------------------------
    1-9   ~ set flag.
    0     ~ remove flag.
    ---control----------------------
    q     ~ quit game.
    r     ~ restart game.

Credit:
    hojamaja.com.
"#;

// The game state.
struct Game<R, W: Write> {
    /// Represents the array of Cells
    board: board::Board,
    /// level
    lv: u8,
    /// health points
    hp: i8,
    /// experience points
    exp: u16,
    /// experience needed for next level
    ne: u16,
    /// Standard output.
    stdout: W,
    /// Standard input.
    stdin: R,
    /// x position of cursor
    x: u16,
    /// y position of cursor
    y: u16,
}


// Initialize the game.
fn init<W: Write, R: Read>(mut stdout: W, stdin: R, difficulty: Difficulty) {
    write!(stdout, "{}", clear::All).unwrap();

    // Set the initial game state.
    let mut game = Game {
        stdout,
        board: board::Board::new(difficulty),
        stdin: stdin.keys(),
        lv: 0,
        hp: 0,
        exp: 0,
        ne: 0,
        x: 2,
        y: 2,
    };

    // Reset the game.
    game.reset();

    // Start the event loop.
    game.start();
}

impl<R, W: Write> Drop for Game<R, W> {
    fn drop(&mut self) {
        // When done, restore the defaults to avoid messing with the terminal.
        write!(
            self.stdout,
            "{}{}{}",
            clear::All,
            style::Reset,
            cursor::Goto(1, 1)
        ).unwrap();
    }
}

impl<R: Iterator<Item = Result<Key, std::io::Error>>, W: Write> Game<R, W> {

    // Start the game loop.
    //
    // This will listen to events and do the appropriate actions.
    fn start(&mut self) {
        loop {
            // Read a single byte from stdin.
            let b = self.stdin.next().unwrap().unwrap();
            match b {
                Char('h') | Char('a') | Key::Left => self.x = left(self.x, self.board.width),
                Char('j') | Char('s') | Key::Down => self.y = down(self.y, self.board.height),
                Char('k') | Char('w') | Key::Up => self.y = up(self.y, self.board.height),
                Char('l') | Char('d') | Key::Right => self.x = right(self.x, self.board.width),
                Char(' ') => {
                    let (x, y, width) = (self.x, self.y, self.board.width);

                    // if the player level == monster level
                    // kill it and get experience
                    // else
                    // perform the battle
                    // each "round" player loses monster level health until monster is dead or
                    // player is dead

                    // if the cell has a monster
                    if let Some(level) = self.board.grid[pos(x, y, width)].level {
                        //self.reveal_all();
                        // Make the background colour of the mine we just
                        // landed on red, and the foreground black.
                        write!(
                            self.stdout,
                            "{}{}{}{}{}",
                            cursor::Goto(x + 2, y + 2),
                            color::Bg(color::Red),
                            color::Fg(color::Black),
                            level,
                            style::Reset
                        ).unwrap();
                        self.game_over();
                        return;
                    }
                    // if the cell doesn't have a monster. level == None
                    else {
                        //self.reveal_all();
                    }

                    if !self.board.grid[pos(x, y, self.board.width)].revealed {
                        let temp = x + 1;
                    }

                    // Reveal the cell.
                    self.reveal(x, y);
                }
                Char('r') => {
                    self.restart();
                    return;
                }
                Char('q') => return,
                _ => {}
            }

            // Make sure the cursor is placed on the current position.
            write!(self.stdout, "{}", cursor::Goto(self.x + 2, self.y + 2)).unwrap();
            self.stdout.flush().unwrap();
        }
    }

    fn draw(&mut self) {
        // Write the upper part of the frame.
        self.stdout.write(board::Window::TopLeftCorner.to_string().as_bytes()).unwrap();
        for _ in 0..self.board.width {
            self.stdout.write(board::Window::HorzBoundary.to_string().as_bytes()).unwrap();
        }
        self.stdout.write(board::Window::TopRightCorner.to_string().as_bytes()).unwrap();
        self.stdout.write(b"\n\r").unwrap();

        // Conceal all the cells.
        for _ in 0..self.board.height {
            // The left part of the frame
            self.stdout.write(board::Window::VertBoundary.to_string().as_bytes()).unwrap();

            for _ in 0..self.board.width {
                self.stdout.write_all(board::Window::Concealed.to_string().as_bytes()).unwrap();
            }

            // The right part of the frame.
            self.stdout.write(board::Window::VertBoundary.to_string().as_bytes()).unwrap();
            self.stdout.write(b"\n\r").unwrap();
        }

        // Write the lower part of the frame.
        self.stdout.write(board::Window::BottomLeftCorner.to_string().as_bytes()).unwrap();
        for _ in 0..self.board.width {
            self.stdout.write(board::Window::HorzBoundary.to_string().as_bytes()).unwrap();
        }
        self.stdout.write(board::Window::BottomRightCorner.to_string().as_bytes()).unwrap();
    }

    // Reset the game.
    //
    // This will display the starting grid, fill the grid with monsters, and then number the tiles
    fn reset(&mut self) {
        // Reset the cursor.
        write!(self.stdout, "{}", cursor::Goto(1, 1)).unwrap();
        self.draw();
        write!(self.stdout, "{}", cursor::Goto(self.x + 2, self.y + 2)).unwrap();
        self.stdout.flush().unwrap();
        self.board.reset();
    }

    // Get the value of a cell.
    //
    // The value represent the sum of adjacent cells containing mines. A cell of value, 0, is
    // called "free".
    fn val(&mut self, x: u16, y: u16) -> u8 {
        adjacent(x, y, self.board.width, self.board.height)
            .iter()
            .filter_map(|(x, y)| self.board.grid[pos(*x, *y, self.board.width)].level)
            .sum()
    }

    // Reveal the cell, _c_.
    //
    // This will recursively reveal free cells, until non-free cell is reached, terminating the
    // current recursion descendant.
    fn reveal(&mut self, x: u16, y: u16) {
        let v = self.val(x, y);

        self.board.grid[pos(x, y, self.board.width)].revealed = true;

        write!(self.stdout, "{}", cursor::Goto(x + 2, y + 2)).unwrap();

        if v == 0 {
            // If the cell is free, simply put a space on the position.
            self.stdout.write(b" ").unwrap();

            //adjacent(x, y, self.board.width, self.board.height)
            //    .iter()
            //    .filter(|(x, y)| self.board.grid[pos(*x, *y, self.board.width)].revealed);
            // Recursively reveal adjacent cells until a non-free cel is reached.
            for &(x, y) in adjacent(x, y, self.board.width, self.board.height).iter() {
                if !self.board.grid[pos(x, y, self.board.width)].revealed && !self.board.grid[pos(x, y, self.board.width)].level.is_none() {
                    self.reveal(x, y);
                }
            }
        } else {
            self.stdout.write(&[b'0' + v]).unwrap();
        }
    }

    // Reveal all the fields, printing where the mines were.
    //fn reveal_all(&mut self) {
    //    write!(self.stdout, "{}", cursor::Goto(1, 1)).unwrap();

    //    write!(self.stdout, "{}", cursor::Goto(self.x + 2, self.y + 2)).unwrap();
    //    self.board.grid.iter_mut().for_each(|c| {
    //        if let None = c.level {
    //            self.stdout.write("M".as_bytes()).unwrap();
    //        }
    //    });
    //}

    // Game over!
    fn game_over(&mut self) {
        write!(self.stdout, "{}", cursor::Goto(1, 1)).unwrap();
        //Goto top left corner

        self.stdout.write("GAME_OVER".as_bytes()).unwrap();
        self.stdout.flush().unwrap();

        loop {
            // Repeatedly read a single byte.
            match self.stdin.next().unwrap().unwrap() {
                Key::Char('r') => {
                    // Replay!
                    self.restart();
                    return;
                }
                Key::Char('q') => return,
                _ => {}
            }
        }
    }

    // Restart (replay) the game.
    fn restart(&mut self) {
        self.reset();
        self.start();
    }

}

// Get the grid position of a given coordinate.
pub fn pos(x: u16, y: u16, width: u16) -> usize {
    y as usize * width as usize + x as usize
}

// Calculate the adjacent cells.
pub fn adjacent(x: u16, y: u16, width: u16, height: u16) -> [(u16, u16); 8] {
    let left = left(x, width);
    let right = right(x, width);
    let up = up(y, height);
    let down = down(y, height);

    [
        // Left-up
        (left, up),
        // Up
        (x, up),
        // Right-up
        (right, up),
        // Left
        (left, y),
        // Right
        (right, y),
        // Left-down
        (left, down),
        // Down
        (x, down),
        // Right-down
        (right, down),
    ]
}

// Calculate the y coordinate of the cell "above" a given y coordinate.
fn up(y: u16, height: u16) -> u16 {
    (y - 1) % height
}
// Calculate the y coordinate of the cell "below" a given y coordinate.
fn down(y: u16, height: u16) -> u16 {
    (y + 1) % height
}
// Calculate the x coordinate of the cell "left to" a given x coordinate.
fn left(x: u16, width: u16) -> u16 {
    (x - 1) % width
}
// Calculate the x coordinate of the cell "right to" a given x coordinate.
fn right(x: u16, width: u16) -> u16 {
    (x + 1) % width
}

fn main() {
    let mut args = env::args().skip(1);
    let diff: Option<board::Difficulty> = Some(Difficulty::EASY);

    // Get and lock the stdios.
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let stderr = io::stderr();
    let mut stderr = stderr.lock();

    loop {
        // Read the arguments.
        // Does not use a for loop because each argument may have second parameter.

        let arg = if let Some(x) = args.next() {
            x
        } else {
            break;
        };

        match arg.as_str() {
            "-h" | "--help" => {
                // Print the help page.
                stdout.write(HELP.as_bytes()).unwrap();
                stdout.flush().unwrap();
                process::exit(0);
            }
            _ => {
                stderr.write(b"Unknown argument.\n").unwrap();
                stderr.flush().unwrap();
                process::exit(1);
            }
        }
    }

    // We go to raw mode to make the control over the terminal more fine-grained.
    let stdout = stdout.into_raw_mode().unwrap();

    let termsize = termion::terminal_size().ok();
    let termwidth = termsize.map(|(w, _)| w - 2);
    let termheight = termsize.map(|(_, h)| h - 2);
    if let Some(difficulty) = diff {
        // Initialize the game!
        init(
            stdout,
            stdin,
            difficulty,
        );
    }
}
