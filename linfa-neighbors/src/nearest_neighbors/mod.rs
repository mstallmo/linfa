use ndarray::prelude::*;
use ndarray::{Data, RemoveAxis};

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

impl<A: Algorithm> NearestNeighbors<A> {
    pub fn new(algorithm: A, _hyperparameters: NearestNeighborsHyperParameters) -> NearestNeighbors<A> {
        NearestNeighbors {
            algorithm
        }
    }

    pub fn fit<I: Data<Elem=f64>, D: Dimension + RemoveAxis>(mut self, input: ArrayBase<I, D>) -> Self {
//        self.algorithm.fit(input);
        self
    }

    pub fn kneighbors<I, D>(&self, x: ArrayBase<I, D>) -> Vec<f64>
        where
            I: Data<Elem=f64>,
            D: Dimension + RemoveAxis
    {
        self.algorithm.query(&x)
    }
}
