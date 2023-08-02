use crossover::CrossoverMethod;
use individual::Individual;
use mutation::MutationMethod;
use rand::RngCore;
use select::SelectionMethod;

pub mod chromosome;
pub mod crossover;
pub mod individual;
pub mod mutation;
pub mod select;

pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod,
{
    pub fn new(
        selection_method: S,
        crossover_method: impl CrossoverMethod + 'static,
        mutation_method: impl MutationMethod + 'static,
    ) -> Self {
        Self {
            selection_method,
            crossover_method: Box::new(crossover_method),
            mutation_method: Box::new(mutation_method),
        }
    }

    pub fn evolve<I>(&self, population: &[I], rng: &mut dyn RngCore) -> Vec<I>
    where
        I: Individual,
    {
        (0..population.len())
            .map(|_| {
                let father = self.selection_method.select(population, rng);
                let mother = self.selection_method.select(population, rng);

                let mut child =
                    self.crossover_method
                        .crossover(father.chromosome(), mother.chromosome(), rng);

                self.mutation_method.mutate(&mut child, rng);

                I::create(child)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use super::Individual;
    use crate::{
        chromosome::Chromosome,
        crossover::uniform::UniformCrossover,
        mutation::gaussian::GaussianMutation,
        select::roulette_wheel::RouletteWheelSelection,
        GeneticAlgorithm,
    };

    #[derive(Clone, Debug, PartialEq)]
    pub enum TestIndividual {
        WithChromosome(Chromosome),
        WithFitness(f32),
    }

    impl TestIndividual {
        pub fn new(fitness: f32) -> Self {
            Self::WithFitness(fitness)
        }
    }

    impl Individual for TestIndividual {
        fn fitness(&self) -> f32 {
            match self {
                Self::WithChromosome(chromosome) => chromosome.iter().sum(),
                Self::WithFitness(fitness) => *fitness,
            }
        }

        fn chromosome(&self) -> &Chromosome {
            match self {
                Self::WithChromosome(chromosome) => chromosome,
                Self::WithFitness(_) => panic!("No chromosome"),
            }
        }

        fn create(chromosome: Chromosome) -> Self {
            Self::WithChromosome(chromosome)
        }
    }

    fn individual(genes: &[f32]) -> TestIndividual {
        TestIndividual::create(genes.iter().copied().collect())
    }

    #[test]
    fn test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let ga = GeneticAlgorithm::new(
            RouletteWheelSelection::new(),
            UniformCrossover::new(),
            GaussianMutation::new(0.5, 0.5),
        );

        let mut polulation = vec![
            individual(&[0.0, 0.0, 0.0]),
            individual(&[1.0, 1.0, 1.0]),
            individual(&[1.0, 2.0, 1.0]),
            individual(&[1.0, 2.0, 4.0]),
        ];

        for _ in 0..10 {
            polulation = ga.evolve(&polulation, &mut rng);
        }

        let expected_population = vec![
            individual(&[1.606008, 2.789879, 3.6941864]),
            individual(&[1.0839049, 2.4461222, -0.8869108]),
            individual(&[0.99193525, 2.588976, 3.5712361]),
            individual(&[1.646358, 2.392836, 3.9752667]),
        ];

        assert_eq!(polulation, expected_population);
    }
}
