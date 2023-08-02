use rand::{seq::SliceRandom, RngCore};

use crate::individual::Individual;

use super::SelectionMethod;

#[derive(Default)]
pub struct RouletteWheelSelection;

impl RouletteWheelSelection {
    pub fn new() -> Self {
        Self
    }
}

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, population: &'a [I], rng: &mut dyn RngCore) -> &'a I
    where
        I: Individual,
    {
        population.choose_weighted(rng, |i| i.fitness()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use crate::{
        individual::Individual,
        select::{roulette_wheel::RouletteWheelSelection, SelectionMethod},
        tests::TestIndividual,
    };

    #[test]
    fn test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let polulation = vec![
            TestIndividual::new(0.0),
            TestIndividual::new(1.0),
            TestIndividual::new(2.0),
            TestIndividual::new(3.0),
        ];

        let actual_histogram = (0..1000)
            .map(|_| {
                RouletteWheelSelection::new()
                    .select(&polulation, &mut rng)
                    .fitness() as i32
            })
            .fold(BTreeMap::new(), |mut acc, x| {
                *acc.entry(x).or_insert(0) += 1;
                acc
            });

        let expected_histogram = vec![(1, 164), (2, 337), (3, 499)]
            .into_iter()
            .collect::<BTreeMap<_, _>>();

        assert_eq!(actual_histogram, expected_histogram);
    }
}
