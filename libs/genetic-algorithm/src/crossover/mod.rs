use rand::RngCore;

use crate::chromosome::Chromosome;

pub mod uniform;

pub trait CrossoverMethod {
    fn crossover(
        &self,
        father: &Chromosome,
        mother: &Chromosome,
        rng: &mut dyn RngCore,
    ) -> Chromosome;
}
