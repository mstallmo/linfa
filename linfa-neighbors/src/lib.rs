mod nearest_neighbors;

pub use nearest_neighbors::*;


#[cfg(test)]
mod tests {
    use crate::nearest_neighbors::*;
    use ndarray::prelude::*;
    use ndarray::{arr2, Array2, Array3, Axis};

    #[test]
    fn it_works() {
        let parameters = NearestNeighborsHyperParameters::new().build();
        let other_test = array![[1., 2.], [3. , 4.], [5., 6.]];
        let nbr = NearestNeighbors::new(Brute::default(),parameters).fit(other_test);

        let sample_input = array![[1., 1.], [5., 6.]];
        let distances = nbr.kneighbors(&sample_input);
        println!("Distances: {:?}", distances);

        assert_eq!(2 + 2, 4);
    }

//    #[test]
//    fn looping_ndarray() {
//        let a = Array3::<f32>::zeros((4, 3, 2));
//
//        println!("Array: {}", a);
//        a.outer_iter().for_each(|elem| println!("Elem: {}", elem.index_axis(Axis(0), 1)));
//        println!("Array Elem: {}", a.index_axis(Axis(0), 1));
//
//        let test = Array2::<f32>::zeros((1, 2));
//        println!("Test array: {}", test);
//
//        let other_test = array![[1., 2.], [3. , 4.], [5., 6.]];
//        println!("Other Test: {}", other_test);
//        println!("Other test shape: {:?}", other_test.shape());
//        other_test.outer_iter().for_each(|elem| println!("Other test elem: {}", elem[0]));
//
//
//        assert_eq!(true, true);
//    }
}

