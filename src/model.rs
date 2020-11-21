use crate::*;
use rand::Rng;
#[derive(Clone, Debug)]
pub struct Instance {
    pub(crate) gene: Vec<usize>,
}

fn generate_meta(
    gene: &[usize],
    sizes: &[usize],
    pgsize: usize,
) -> (
    hashbrown::HashMap<usize, usize>,
    hashbrown::HashMap<usize, usize>,
) {
    let mut ret1 = hashbrown::HashMap::<usize, usize>::new();
    let mut ret2 = hashbrown::HashMap::<usize, usize>::new();
    let mut page_id = 0;
    let mut curr_byte = 0;
    for i in gene {
        ret1.insert(*i, page_id);
        ret2.insert(*i, curr_byte);
        curr_byte += sizes[*i];
        if curr_byte > pgsize {
            page_id += 1;
            curr_byte -= pgsize;
        }
    }
    (ret1, ret2)
}

impl Instance {
    pub fn mutate(&mut self) {
        // use bernoulli experiment
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
        Instance { gene }
    }

    pub fn fitness(&self, graph: &crate::graph::Graph) -> f64 {
        //final fitness: scale factor / penalty
        let mut penalty = 0f64;

        let (id_to_page_map, id_to_relpos_map) = generate_meta(&self.gene, &graph.sizes, PAGE_SIZE);

        for ((f, t), freq) in &graph.edges {
            if id_to_page_map.get(f) != id_to_page_map.get(t) {
                penalty += *freq as f64 * PAGE_FAULT_PENALTY;
            }
            if id_to_relpos_map.get(f) != id_to_relpos_map.get(t) {
                penalty += *freq as f64 * CACHE_MISS_PENALTY;
            }
        }
        SCALE_FACTOR / penalty
    }

    pub fn mate(a: &Self, b: &Self) -> Self {
        let mut rng = rand::thread_rng();
        let rand: f64 = rng.gen();
        let mut target = if rand <= 0.45 {
            a.clone()
        } else if rand <= 0.90 {
            b.clone()
        } else {
            Instance::crossover(a, b)
        };
        if rng.gen::<f64>() < INSTANCE_MUTATION_RATE {
            target.mutate();
        }
        target
    }
}
