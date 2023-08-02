use rand::Rng;

pub struct Neuron {
    pub(crate) bias: f32,
    pub(crate) weights: Vec<f32>,
}

impl Neuron {
    pub fn new(bias: f32, weights: Vec<f32>) -> Self {
        Self { bias, weights }
    }

    pub fn propagate(&self, inputs: &[f32]) -> f32 {
        assert!(inputs.len() == self.weights.len());
        (inputs
            .iter()
            .zip(self.weights.iter())
            .map(|(input, weight)| input * weight)
            .sum::<f32>()
            + self.bias)
            .max(0.0)
    }

    pub fn random(rng: &mut dyn rand::RngCore, input_neurons: usize) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..input_neurons)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();

        Self { bias, weights }
    }

    pub fn from_weights(output_neurons: usize, weights: &mut dyn Iterator<Item = f32>) -> Self {
        let bias = weights.next().expect("got not enough weights");

        let weights = (0..output_neurons)
            .map(|_| weights.next().expect("got not enough weights"))
            .collect();

        Self { bias, weights }
    }
}