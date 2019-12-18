use ndarray::prelude::*;
use ndarray::{Data, RemoveAxis};

mod algorithm;
mod hyperparameters;

pub use algorithm::*;
pub use hyperparameters::*;
use std::marker::PhantomData;

pub enum AlgorithmType {
    BallTree,
    Brute,
    KDTree,
}

pub struct NearestNeighbors<A: Algorithm<Dim>, Dim: Dimension + RemoveAxis> {
    algorithm: A,
    dimension: std::marker::PhantomData<Dim>,
}

impl<A, Dim> NearestNeighbors<A, Dim>
    where
        Dim: Dimension + RemoveAxis,
        A: Algorithm<Dim>,
{
    pub fn new(algorithm: A, _hyperparameters: NearestNeighborsHyperParameters) -> NearestNeighbors<A, Dim> {
        NearestNeighbors {
            algorithm,
            dimension: PhantomData,
        }
    }

    pub fn fit<I: Data<Elem=f64>, D: Dimension + RemoveAxis>(mut self, input: ArrayBase<I, D>) -> Self
        where
            A: Algorithm<D>
    {
        self.algorithm = Algorithm::fit(input);
        self
    }

    pub fn kneighbors<I>(&self, x: ArrayBase<I, Dim>) -> Vec<f64>
        where
            I: Data<Elem=f64>,
    {
        self.algorithm.query(&x)
    }
}


#[cfg(test)]
mod tests {
    use crate::nearest_neighbors::*;

    #[test]
    fn nearest_neighbor_brute_force() {
        let other_test = array![[1., 2.], [3. , 4.], [5., 6.]];
        let sample_input = array![[1., 1.], [5., 6.]];

        let parameters = NearestNeighborsHyperParameters::new().build();
        let nbr = NearestNeighbors::new(Brute::<Ix2>::default(),parameters).fit(other_test);
        let distances = nbr.kneighbors(sample_input);
        assert_eq!(distances, [1.0, 3.605551275463989, 6.4031242374328485, 5.656854249492381, 2.8284271247461903, 0.0]);
    }
}
