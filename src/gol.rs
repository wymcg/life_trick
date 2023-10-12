use crate::util::get_neighboring_coordinates;
use rand::Rng;
use std::collections::HashSet;

type LifeState = Vec<Vec<bool>>;

pub struct GameOfLife {
    pub state: LifeState,
    width: usize,
    height: usize,
    visited_states: HashSet<LifeState>,
    pub is_cycling: bool,
}

impl Default for GameOfLife {
    fn default() -> Self {
        Self::new(LifeState::default(), 0, 0)
    }
}

impl GameOfLife {
    /// Make a new game with the given initial state and dimensions
    pub fn new(state: LifeState, width: usize, height: usize) -> Self {
        Self {
            state,
            width,
            height,
            visited_states: HashSet::<LifeState>::new(),
            is_cycling: false,
        }
    }

    /// Make a new game, with a random initial state
    pub fn random(width: usize, height: usize) -> Self {
        let mut rng = rand::thread_rng();

        // Make a new state with random values
        let mut state = vec![];
        for y in 0..height {
            state.push(vec![]);
            for _x in 0..width {
                state[y].push(rng.gen());
            }
        }

        // Return a new game object
        Self::new(state, width, height)
    }

    /// Update the game to the next generation
    pub fn advance(&mut self) -> LifeState {
        // Determine the next game state
        let mut next_state = LifeState::new();
        for y in 0..self.height {
            next_state.push(vec![]);
            for x in 0..self.width {
                next_state[y].push(self.next_cell_state((x, y)));
            }
        }

        // Check if the next state has been seen before
        if self.visited_states.contains(&next_state) {
            // If we have seen this state before, we must be in a cycle
            self.is_cycling = true
        } else {
            // If we haven't seen this state before, mark it as seen
            self.visited_states.insert(next_state.clone());
        }

        // Swap the state
        self.state = next_state.clone();

        next_state
    }

    /// Find the next state for a given cell
    fn next_cell_state(&self, coord: (usize, usize)) -> bool {
        let n_neighbors = self.count_live_neighbors(coord);
        let cell_is_live = self.is_cell_live(coord);

        if cell_is_live && (n_neighbors == 2 || n_neighbors == 3) {
            true
        } else if !cell_is_live && (n_neighbors == 3) {
            true
        } else {
            false
        }
    }

    /// Find the number of live neighbors of a given cell
    fn count_live_neighbors(&self, coord: (usize, usize)) -> usize {
        get_neighboring_coordinates(coord, self.width, self.height)
            .iter()
            .filter(|coord| self.is_cell_live(**coord))
            .count()
    }

    /// Check if a cell is live or dead
    fn is_cell_live(&self, coord: (usize, usize)) -> bool {
        self.state[coord.1][coord.0]
    }
}
