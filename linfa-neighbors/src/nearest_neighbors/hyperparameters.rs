pub struct NearestNeighborsHyperParameters {
    algorithm: crate::AlgorithmType,
}

pub struct NearestNeighborsHyperParametersBuilder {
    algorithm: crate::AlgorithmType,
}

impl NearestNeighborsHyperParametersBuilder {
   pub fn algorithm(mut self, algorithm: crate::AlgorithmType) -> Self {
       self.algorithm = algorithm;
       self
   }

    pub fn build(self) -> NearestNeighborsHyperParameters {
        NearestNeighborsHyperParameters::build(self.algorithm)
    }
}

impl NearestNeighborsHyperParameters {
    pub fn new() -> NearestNeighborsHyperParametersBuilder {
        NearestNeighborsHyperParametersBuilder {
            algorithm: crate::AlgorithmType::BallTree,
        }
    }

    pub fn algorithm(self) -> crate::AlgorithmType {
        self.algorithm
    }

    pub fn build(algorithm: crate::AlgorithmType) -> NearestNeighborsHyperParameters {
        NearestNeighborsHyperParameters {
            algorithm
        }
    }
}

