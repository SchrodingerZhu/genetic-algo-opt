use std::cmp::Ordering::Equal;
use std::time::Duration;

use log::*;
use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use rayon::prelude::*;

use crate::graph::Graph;
use crate::model::Instance;
use crate::Conf;

use statrs::statistics::Statistics;

struct PrefixSum {
    prefix: Vec<f64>,
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
        match self
            .prefix
            .binary_search_by(|a| a.partial_cmp(&f).unwrap_or(Equal))
        {
            Ok(n) => n,
            Err(e) => e - 1,
        }
    }
    pub fn all(&self) -> f64 {
        *self.prefix.last().unwrap()
    }
}

pub struct Simulation<'a> {
    graph: &'a Graph,
    population: Vec<Instance>,
    conf: crate::Conf,
}

impl<'a> Simulation<'a> {
    pub fn new(graph: &'a Graph, conf: Conf) -> Self {
        let from: Vec<usize> = (0..graph.sizes.len()).collect();
        let population = (0..conf.population)
            .into_par_iter()
            .map(|_| {
                let mut instance = from.clone();
                instance.as_mut_slice().shuffle(&mut rand::thread_rng());
                Instance { gene: instance }
            })
            .collect();
        Simulation { graph, population, conf }
    }

    pub fn fitness_vector(&self) -> Vec<f64> {
        self.population
            .iter()
            .map(|x| return x.fitness(self.graph,
                                      self.conf.page_size,
                                      self.conf.icache_size,
                                      self.conf.page_fault_penalty,
                                      self.conf.cache_miss_penalty,
                                      self.conf.distance_penalty,
                                      self.conf.order_penalty,
                                      self.conf.scale_factor))
            .collect()
    }

    pub fn simulate(&mut self) -> f64 {
        let fitness = self.fitness_vector();
        let prefix_sum = PrefixSum::new(&fitness);
        let new_population = (0..self.conf.population)
            .into_par_iter()
            .map(|_| {
                let mut rng = rand::thread_rng();
                let uniform: Uniform<f64> = Uniform::new(0.0, prefix_sum.all());
                let (f, m) = (uniform.sample(&mut rng), uniform.sample(&mut rng));
                let (f, m) = (prefix_sum.search(f), prefix_sum.search(m));
                Instance::mate(&self.population[f], &self.population[m],
                               self.conf.cross_over_possibility,
                               self.conf.instance_mutation_rate,
                               self.conf.single_mutation_possibility)
            })
            .collect();
        self.population = new_population;
        fitness.max()
    }
    pub fn start_loop(&mut self) {
        let start_time = std::time::SystemTime::now();
        let mut round = 0_usize;
        let duration = Duration::from_secs(self.conf.simulation_time as u64);
        loop {
            let now = std::time::SystemTime::now();
            let used = now.duration_since(start_time).unwrap();
            if used.ge(&duration) {
                info!("simulation finished");
                break;
            }
            let max_fit = self.simulate();
            if round % self.conf.log_factor == 0 {
                info!("simulation round #{}, time used: {}s, max_fit: {}", round, used.as_secs(), max_fit);
            }
            round = round + 1;
        }
        let fitness = self.fitness_vector();
        let mut index = 0;
        for i in 0..fitness.len() {
            if fitness[i] > fitness[index] {
                index = i;
            }
        }
        for i in &self.population[index].gene {
            println!("{}", i);
        }
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
