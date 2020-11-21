use log::*;
use rand::seq::SliceRandom;
use rayon::prelude::*;

use crate::graph::Graph;
use crate::model::Instance;
use crate::POPULATION;
use std::cmp::Ordering::Equal;
use rand::distributions::{Uniform, Distribution};

struct PrefixSum {
    prefix: Vec::<f64>
}

impl PrefixSum {
    pub fn new(fitness: &[f64]) -> Self {
        let mut prefix = Vec::new();
        prefix.push(0f64);
        for i in 0..fitness.len() {
            prefix.push(prefix.last().unwrap() + fitness[i]);
        }
        PrefixSum { prefix }
    }
    pub fn search(&self, f: f64) -> usize {
        match self.prefix.binary_search_by(|a| a.partial_cmp(&f).unwrap_or(Equal)) {
            Ok(n) => n,
            Err(e) => e - 1
        }
    }
    pub fn all(&self) -> f64 {
        *self.prefix.last().unwrap()
    }
}

pub struct Simulation<'a> {
    graph: &'a Graph,
    population: Vec<Instance>,
}

impl<'a> Simulation<'a> {
    pub fn new(graph: &'a Graph) -> Self {
        let from: Vec<usize> = (0..graph.sizes.len()).collect();
        let population = (0..POPULATION).into_par_iter()
            .map(|_| {
                let mut instance = from.clone();
                instance.as_mut_slice().shuffle(&mut rand::thread_rng());
                Instance {
                    gene: instance
                }
            }).collect();
        Simulation {
            graph,
            population,
        }
    }

    pub fn fitness_vector(&self) -> Vec<f64> {
        self.population
            .iter()
            .map(|x| return x.fitness(self.graph))
            .collect()
    }

    pub fn simulate(&mut self) {
        let fitness = self.fitness_vector();
        let prefix_sum = PrefixSum::new(&fitness);
        let new_population = (0..POPULATION)
            .into_par_iter()
            .map(|_| {
                let mut rng = rand::thread_rng();
                let uniform : Uniform<f64> = Uniform::new(0.0, prefix_sum.all());
                let (f, m) = (uniform.sample(&mut rng), uniform.sample(&mut rng));
                let (f, m) = (prefix_sum.search(f), prefix_sum.search(m));
                Instance::mate(&self.population[f], &self.population[m])
            }).collect();
        self.population = new_population;
    }
}

#[cfg(test)]
mod test {
    use crate::simulation::PrefixSum;

    #[test]
    fn test_prefix_sum() {
        let targets = [5.0, 2.0, 1.0];
        let prefix = PrefixSum::new(&targets);
        assert_eq!(prefix.search(6.5), 1);
        assert_eq!(prefix.search(4.9), 0);
        assert_eq!(prefix.search(7.1), 2);
    }
}