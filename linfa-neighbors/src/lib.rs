mod nearest_neighbors;

pub use nearest_neighbors::*;


#[cfg(test)]
mod tests {
    use crate::nearest_neighbors::*;
    use ndarray::prelude::*;

    #[test]
    fn brute_force() {
//        let parameters = NearestNeighborsHyperParameters::new().build();
        let other_test = array![[1., 2.], [3. , 4.], [5., 6.]];
//        let nbr = NearestNeighbors::new(Brute::<f64>::default(),parameters).fit(other_test);

        let sample_input = array![[1., 1.], [5., 6.]];
//        let distances = nbr.kneighbors(sample_input);
//        println!("Distances: {:?}", distances);
        let algo = Brute::new(other_test);
        let result = algo.query(&sample_input);
        assert_eq!(vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0], result);
//        assert_eq!(distances, [1.0, 3.605551275463989, 6.4031242374328485, 5.656854249492381, 2.8284271247461903, 0.0]);
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

