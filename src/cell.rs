// A cell in the grid.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
struct Cell {
    // Is it revealed?
    //
    // That is, is it showed or chosen previously by the player?
    revealed: bool,
    // Does this cell contain a flag?
    // Valid numbers are 1 through 9
    flag: u8,
    // Total level of adjacent monsters
    adjacent_tiles: u8,
    // level of the current monster
    // -1 if cell contains no monster
    level: i8,
}

