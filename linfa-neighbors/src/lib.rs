mod nearest_neighbors;

pub use nearest_neighbors::*;

#[test]
fn it_works() {
    NearestNeighbors::fit(Algorithm::Auto);
    assert_eq!(2 + 2, 4);
}
