mod test;

use std::{collections::HashMap, rc::Rc};

use crate::tree::Tree;

#[derive(PartialEq)]
pub enum Cell {
    Obstacle,
    Path,
    Goal,
}

impl From<i8> for Cell {
    fn from(value: i8) -> Self {
        match value {
            0 => Cell::Path,
            2 => Cell::Goal,
            _ => Cell::Obstacle,
        }
    }
}

pub fn run_game(matrix: Vec<Vec<Cell>>) -> Vec<(usize, usize)> {
    let entry_index = matrix[0]
        .iter()
        .enumerate()
        .find_map(|(i, c)| match *c {
            Cell::Path => Some((0 as i32, i as i32)),
            _ => None,
        })
        .unwrap();

    let mut node_runned: HashMap<usize, Vec<(usize, usize)>> = HashMap::new(); // Save the walked node

    let mut nodes_to_walk = Tree::new(None);

    node_runned.insert(
        entry_index.0 as usize,
        vec![(entry_index.0 as usize, entry_index.1 as usize)],
    );
    nodes_to_walk.append_last((entry_index.0 as usize, entry_index.1 as usize));

    let mut current_index = Some(entry_index);

    while let Some((row, column)) = current_index {
        if nodes_to_walk.size == 0 || current_index.is_none() {
            break;
        }

        if let Cell::Goal =
            matrix[current_index.unwrap().0 as usize][current_index.unwrap().1 as usize]
        {
            break;
        }

        let mut new_path_cells = vec![];

        {
            let cell_to_check = vec![
                get_indices(&matrix, (row - 1, column)),
                get_indices(&matrix, (row + 1, column)),
                get_indices(&matrix, (row, column - 1)),
                get_indices(&matrix, (row, column + 1)),
            ];
            for opt_cell in cell_to_check {
                // Verify if the cell exist
                if let Some(cell) = opt_cell {
                    // Check if exist in the walked nodes
                    let exist_before = match node_runned.get(&cell.0) {
                        Some(v) => v.iter().find(|(_row, column)| *column == cell.1).is_some(),
                        None => false,
                    };
                    // If not exist in the previous cells visited, then push
                    if !exist_before {
                        new_path_cells.push((cell.0, cell.1));
                    }
                }
            }
        }

        // Add the new nodes as child of last node
        for index in &new_path_cells {
            match node_runned.get_mut(&index.0) {
                Some(v) => {
                    v.push(index.clone());
                }
                None => {
                    node_runned.insert(index.0, vec![index.clone()]);
                }
            }
            nodes_to_walk.add_child(index.clone());
        }

        if new_path_cells.is_empty() {
            // If its empty, go back, removing the last nodes
            while let Some(last_node) = nodes_to_walk.remove_last() {
                if let Some(new_last_node) = nodes_to_walk.get_last_node() {
                    // Check if have more childs
                    if !new_last_node.borrow().is_leaf() {
                        new_last_node
                            .borrow_mut()
                            .remove_child(last_node.borrow().value);

                        if new_last_node.borrow().children_len() > 0 {
                            nodes_to_walk
                                .set_last(Rc::clone(&new_last_node).borrow().get_child(0).unwrap());
                            break;
                        }
                    }
                }
            }
        } else {
            let last_node = Rc::clone(&nodes_to_walk.get_last_node().unwrap());
            nodes_to_walk.set_last(last_node.borrow().get_child(0).unwrap());
        }

        let last_value = nodes_to_walk.get_last_value();
        current_index = last_value.map(|v| (v.0 as i32, v.1 as i32));
    }

    let mut paths = vec![];

    while let Some(last_node) = nodes_to_walk.remove_last() {
        paths.push(last_node.borrow().value);
    }
    paths.reverse();
    paths
}

fn get_indices(matrix: &Vec<Vec<Cell>>, index: (i32, i32)) -> Option<(usize, usize)> {
    if index.0 < 0 || index.1 < 0 {
        return None;
    }
    if index.0 as usize >= matrix.len() || index.1 as usize >= matrix[index.0 as usize].len() {
        return None;
    }

    let opt_cell = matrix
        .get(index.0 as usize)
        .and_then(|cell| cell.get(index.1 as usize));

    match opt_cell {
        Some(cell) => match cell {
            Cell::Obstacle => None,
            _ => Some(((index.0) as usize, index.1 as usize)),
        },
        None => None,
    }
}
