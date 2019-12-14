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
    data: Array2<f64>,
}

impl NearestNeighbors {
    pub fn fit(hyperparameters: NearestNeighborsHyperParameters, input: Array2<f64>) -> NearestNeighbors {
      let data = match hyperparameters.algorithm() {
            AlgorithmType::BallTree => {
                BallTree::fit(input)
            }
            AlgorithmType::Brute => {
                input
            }
            AlgorithmType::KDTree => {
                KDTree::fit(input)
            }
        };

        NearestNeighbors {
            data
        }
    }
}
