pub struct NearestNeighborsHyperParameters {
    n_neighbors: usize,
}

pub struct NearestNeighborsHyperParametersBuilder {
    n_neighbors: usize,
}

impl NearestNeighborsHyperParametersBuilder {
   pub fn n_neighbors(mut self, n_neighbors: usize) -> Self {
       self.n_neighbors = n_neighbors;
       self
   }

    pub fn build(self) -> NearestNeighborsHyperParameters {
        NearestNeighborsHyperParameters::build(self.n_neighbors)
    }
}

impl NearestNeighborsHyperParameters {
    pub fn new() -> NearestNeighborsHyperParametersBuilder {
        NearestNeighborsHyperParametersBuilder {
            n_neighbors: 5
        }
    }

    pub fn n_neighbors(self) -> usize {
        self.n_neighbors
    }

    pub fn build(n_neighbors: usize) -> NearestNeighborsHyperParameters {
        NearestNeighborsHyperParameters {
            n_neighbors
        }
    }
}

