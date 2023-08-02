use rand::RngCore;

pub use self::{animal::Animal, food::Food};

pub mod animal;
pub mod food;
pub mod individual;

pub struct World {
    pub animals: Vec<Animal>,
    pub foods: Vec<Food>,
}

impl World {
    pub(crate) fn random(rng: &mut dyn RngCore) -> World {
        let animals = (0..40).map(|_| Animal::random(rng)).collect();

        let foods = (0..60).map(|_| Food::random(rng)).collect();

        World { animals, foods }
    }

    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn food(&self) -> &[Food] {
        &self.foods
    }
}
