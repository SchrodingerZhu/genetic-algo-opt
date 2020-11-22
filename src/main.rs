use anyhow::*;
use crate::graph::load_raw_data;
use std::time::Duration;

mod graph;
mod model;
mod simulation;

#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

pub const SINGLE_MUTATION_POSSIBILITY: f64 = 0.35;
pub const INSTANCE_MUTATION_RATE: f64 = 0.05;
pub const PAGE_FAULT_PENALTY: f64 = 10.0;
pub const CACHE_MISS_PENALTY: f64 = 5.0;
pub const ORDER_PENALTY : f64 = 2.5;
pub const DISTANCE_PENALTY : f64 = 1.0;
pub const SCALE_FACTOR: f64 = 800000.0;
pub const ICACHE_SIZE: usize = 256;
pub const PAGE_SIZE: usize = 4 * 1024;
pub const POPULATION : usize = 500;



fn main() -> Result<()> {
    pretty_env_logger::init();
    let graph = load_raw_data("in.json")?;
    let mut simulation = simulation::Simulation::new(&graph);
    simulation.start_loop(Duration::from_secs(20));
    Ok(())
}
