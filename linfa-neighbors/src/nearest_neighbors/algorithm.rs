use ndarray::prelude::*;

pub trait Tree {
    fn fit(data: Array2::<f64>) -> Array2::<f64>;
}

pub struct BallTree;

impl Tree for BallTree {
    fn fit(data: Array2::<f64>) -> Array2::<f64> {
        println!("You called fit on a Ball Tree!");
        data
    }
}

pub struct KDTree;

impl Tree for KDTree {
    fn fit(data: Array2::<f64>) -> Array2::<f64> {
        println!("You called fit on a KD Tree!");
        data
    }
}