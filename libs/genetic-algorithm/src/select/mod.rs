use rand::RngCore;

use crate::individual::Individual;

pub mod roulette_wheel;

pub trait SelectionMethod {
    fn select<'a, I>(&self, population: &'a [I], rng: &mut dyn RngCore) -> &'a I
    where
        I: Individual;
}
