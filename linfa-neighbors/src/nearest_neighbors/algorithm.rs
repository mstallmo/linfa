use ndarray::prelude::*;
use ndarray::{Data, RemoveAxis};

pub trait Algorithm {
    fn query<I: Data<Elem=f64>, Dim: Dimension + RemoveAxis>(&self, x: &ArrayBase<I, Dim>) -> Vec<f64>;
}

#[derive(Default)]
pub struct Brute<D: Dimension + RemoveAxis> {
    data: Array<f64, D>
}

impl<D: Dimension + RemoveAxis> Brute<D> {
    pub fn new(data: Array<f64, D>) -> Self {
        Brute {
            data
        }
    }
}

impl<D: Dimension + RemoveAxis> Algorithm for Brute<D> {
    fn query<I, Dim>(&self, x: &ArrayBase<I, Dim>) -> Vec<f64>
        where
            I: Data<Elem=f64>,
            Dim: Dimension + RemoveAxis
    {
        x.outer_iter()
            .flat_map(|elem| {
                self.data.outer_iter()
                    .map(move |inner_elem| {
                        calculate_euclidan(&elem, &inner_elem)
                    })
            }).collect()
    }
}

fn calculate_euclidan<A, D, E>(point1: &ArrayBase<A, D>, point2: &ArrayBase<A, E>) -> f64
    where
        A: Data<Elem=f64>,
        D: Dimension,
        E: Dimension,
{
    point2.iter()
        .zip(point1.iter())
        .map(|(&elem2, &elem1)| {
            (elem2 - elem1).powi(2)
        }).sum::<f64>().sqrt()
}