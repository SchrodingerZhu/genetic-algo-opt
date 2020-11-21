use anyhow::*;

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



fn main() {
    pretty_env_logger::init_timed();
    let a = model::Instance {
        gene: vec![1, 5, 9, 2, 4, 6, 7, 8, 3, 0]
    };
    let b = model::Instance {
        gene: vec![9, 0, 2, 7, 8, 6, 3, 5, 4, 1]
    };
    let mu = model::Instance::crossover(&a, &b);
    println!("{:?}", mu);
}
