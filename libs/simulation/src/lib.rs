use lib_genetic_algorithm::{
    crossover::uniform::UniformCrossover, mutation::gaussian::GaussianMutation,
    select::roulette_wheel::RouletteWheelSelection, GeneticAlgorithm,
};
use nalgebra::{distance, wrap, Rotation2, Vector2};
use rand::{Rng, RngCore};
use world::individual::AnimalIndividual;
pub use world::World;

pub mod world;

use std::f32::consts::FRAC_PI_2;

const SPEED_MIN: f32 = 0.0006;
const SPEED_MAX: f32 = 0.0012;
const SPEED_ACCEL: f32 = 0.2;
const ROTATION_ACCEL: f32 = FRAC_PI_2;
const GENERATION_LENGTH: usize = 2500;

pub struct Simulation {
    pub world: World,
    ga: GeneticAlgorithm<RouletteWheelSelection>,
    age: usize,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let world = World::random(rng);

        let ga = GeneticAlgorithm::new(
            RouletteWheelSelection::new(),
            UniformCrossover::new(),
            GaussianMutation::new(0.01, 0.3),
            // ---------------------- ^--^ -^-^
            // | Chosen with a bit of experimentation.
            // |
            // | Higher values can make the simulation more chaotic,
            // | which - a bit counterintuitively - might allow for
            // | it to discover *better* solutions; but the trade-off
            // | is that higher values might also cause current, good
            // | enough solutions to be discarded.
            // ---
        );
        Self { world, ga, age: 0 }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_collisions(rng);
        self.process_brains();
        self.process_movements();

        self.age += 1;

        if self.age > GENERATION_LENGTH {
            self.evolve(rng);
        }
    }

    fn process_movements(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation * Vector2::new(0.0, animal.speed);
            animal.position.x = wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = wrap(animal.position.y, 0.0, 1.0);
        }
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = distance(&animal.position, &food.position);

                if distance <= 0.01 {
                    animal.satiation += 1;
                    food.position = rng.gen();
                }
            }
        }
    }

    fn process_brains(&mut self) {
        for animal in &mut self.world.animals {
            let vision =
                animal
                    .eyes
                    .process_vision(animal.position, animal.rotation, &self.world.foods);

            let response = animal.brain.nn.propagate(vision);

            // ---
            // | Limits number to given range.
            // -------------------- v---v
            let speed = response[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);

            let rotation = response[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);

            // Our speed & rotation here are *relative* - that is: when
            // they are equal to zero, what the brain says is "keep
            // flying as you are now", not "stop flying".
            //
            // Both values being relative is crucial, because our bird's
            // brain doesn't know its own speed and rotation*, meaning
            // that it fundamentally cannot return absolute values.
            //
            // * they'd have to be provided as separate inputs to the
            //   neural network, which would make the evolution process
            //   waaay longer, if even possible.

            animal.speed = (animal.speed + speed).clamp(SPEED_MIN, SPEED_MAX);

            animal.rotation = Rotation2::new(animal.rotation.angle() + rotation);

            // (btw, there is no need for ROTATION_MIN or ROTATION_MAX,
            // because rotation automatically wraps from 2*PI back to 0 -
            // we've already witnessed that when we were testing eyes,
            // inside `mod different_rotations { ... }`.)
        }
    }

    fn evolve(&mut self, rng: &mut dyn RngCore) {
        self.age = 0;

        // Transforms `Vec<Animal>` to `Vec<AnimalIndividual>`
        let current_population: Vec<_> = self
            .world
            .animals
            .iter()
            .map(AnimalIndividual::from_animal)
            .collect();

        // Evolves this `Vec<AnimalIndividual>`
        let evolved_population = self.ga.evolve(&current_population, rng);

        // Transforms `Vec<AnimalIndividual>` back into `Vec<Animal>`
        self.world.animals = evolved_population
            .into_iter()
            .map(|individual| individual.into_animal(rng))
            .collect();

        for food in &mut self.world.foods {
            food.position = rng.gen();
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use super::*;

    #[test]
    fn random_simulation() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let sim = Simulation::random(&mut rng);

        assert_eq!(sim.world().animals().len(), 40);
        assert_eq!(sim.world().food().len(), 60);
    }
}
