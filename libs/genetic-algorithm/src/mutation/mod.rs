use rand::RngCore;

use crate::chromosome::Chromosome;

pub mod gaussian;

pub trait MutationMethod {
    fn mutate(&self, chromosome: &mut Chromosome, rng: &mut dyn RngCore);
}
