mod game;
mod node;
mod tree;

use game::Cell;

use crate::game::run_game;

fn main() {
    println!("path: {:?}", run_game(create_matrix()))
}

fn create_matrix() -> Vec<Vec<Cell>> {
    let mut matrix = vec![];
    matrix.push(vec![Cell::Obstacle, Cell::Path, Cell::Obstacle]);
    matrix.push(vec![Cell::Path, Cell::Path, Cell::Path]);
    matrix.push(vec![Cell::Path, Cell::Obstacle, Cell::Path]);
    matrix.push(vec![Cell::Obstacle, Cell::Goal, Cell::Path]);

    matrix
}
