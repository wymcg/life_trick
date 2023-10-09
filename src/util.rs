use std::collections::HashSet;

/// Get the neighboring coordinates of a given coordinate. Wraps if a neigbor is out of bounds
pub fn get_neighboring_coordinates(
    coord: (usize, usize),
    width: usize,
    height: usize,
) -> HashSet<(usize, usize)> {
    let left_coord: usize = (coord.0 + width - 1) % width;
    let right_coord: usize = (coord.0 + 1) % width;
    let top_coord: usize = (coord.1 + height - 1) % height;
    let bottom_coord: usize = (coord.1 + 1) % height;

    HashSet::from([
        (left_coord, bottom_coord),
        (coord.0, bottom_coord),
        (right_coord, bottom_coord),
        (left_coord, coord.1),
        (right_coord, coord.1),
        (left_coord, top_coord),
        (coord.0, top_coord),
        (right_coord, top_coord),
    ])
}

/// Convert a Game of Life state to a set of colors that Matricks can use
pub fn life_state_to_matrix_state(
    life_state: Vec<Vec<bool>>,
    live_color: [u8; 4],
    dead_color: [u8; 4],
) -> Vec<Vec<[u8; 4]>> {
    life_state
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell_state| if *cell_state { live_color } else { dead_color })
                .collect()
        })
        .collect()
}
