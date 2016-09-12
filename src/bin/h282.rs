//! [Hard] Hidato
//!
//! Description
//! ===========
//! From wikipedia
//! Hidato (Hebrew: חידאתו‎‎, originating from the Hebrew word Hida = Riddle) is a logic puzzle game
//! invented by Dr. Gyora M. Benedek, an Israeli mathematician. The goal of Hidato is to fill the
//! grid with consecutive numbers that connect horizontally, vertically, or diagonally. Numbrix
//! puzzles, created by Marilyn vos Savant, are similar to Hidato except that diagonal moves are
//! not allowed. Jadium puzzles (formerly Snakepit puzzles), created by Jeff Marchant, are a more
//! difficult version of Numbrix with fewer given numbers and have appeared on the Parade magazine
//! web site regularly since 2014. The name Hidato is a registered trademark of Doo-Bee Toys and
//! Games LTD, a company co-founded by Benebek himself. Some publishers use different names for
//! this puzzle such as Number Snake.
//!
//! Further info:
//! In Hidato, a grid of cells is given. It is usually square-shaped, like Sudoku or Kakuro, but it
//! can also include irregular shaped grids like hearts, skulls, and so forth. It can have inner
//! holes (like a disc), but it has to be made of only one piece. The goal is to fill the grid with
//! a series of consecutive numbers adjacent to each other vertically, horizontally, or diagonally.
//! In every Hidato puzzle the smallest and the highest numbers are given on the grid. There are
//! also other given numbers on the grid (with values between the smallest and the highest) to help
//! direct the player how to start the solution and to ensure that Hidato has a single solution.
//! Note: the above condition on the smallest or highest numbers are sometimes relaxed: only their
//! values can be given, without their positions on the grid (of course, the difference between
//! these values must be equal to the number of cells in the grid minus one). This may lead to
//! harder puzzles. Every well-formed Hidato puzzle is supposed to have a unique solution.
//! Moreover, a Hidato puzzle intended for human solvers should have a solution that can be found
//! by (more or less) simple logic. However, there exist very hard Hidato puzzles, even of small
//! size. Hidato puzzles are published in newspapers such as the Daily Mail and Detroit Free Press.
//!
//! So basically:
//! You'll recieve a grid with numbers, empty spaces and blocked spaces.
//! You need to fill in all empty spaces with numbers. These numbers must be consecutive that
//! connect in any direction.
//!
//! Notes/Hints
//! ===========
//! Also from wikipedia
//! As in many logic puzzles, the basic resolution technique consists of analyzing the
//! possibilities for each number of being present in each cell. When a cell can contain only one
//! number (Naked Single) or when a number has only one possible place (Hidden Single), it can be
//! asserted as belonging to the solution. One key to the solution is, it does not have to be built
//! in ascending (or descending) order; it can be built piecewise, with pieces starting from
//! different givens. As in the Sudoku case, the resolution of harder Hidato or Numbrix puzzles
//! requires the use of more complex techniques - in particular of various types of chain patterns.
//!
//! https://www.reddit.com/r/dailyprogrammer/comments/51wg0j/20160909_challenge_282_hard_hidato/

fn main() {

}

struct Hidato {
    datas: Vec<Vec<Cell>>
}

type Candidates = Vec<usize>;

enum Cell {
    Empty(Candidates),
    Blocked,
    Fixed(usize),
    Guess(usize),
}