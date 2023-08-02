use crate::neuron::Neuron;

pub struct Layer {
    pub(crate) neurons: Vec<Neuron>,
}

impl Layer {
    pub fn new(neurons: Vec<Neuron>) -> Self {
        Self { neurons }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons.iter().map(|n| n.propagate(&inputs)).collect()
    }

    pub(crate) fn random(input_neurons: usize, output_neurons: usize) -> Self {
        let mut rng = rand::thread_rng();

        let neurons = (0..output_neurons)
            .map(|_| Neuron::random(&mut rng, input_neurons))
            .collect();
        Layer { neurons }
    }

    pub fn from_weights(
        input_size: usize,
        output_size: usize,
        weights: &mut dyn Iterator<Item = f32>,
    ) -> Self {
        let neurons = (0..output_size)
            .map(|_| Neuron::from_weights(input_size, weights))
            .collect();

        Self { neurons }
    }
}