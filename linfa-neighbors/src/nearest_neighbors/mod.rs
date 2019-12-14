use ndarray::prelude::*;

mod algorithm;
mod hyperparameters;

pub use algorithm::*;
pub use hyperparameters::*;
use std::ops::Index;

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

    pub fn kneighbors(&self, x: &Array2<f64>) -> Vec<f64> {
        x.outer_iter()
            .flat_map(|elem| {
                self.data.outer_iter()
                    .map(move |inner_elem| {
                        calculate_euclidan(&elem, &inner_elem)
                    })
            }).collect()
    }
}

fn calculate_euclidan(point1: &ArrayView1<f64>, point2: &ArrayView1<f64>) -> f64 {
    let x_diff = (point2.index(0) - point1.index(0)).powi(2);
    let y_diff = (point2.index(1) - point1.index(1)).powi(2);
    (x_diff + y_diff).sqrt()
}