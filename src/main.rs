use anyhow::*;
use crate::graph::{load_raw_data, Graph};
use std::time::Duration;

mod graph;
mod model;
mod simulation;

#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;
pub const SINGLE_MUTATION_POSSIBILITY: f64 = 0.5;
pub const INSTANCE_MUTATION_RATE: f64 = 0.05;
pub const CROSS_OVER_LOW: usize = 10;
pub const CROSS_OVER_HIGH: usize = 20;
pub const PAGE_FAULT_PENALTY: f64 = 100.0;
pub const CACHE_MISS_PENALTY: f64 = 50.0;
pub const SCALE_FACTOR: f64 = 10000.0;
pub const ICACHE_SIZE: usize = 256;
pub const PAGE_SIZE: usize = 2048 * 1024;
pub const POPULATION : usize = 100;



fn main() -> Result<()> {
    pretty_env_logger::init_timed();
    let graph = load_raw_data("in.json")?;
    let mut simulation = simulation::Simulation::new(&graph);
    simulation.start_loop(Duration::from_secs(120));
    Ok(())
}
