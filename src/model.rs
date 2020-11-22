use rand::Rng;
use rand::distributions::Distribution;
use rand_distr::num_traits::abs;

#[derive(Clone, Debug)]
pub struct Instance {
    pub(crate) gene: Vec<usize>,
}

fn generate_meta(
    gene: &[usize],
    sizes: &[usize],
) ->
    Vec<usize>
{
    let mut ret1 = Vec::with_capacity(gene.len());
    let mut curr_byte = 0;
    ret1.resize(gene.len(), 0);
    for i in gene {
        ret1[*i] = curr_byte;
        curr_byte += sizes[*i];
    }
    ret1
}

impl Instance {
    pub fn mutate(&mut self, possibility: f64) {
        // use bernoulli experiment
        let mut gen = rand::rngs::ThreadRng::default();
        let mut counter = 0usize;
        while gen.gen::<f64>() < possibility {
            counter += 1;
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

            if x1 == y1 || x1 == x2 || x1 == y2 || y1 == x2 || y1 == y2 || x2 == y2 {
                continue;
            }

            // now we exchange x1 <-> x2, y1 <-> y2
            self.gene.iter_mut().for_each(|x| {
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
        log::trace!("mutation happened with {} changes", counter);
    }

    pub fn crossover(a: &Instance, b: &Instance) -> Self {
        log::trace!("crossover happened");
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
        Instance { gene }
    }

    pub fn fitness(&self, graph: &crate::graph::Graph, page_size: usize, cache_size: usize, page_fault: f64, cache_miss: f64, distance: f64, order: f64, scale_factor: f64) -> f64 {
        //final fitness: scale factor / penalty
        let mut penalty = 0f64;

        let locations = generate_meta(&self.gene, &graph.sizes);

        for (f, t, freq) in &graph.edges {
            if locations[*f] / page_size != locations[*t] / page_size {
                penalty += *freq as f64 * page_fault;
            }
            if locations[*f] / cache_size != locations[*t] / cache_size {
                penalty += *freq as f64 * cache_miss;
            }
            if locations[*f] > locations[*t] {
                penalty += *freq as f64 * order;
            }
            penalty += distance * (*freq as f64) * abs(locations[*f] as f64 - locations[*t] as f64);
        }
        scale_factor / penalty
    }

    pub fn mate(a: &Self, b: &Self, cross_over: f64, mutation: f64, single_mutation: f64) -> Self {
        let mut rng = rand::thread_rng();
        let dist = rand::distributions::Uniform::new(0.0, 1.0);
        let rand = dist.sample(&mut rng);
        let mut target = if rand <= (1.0 - cross_over) / 2.0 {
            a.clone()
        } else if rand <= (1.0 - cross_over) {
            b.clone()
        } else {
            Instance::crossover(a, b)
        };
        if rng.gen::<f64>() < mutation {
            target.mutate(single_mutation);
        }
        target
    }
}
