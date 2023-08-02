use rand::{Rng, RngCore};

use crate::chromosome::Chromosome;

use super::CrossoverMethod;

#[derive(Clone, Debug, Default)]
pub struct UniformCrossover;

impl UniformCrossover {
    pub fn new() -> Self {
        Self
    }
}

impl CrossoverMethod for UniformCrossover {
    fn crossover(
        &self,
        father: &Chromosome,
        mother: &Chromosome,
        rng: &mut dyn RngCore,
    ) -> Chromosome {
        father
            .iter()
            .zip(mother.iter())
            .map(|(gene, mother_gene)| {
                if rng.gen_bool(0.5) {
                    *gene
                } else {
                    *mother_gene
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use crate::{chromosome::Chromosome, crossover::CrossoverMethod};

    #[test]
    fn test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let father: Chromosome = (1..=100).map(|n| n as f32).collect();
        let mother: Chromosome = (1..=100).map(|n| -n as f32).collect();

        let child = super::UniformCrossover.crossover(&father, &mother, &mut rng);
        assert_eq!(child.len(), 100);
        assert!(child.iter().any(|gene| *gene > 0.0));
        assert!(child.iter().any(|gene| *gene < 0.0));

        let different_father = child
            .iter()
            .zip(father.iter())
            .filter(|(a, b)| a != b)
            .count();
        let different_mother = child
            .iter()
            .zip(mother.iter())
            .filter(|(a, b)| a != b)
            .count();
        assert_eq!(different_father, 49);
        assert_eq!(different_mother, 51);
    }
}
