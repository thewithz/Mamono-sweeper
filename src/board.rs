enum Difficulty {
    EASY,
    HUGE,
    EXTREME,
    BLIND,
}

// The string printed for concealed cells.
const CONCEALED: &'static str = "▒";

// The upper and lower boundary char.
const HORZ_BOUNDARY: &'static str = "─";
// The left and right boundary char.
const VERT_BOUNDARY: &'static str = "│";

// The top-left corner
const TOP_LEFT_CORNER: &'static str = "┌";
// The top-right corner
const TOP_RIGHT_CORNER: &'static str = "┐";
// The bottom-left corner
const BOTTOM_LEFT_CORNER: &'static str = "└";
// The bottom-right corner
const BOTTOM_RIGHT_CORNER: &'static str = "┘";
