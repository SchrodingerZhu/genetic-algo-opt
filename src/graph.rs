use anyhow::*;
use std::path::Path;

#[derive(serde::Deserialize)]
pub struct Graph {
    pub edges: Vec<(usize, usize, usize)>,
    pub sizes: Vec<usize>,
}

pub fn load_raw_data<S: AsRef<Path>>(path: S) -> Result<Graph> {
    let reader = std::fs::File::open(path.as_ref())?;
    simd_json::from_reader(reader).map_err(Into::into)
}
