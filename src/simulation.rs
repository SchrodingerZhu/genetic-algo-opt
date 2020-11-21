use crate::graph::Graph;
use crate::model::Instance;
use crate::POPULATION;
use rayon::prelude::*;
use rand::seq::SliceRandom;

pub struct Simulation<'a> {
    graph: &'a Graph,
    population: Vec<Instance>,
}

impl<'a> Simulation<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        let from : Vec<usize> = (0..graph.sizes.len()).collect();
        let population = (0..POPULATION).into_par_iter()
            .map(|_| {
                let mut instance = from.clone();
                instance.as_mut_slice().shuffle(&mut rand::thread_rng());
                Instance {
                    gene: instance
                }
            }).collect();
        Simulation {
            graph, population
        }
    }
}