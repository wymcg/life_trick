use crate::gol::GameOfLife;
use crate::util::life_state_to_matrix_state;
use extism_pdk::config;
use extism_pdk::*;
use lazy_static::lazy_static;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

// How many seconds to continue playing the game until exiting the plugin
const N_SECONDS_STALE: f32 = 3.0;

// Colors for live and dead cells
const LIVE_COLOR: [u8; 4] = [255; 4];
const DEAD_COLOR: [u8; 4] = [0; 4];

#[host_fn]
extern "ExtismHost" {
    fn matricks_debug(input: &str);
    fn matricks_info(input: &str);
    fn matricks_warn(input: &str);
    fn matricks_error(input: &str);
}

lazy_static! {
    static ref GOL: Arc<Mutex<GameOfLife>> = Arc::new(Mutex::new(GameOfLife::default()));
    static ref CYCLE_COUNTER: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
}

#[plugin_fn]
pub fn setup(_: ()) -> FnResult<()> {
    unsafe { matricks_info("Starting up the Life Trick")? };

    let width: usize = config::get("width").unwrap().parse().unwrap();
    let height: usize = config::get("height").unwrap().parse().unwrap();
    let target_fps: f32 = config::get("target_fps").unwrap().parse().unwrap();

    // Make a random state for the game
    let mut gol = GOL.lock().unwrap();
    let gol = gol.deref_mut();
    *gol = GameOfLife::random(width, height);

    // Setup the cycle counter to be the number of frames
    let mut cycle_counter = CYCLE_COUNTER.lock().unwrap();
    let cycle_counter = cycle_counter.deref_mut();
    *cycle_counter = (target_fps * N_SECONDS_STALE).round() as usize;
    unsafe { matricks_info("Created cycle counter.")? };
    unsafe { matricks_debug(&format!("Cycle counter is {cycle_counter} frames."))? };

    Ok(())
}

#[plugin_fn]
pub fn update(_: ()) -> FnResult<Json<Option<Vec<Vec<[u8; 4]>>>>> {
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

    let next_state = if *cycle_counter == 0 {
        Ok(Json(None))
    } else {
        Ok(Json(Some(current_state)))
    };

    // If the game is cycling, decrement the cycle counter
    if gol.is_cycling {
        *cycle_counter -= 1;
    }

    next_state
}
