// code is copied from https://rosettacode.org/wiki/Conway's_Game_of_Life#Rust

use std::collections::HashMap;
use std::collections::HashSet;

pub type Cell = (i32, i32);
pub type Colony = HashSet<Cell>;

fn neighbours(&(x, y): &Cell) -> Vec<Cell> {
    vec![
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
}

fn neighbour_counts(col: &Colony) -> HashMap<Cell, i32> {
    let mut ncnts = HashMap::new();
    for cell in col.iter().flat_map(neighbours) {
        *ncnts.entry(cell).or_insert(0) += 1;
    }
    ncnts
}

// Game of Life, B3S23 variation
pub fn generation(col: Colony) -> Colony {
    neighbour_counts(&col)
        .into_iter()
        .filter_map(|(cell, cnt)| match (cnt, col.contains(&cell)) {
            (2, true) | (3, ..) => Some(cell),
            _ => None,
        })
        .collect()
}
