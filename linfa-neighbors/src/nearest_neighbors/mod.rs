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

pub struct NearestNeighbors {
    algorithm: Box<dyn Algorithm>,
}

impl NearestNeighbors {
    pub fn new(hyperparameters: NearestNeighborsHyperParameters) -> Self {
        match hyperparameters.algorithm() {
            AlgorithmType::Brute => {
                NearestNeighbors {
                    algorithm: Box::new(Brute::default())
                }
            }
            AlgorithmType::BallTree => {
                NearestNeighbors {
                    algorithm: Box::new(BallTree::default())
                }
            }
            _ => {
                unimplemented!();
            }
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
