mod algorithm;

pub use algorithm::*;

pub enum AlgorithmType {
    Auto,
    BallTree,
    Brute,
    KDTree,
}

pub struct NearestNeighbors;

impl NearestNeighbors {
    pub fn fit(algorithm: AlgorithmType) {
        match algorithm {
            AlgorithmType::BallTree => {
                BallTree::fit();
            }
            AlgorithmType::Brute => {
                unimplemented!();
            }
            AlgorithmType::KDTree => {
                KDTree::fit();
            }
            AlgorithmType::Auto => {
                BallTree::fit();
            }
        }
    }
}
