use anyhow::*;
use structopt::StructOpt;
use crate::graph::load_raw_data;

mod graph;
mod model;
mod simulation;

#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

#[derive(structopt::StructOpt)]
pub struct Conf {
    #[structopt(long, short, default_value = "0.35")]
    pub single_mutation_possibility: f64,
    #[structopt(long, short = "r", default_value = "0.05")]
    pub instance_mutation_rate: f64,
    #[structopt(long, short = "w", default_value = "10.0")]
    pub page_fault_penalty: f64,
    #[structopt(long, short, default_value = "5.0")]
    pub cache_miss_penalty: f64,
    #[structopt(long, short, default_value = "4.0")]
    pub order_penalty: f64,
    #[structopt(long, short = "x", default_value = "0.02")]
    pub cross_over_possibility: f64,
    #[structopt(long, short, default_value = "0.01")]
    pub distance_penalty: f64,
    #[structopt(long, short = "f", default_value = "50000000.0")]
    pub scale_factor: f64,
    #[structopt(long, short, default_value = "256")]
    pub icache_size: usize,
    #[structopt(long, short, default_value = "4096")]
    pub page_size: usize,
    #[structopt(long, short = "k", default_value = "500")]
    pub population: usize,
    #[structopt(long, short = "t", default_value = "60")]
    pub simulation_time: usize,
    #[structopt(long, short, default_value = "100")]
    pub log_factor: usize,
    #[structopt(long, short = "g", default_value = "INFO", env = "SIM_LOG_LEVEL")]
    pub log_level: String,
}


fn main() -> Result<()> {
    let conf = Conf::from_args();
    std::env::set_var("SIM_LOG_LEVEL", &conf.log_level);
    pretty_env_logger::try_init_timed_custom_env("SIM_LOG_LEVEL")?;
    let graph = load_raw_data("in.json")?;
    let mut simulation = simulation::Simulation::new(&graph, conf);
    simulation.start_loop();
    Ok(())
}
