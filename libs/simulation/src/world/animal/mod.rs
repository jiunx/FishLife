use lib_genetic_algorithm::chromosome::Chromosome;
use nalgebra::{Point2, Rotation2};
use rand::{distributions::Uniform, prelude::Distribution, Rng, RngCore};

use crate::{SPEED_MAX, SPEED_MIN};

use self::{brain::Brain, eyes::Eyes};

pub mod brain;
pub mod eyes;

pub struct Animal {
    pub(crate) position: Point2<f32>,
    pub(crate) rotation: Rotation2<f32>,
    pub(crate) speed: f32,
    pub(crate) eyes: Eyes,
    pub(crate) brain: Brain,
    pub(crate) satiation: usize,
}

impl Animal {
    fn new(eyes: Eyes, brain: Brain, rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
            eyes,
            brain,
            satiation: 0,
        }
    }

    pub fn random(rng: &mut dyn RngCore) -> Animal {
        let between = Uniform::from(SPEED_MIN..=SPEED_MAX);
        let eyes = Eyes::default();
        let brain = Brain::random(rng, &eyes);

        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: between.sample(rng),
            eyes,
            brain,
            satiation: 0,
        }
    }

    pub fn position(&self) -> Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> Rotation2<f32> {
        self.rotation
    }

    pub fn speed(&self) -> f32 {
        self.speed
    }

    pub(crate) fn as_chromosome(&self) -> Chromosome {
        // We evolve only our birds' brains, but technically there's no
        // reason not to simulate e.g. physical properties such as size.
        //
        // If that was to happen, this function could be adjusted to
        // return a longer chromosome that encodes not only the brain,
        // but also, say, birdie's color.

        self.brain.as_chromosome()
    }

    pub(crate) fn from_chromosome(chromosome: Chromosome, rng: &mut dyn RngCore) -> Self {
        let eye = Eyes::default();
        let brain = Brain::from_chromosome(chromosome, &eye);

        Self::new(eye, brain, rng)
    }
}
