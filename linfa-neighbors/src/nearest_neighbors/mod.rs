use ndarray::prelude::*;

mod algorithm;
mod hyperparameters;

pub use algorithm::*;
pub use hyperparameters::*;

pub enum AlgorithmType {
    BallTree,
    Brute,
    KDTree,
}

pub struct NearestNeighbors<A: Algorithm> {
    algorithm: A,
}

impl <A: Algorithm> NearestNeighbors <A> {
    pub fn new(algorithm: A, hyperparameters: NearestNeighborsHyperParameters) -> NearestNeighbors<A> {
        NearestNeighbors {
            algorithm
        }
    }

    pub fn fit(mut self, input: Array2<f64>) -> Self {
        self.algorithm.fit(input);
        self
    }

    pub fn kneighbors(&self, x: &Array2<f64>) -> Vec<f64> {
        self.algorithm.query(x)
    }
}
