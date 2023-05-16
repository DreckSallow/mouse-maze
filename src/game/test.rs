use super::{run_game, Cell};

fn create_matrix_1_path() -> Vec<Vec<Cell>> {
    let mut matrix = vec![];
    matrix.push(vec![-1, 0, -1]);
    matrix.push(vec![0, 0, 0]);
    matrix.push(vec![0, -1, 0]);
    matrix.push(vec![-1, 2, 0]);

    matrix
        .iter()
        .map(|v| v.iter().map(|n| Cell::from(*n as i8)).collect())
        .collect()
}

fn create_matrix_large() -> Vec<Vec<Cell>> {
    let mut matrix = vec![];
    matrix.push(vec![-1, -1, 0, -1, -1]);
    matrix.push(vec![-1, 0, 0, 0, -1]);
    matrix.push(vec![0, -1, 0, 0, 0]);
    matrix.push(vec![0, 0, -1, -1, 0]);
    matrix.push(vec![-1, 0, 2, 0, 0]);

    matrix
        .iter()
        .map(|v| v.iter().map(|n| Cell::from(*n as i8)).collect())
        .collect()
}

#[test]
fn test_game_1_path() {
    let path = run_game(create_matrix_1_path());
    let correct_result = vec![(0, 1), (1, 1), (1, 2), (2, 2), (3, 2), (3, 1)];
    assert_eq!(path, correct_result)
}

#[test]
fn test_game_large() {
    let path = run_game(create_matrix_large());
    let correct_result = vec![
        (0, 2),
        (1, 2),
        (2, 2),
        (2, 3),
        (2, 4),
        (3, 4),
        (4, 4),
        (4, 3),
        (4, 2),
    ];
    assert_eq!(path, correct_result)
}
