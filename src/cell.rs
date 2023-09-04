#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Cell {
    /// Determines if the cell has been revealed to the player
    pub revealed: bool,
    /// Total level of adjacent monsters
    pub adjacent_tiles: u8,
    /// The player's estimated level of this cell
    /// Valid values are Some(1..=9) or None
    pub flag: Option<u8>,
    /// Level of the monster occupying this cell
    /// Valid values are Some(1..=9) or None
    pub level: Option<u8>,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            revealed: false,
            adjacent_tiles: 0,
            flag: None,
            level: None,
        }
    }

    // Set a flag on cell.
    fn set_flag(&mut self, level: u8) {
        if !self.revealed {
            self.flag = Some(level);
        }
    }

    // Remove a flag on cell.
    fn remove_flag(&mut self) {
        self.flag = None;
    }
}
