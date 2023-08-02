use lib_genetic_algorithm::chromosome::Chromosome;
use lib_neural_network::{LayerTopology, Network};
use rand::RngCore;

use super::eyes::Eyes;

pub struct Brain {
    pub(crate) nn: Network,
}

impl Brain {
    pub fn random(_rng: &mut dyn RngCore, eye: &Eyes) -> Self {
        Self {
            nn: Network::random(&Self::topology(eye)),
        }
    }

    pub(crate) fn as_chromosome(&self) -> Chromosome {
        self.nn.weights().collect()
    }

    fn topology(eye: &Eyes) -> [LayerTopology; 3] {
        [
            LayerTopology {
                neurons: eye.cells(),
            },
            LayerTopology {
                neurons: 2 * eye.cells(),
            },
            LayerTopology { neurons: 2 },
        ]
    }

    pub(crate) fn from_chromosome(chromosome: Chromosome, eye: &Eyes) -> Self {
        Self {
            nn: Network::from_weights(&Self::topology(eye), chromosome),
        }
    }
}
