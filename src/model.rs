use rand::Rng;
use crate::*;
#[derive(Clone, Debug)]
pub struct Instance {
    pub(crate) gene: Vec<usize>
}

impl Instance {
    pub fn mutate(&self) -> Self {
        // use bernoulli experiment
        let mut new = self.clone();
        let mut gen = rand::rngs::ThreadRng::default();
        while gen.gen::<f64>() < SINGLE_MUTATION_POSSIBILITY {
            // select two different pairs from the gnome
            let mut x1: usize = gen.gen::<usize>() % self.gene.len();
            let mut y1: usize = gen.gen::<usize>() % self.gene.len();
            if x1 > y1 {
                std::mem::swap(&mut x1, &mut y1);
            }

            let mut x2: usize = gen.gen::<usize>() % self.gene.len();
            let mut y2: usize = gen.gen::<usize>() % self.gene.len();
            if x2 > y2 {
                std::mem::swap(&mut x2, &mut y2);
            }

            if x1 == y1 || x1 == x2
                || x1 == y2 || y1 == x2
                || y1 == y2 || x2 == y2 { continue; }

            // now we exchange x1 <-> x2, y1 <-> y2
            new.gene.iter_mut().for_each(|x| {
                if *x == x1 {
                    *x = x2;
                } else if *x == x2 {
                    *x = x1;
                } else if *x == y1 {
                    *x = y2;
                } else if *x == y2 {
                    *x = y1;
                }
            });
        }
        new
    }

    pub fn crossover(a: &Instance, b: &Instance) -> Self {
        // 0 4 1 5 6 3 2
        // 5 6 0 4 1 2 3
        let mut gene = Vec::new();
        let mut existed = hashbrown::HashSet::new();
        let mut arr = [a, b];
        let mut gen = rand::thread_rng();
        let mut i = 0;
        while gene.len() < a.gene.len() {
            if gen.gen::<f64>() < 0.5 {
                let x = arr[0];
                let y = arr[1];
                arr[0] = y;
                arr[1] = x;
            }
            if !existed.contains(&arr[0].gene[i]) {
                gene.push(arr[0].gene[i]);
                existed.insert(arr[0].gene[i]);
            }
            if !existed.contains(&arr[1].gene[i]) {
                gene.push(arr[1].gene[i]);
                existed.insert(arr[1].gene[i]);
            }
            i += 1;
        }
        Instance {
            gene
        }
    }

    pub fn fitness(&self, graph: &crate::graph::Graph) -> f64 {
        //final fitness: scale factor / penalty
        unimplemented!()
    }
}