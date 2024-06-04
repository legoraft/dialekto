use rand::prelude::*;

pub fn generate_indices(length: usize) -> Vec<usize> {
    let indices: Vec<usize> = (0..length).collect();
    let indices: Vec<usize> = indices.choose_multiple(&mut thread_rng(), length).cloned().collect();

    indices
}