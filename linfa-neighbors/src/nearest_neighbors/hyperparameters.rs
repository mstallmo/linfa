#[derive(Copy, Clone)]
pub struct NearestNeighborsHyperParameters {
    n_neighbors: usize,
    leaf_size: usize,
}

pub struct NearestNeighborsHyperParametersBuilder {
    n_neighbors: usize,
    leaf_size: usize,
}

impl NearestNeighborsHyperParametersBuilder {
    pub fn n_neighbors(mut self, n_neighbors: usize) -> Self {
        self.n_neighbors = n_neighbors;
        self
    }

    pub fn leaf_size(mut self, leaf_size: usize) -> Self {
        self.leaf_size = leaf_size;
        self
    }

    pub fn build(self) -> NearestNeighborsHyperParameters {
        NearestNeighborsHyperParameters::build(self.n_neighbors, self.leaf_size)
    }
}

impl NearestNeighborsHyperParameters {
    pub fn new() -> NearestNeighborsHyperParametersBuilder {
        NearestNeighborsHyperParametersBuilder {
            n_neighbors: 5,
            leaf_size: 30,
        }
    }

    pub fn n_neighbors(self) -> usize {
        self.n_neighbors
    }

    pub fn leaf_size(self) -> usize {
        self.leaf_size
    }

    pub fn build(n_neighbors: usize, leaf_size: usize) -> NearestNeighborsHyperParameters {
        NearestNeighborsHyperParameters {
            n_neighbors,
            leaf_size,
        }
    }
}

