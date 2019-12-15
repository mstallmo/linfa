use ndarray::prelude::*;
use std::ops::Index;

pub trait Algorithm {
    fn fit(&mut self, data: Array2<f64>);
    fn query(&self, x: &Array2<f64>) -> Vec<f64>;
}

#[derive(Default)]
pub struct BallTree {
    data: Array2<f64>
}

impl Algorithm for BallTree {
    fn fit(&mut self, data: Array2<f64>) {
        self.data = data;
    }

    fn query(&self, x: &Array2<f64>) -> Vec<f64> {
        vec![0.0, 0.0, 0.0]
    }
}

#[derive(Default)]
pub struct KDTree;

#[derive(Default)]
pub struct Brute {
    data: Array2<f64>
}

impl Algorithm for Brute {
    fn fit(&mut self, data: Array2<f64>) {
        self.data = data;
    }

    fn query(&self, x: &Array2<f64>) -> Vec<f64> {
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