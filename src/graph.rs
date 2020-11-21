use anyhow::*;
use std::path::Path;
use hashbrown::{HashMap, HashSet};

pub struct Graph {
    pub(crate) sizes: Vec<usize>,
    pub(crate) edges: hashbrown::HashMap<(usize, usize), usize>,
}

#[derive(serde::Deserialize)]
pub struct RawData {
    edges: Vec<(usize, usize, usize)>,
    sizes: Vec<usize>,
}

pub fn load_raw_data<S: AsRef<Path>>(path: S) -> Result<RawData> {
    let reader = std::fs::File::open(path.as_ref())?;
    simd_json::from_reader(reader)
        .map_err(Into::into)
}

impl Graph {
    pub fn from_raw_data(raw: RawData) -> Self {
        let mut edges = hashbrown::HashMap::new();
        raw.edges.into_iter()
            .for_each(|(a, b, w)| { edges.insert((a, b), w); });
        Graph {
            edges,
            sizes: raw.sizes,
        }
    }
}