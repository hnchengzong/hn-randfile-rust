use rand::Rng;
use rand_distr::{Alphanumeric, Distribution};

pub fn random_name(rng: &mut impl Rng, len: usize, ext: Option<&str>) -> String {
    let name: String = Alphanumeric
        .sample_iter(rng)
        .take(len)
        .map(char::from)
        .collect();

    match ext {
        Some(e) => format!("{}.{}", name, e),
        None => name,
    }
}

pub fn random_content(rng: &mut impl Rng, size: usize) -> Vec<u8> {
    Alphanumeric
        .sample_iter(rng)
        .take(size)
        .map(|c| c as u8)
        .collect()
}