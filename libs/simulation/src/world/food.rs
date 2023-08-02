use nalgebra::Point2;
use rand::{Rng, RngCore};

#[derive(Debug)]
pub struct Food {
    pub(crate) position: Point2<f32>,
    pub(crate) color: String,
}
impl Food {
    pub(crate) fn random(rng: &mut dyn RngCore) -> Food {
        // random color rgb(0, 0, 0)
        let r: u8 = rng.gen();
        let g: u8 = rng.gen();
        let b: u8 = rng.gen();
        Self {
            position: rng.gen(),
            color: format!("rgb({}, {}, {})", r, g, b),
        }
    }

    pub fn position(&self) -> Point2<f32> {
        self.position
    }

    pub fn color(&self) -> &str {
        &self.color
    }
}
