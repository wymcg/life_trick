use extism_pdk::*;
use lazy_static::lazy_static;
use matricks_plugin::{MatrixConfiguration, PluginUpdate};
use serde_json::from_str;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use crate::gol::GameOfLife;
use crate::util::life_state_to_matrix_state;

// How many seconds to continue playing the game until exiting the plugin
const N_SECONDS_STALE: f32 = 3.0;

// Colors for live and dead cells
const LIVE_COLOR: [u8; 4] = [255; 4];
const DEAD_COLOR: [u8; 4] = [0; 4];

lazy_static! {
    static ref CONFIG: Arc<Mutex<MatrixConfiguration>> =
        Arc::new(Mutex::new(MatrixConfiguration::default()));
    static ref GOL: Arc<Mutex<GameOfLife>> = Arc::new(Mutex::new(GameOfLife::default()));
    static ref CYCLE_COUNTER: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
}


#[plugin_fn]
pub fn setup(cfg_json: String) -> FnResult<()> {
    // Set the matrix configuration struct
    let mut config = CONFIG.lock().unwrap();
    let config = config.deref_mut();
    *config = from_str(&*cfg_json).expect("Unable to deserialize matrix config!");

    // Make a random state for the game
    let mut gol = GOL.lock().unwrap();
    let gol = gol.deref_mut();
    *gol = GameOfLife::random(config.width, config.height);

    // Setup the cycle counter to be the number of frames
    let mut cycle_counter = CYCLE_COUNTER.lock().unwrap();
    let cycle_counter = cycle_counter.deref_mut();
    *cycle_counter = (config.target_fps * N_SECONDS_STALE).round() as usize;

    Ok(())
}

#[plugin_fn]
pub fn update(_: ()) -> FnResult<Json<PluginUpdate>> {
    // Grab a reference to the cycle counter
    let mut cycle_counter = CYCLE_COUNTER.lock().unwrap();
    let cycle_counter = cycle_counter.deref_mut();

    // Grab a reference to the game of life object
    let mut gol = GOL.lock().unwrap();
    let gol = gol.deref_mut();

    // Convert the current state to a set of colors for the matrix
    let current_state = life_state_to_matrix_state(gol.state.clone(), LIVE_COLOR, DEAD_COLOR);

    // Go to the next generation
    gol.advance();

    // If the game is cycling, decrement the cycle counter
    if gol.is_cycling {
        *cycle_counter -= 1;
    }

    Ok(Json(PluginUpdate {
        state: current_state,
        done: *cycle_counter == 0, // Stop the plugin if the cycle counter has been depleted
        ..Default::default()
    }))
}
