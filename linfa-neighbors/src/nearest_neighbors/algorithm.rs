use ndarray::prelude::*;
use ndarray::{Data, RemoveAxis};

pub trait Algorithm<D: Dimension + RemoveAxis> {
    fn fit<I: Data<Elem = f64>>(x: ArrayBase<I,D>) -> Self;

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

impl<D: Dimension + RemoveAxis> Algorithm <D> for Brute<D> {
    fn fit<I: Data<Elem = f64>>(x: ArrayBase<I, D>) -> Self {
        Brute {
            data: x.into_owned()
        }
    }

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

#[cfg(test)]
mod tests {
    use crate::nearest_neighbors::*;

    #[test]
    fn brute_force() {
        let other_test = array![[1., 2.], [3. , 4.], [5., 6.]];
        let sample_input = array![[1., 1.], [5., 6.]];

        let algo = Brute::new(other_test);
        let distances = algo.query(&sample_input);
        assert_eq!(distances, [1.0, 3.605551275463989, 6.4031242374328485, 5.656854249492381, 2.8284271247461903, 0.0]);
    }

    //    #[test]
//    fn ball_tree() {
//        let other_test = array![[1., 2.], [3. , 4.], [5., 6.]];
//
//        let parameters = NearestNeighborsHyperParameters::new().build();
//        let nbr = NearestNeighbors::new(BallTree::default(),parameters).fit(other_test);
//
//        let sample_input = array![[1., 1.], [5., 6.]];
//        let distances = nbr.kneighbors(sample_input.into_dyn());
//        println!("Distances: {:?}", distances);
//
//        assert_eq!(2 + 2, 4);
//    }
}