use rand::{Rng, RngCore};

use crate::chromosome::Chromosome;

use super::MutationMethod;

pub struct GaussianMutation {
    chance: f32,
    coeff: f32,
}

impl GaussianMutation {
    pub fn new(chance: f32, coeff: f32) -> Self {
        assert!(chance >= 0.0);
        Self { chance, coeff }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(&self, chromosome: &mut Chromosome, rng: &mut dyn RngCore) {
        chromosome.iter_mut().for_each(|gene| {
            let sign = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };
            if rng.gen_bool(self.chance as f64) {
                *gene += sign * rng.gen::<f32>() * self.coeff;
            }
        });
    }
}

#[cfg(test)]
mod tests {
    mod given_zero_chance {
        mod and_zero_coeff {
            use rand::SeedableRng;
            use rand_chacha::ChaCha8Rng;

            use crate::{
                chromosome::Chromosome,
                mutation::{gaussian::GaussianMutation, MutationMethod},
            };

            #[test]
            fn does_not_change_the_original_chromosome() {
                let mut rng = ChaCha8Rng::from_seed(Default::default());
                let mut chromosome = Chromosome::new(vec![0.0, 1.0, 2.0, 3.0]);
                let original = chromosome.clone();

                let mutation = GaussianMutation::new(0.0, 0.0);
                mutation.mutate(&mut chromosome, &mut rng);

                assert_eq!(chromosome, original);
            }
        }

        mod and_nonzero_coeff {

            use rand::SeedableRng;
            use rand_chacha::ChaCha8Rng;

            use crate::{
                chromosome::Chromosome,
                mutation::{gaussian::GaussianMutation, MutationMethod},
            };

            #[test]
            fn does_not_change_the_original_chromosome() {
                let mut rng = ChaCha8Rng::from_seed(Default::default());
                let mut chromosome = Chromosome::new(vec![0.0, 1.0, 2.0, 3.0]);
                let original = chromosome.clone();

                let mutation = GaussianMutation::new(0.0, 0.9);
                mutation.mutate(&mut chromosome, &mut rng);

                assert_eq!(chromosome, original);
            }
        }
    }

    mod given_fifty_fifty_chance {
        mod and_zero_coeff {
            use crate::chromosome::Chromosome;

            #[test]
            fn does_not_change_the_original_chromosome() {
                use rand::SeedableRng;
                use rand_chacha::ChaCha8Rng;

                use crate::mutation::{gaussian::GaussianMutation, MutationMethod};

                let mut rng = ChaCha8Rng::from_seed(Default::default());
                let mut chromosome = Chromosome::new(vec![0.0, 1.0, 2.0, 3.0]);
                let original = chromosome.clone();

                let mutation = GaussianMutation::new(0.5, 0.0);
                mutation.mutate(&mut chromosome, &mut rng);

                assert_eq!(chromosome, original);
            }
        }

        mod and_nonzero_coeff {
            use crate::chromosome::Chromosome;

            #[test]
            fn slightly_changes_the_original_chromosome() {
                use rand::SeedableRng;
                use rand_chacha::ChaCha8Rng;

                use crate::mutation::{gaussian::GaussianMutation, MutationMethod};

                let mut rng = ChaCha8Rng::from_seed(Default::default());
                let mut chromosome = Chromosome::new(vec![0.0, 1.0, 2.0, 3.0]);
                let original = chromosome.clone();

                let mutation = GaussianMutation::new(0.5, 0.9);
                mutation.mutate(&mut chromosome, &mut rng);

                assert_ne!(chromosome, original);
            }
        }
    }

    mod given_max_chance {
        mod and_zero_coeff {
            use crate::chromosome::Chromosome;

            #[test]
            fn does_not_change_the_original_chromosome() {
                use rand::SeedableRng;
                use rand_chacha::ChaCha8Rng;

                use crate::mutation::{gaussian::GaussianMutation, MutationMethod};

                let mut rng = ChaCha8Rng::from_seed(Default::default());
                let mut chromosome = Chromosome::new(vec![0.0, 1.0, 2.0, 3.0]);
                let original = chromosome.clone();

                let mutation = GaussianMutation::new(1.0, 0.0);
                mutation.mutate(&mut chromosome, &mut rng);

                assert_eq!(chromosome, original);
            }
        }

        mod and_nonzero_coeff {
            use crate::chromosome::Chromosome;

            #[test]
            fn entirely_changes_the_original_chromosome() {
                use rand::SeedableRng;
                use rand_chacha::ChaCha8Rng;

                use crate::mutation::{gaussian::GaussianMutation, MutationMethod};

                let mut rng = ChaCha8Rng::from_seed(Default::default());
                let mut chromosome = Chromosome::new(vec![0.0, 1.0, 2.0, 3.0]);
                let original = chromosome.clone();

                let mutation = GaussianMutation::new(1.0, 0.9);
                mutation.mutate(&mut chromosome, &mut rng);

                assert_ne!(chromosome, original);
            }
        }
    }
}
