use rand::{Rng, RngCore};

#[derive(Debug)]
pub struct Network {
	layers: Vec<Layer>,
}

impl Network {
	pub fn new(layers: Vec<Layer>) -> Self {
		Self { layers }
	}

	pub fn random(rng: &mut dyn RngCore, neurons_per_layer: Vec<usize>) -> Self {
		assert!(neurons_per_layer.len() > 1);
		let mut built_layers = Vec::new();

		for i in 0..neurons_per_layer.len() - 1 {
			let input_size = neurons_per_layer[i];
			let output_size = neurons_per_layer[i + 1];

			built_layers.push(Layer::random(rng, input_size, output_size));
		}

		Self {layers: built_layers}

	}

	pub fn propagate(&self, mut inputs: Vec<f32>) -> Vec<f32> {
		for layer in &self.layers {
			inputs = layer.propagate(inputs);
		}

		inputs
		// self.layers
		// 	.iter()
		// 	.fold(inputs, |inputs, layer| layer.propagate(inputs))
	}
}

#[derive(Debug)]
pub struct Layer {
	neurons: Vec<Neuron>
}

impl Layer {
	fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
		let mut outputs = Vec::new();
		for neuron in &self.neurons {
			let output = neuron.propagate(&inputs);
			outputs.push(output);
		}
		outputs
	}

	fn random(rng: &mut dyn RngCore, input_size: usize, output_size: usize) -> Self {
		let mut neurons = Vec::new();
		for _ in 0..output_size {
			neurons.push(Neuron::random(rng, input_size));
		}
		Self {neurons}
	}

}

#[derive(Debug)]
struct Neuron {
	bias: f32,
	weights: Vec<f32>
}

impl Neuron {
	fn propagate(&self, inputs: &[f32]) -> f32 {
		assert_eq!(inputs.len(), self.weights.len());
		let mut output = 0.0;

		for i in 0..inputs.len() {
			output += inputs[i] * self.weights[i];
		}
		
		(self.bias + output).max(0.0)
	}

	fn random(rng: &mut dyn RngCore, input_size: usize) -> Self {
		// let mut rng = rand::thread_rng();
		let bias = rng.gen_range(-1.0..=1.0);

		let weights = (0..input_size)
			.map(|_| rng.gen_range(-1.0..=1.0))
			.collect();

		Self {bias, weights}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use approx::assert_relative_eq;
	use rand::SeedableRng;
	use rand_chacha::ChaCha8Rng;

	#[test]
	fn random() {
		let mut rng = ChaCha8Rng::from_seed(Default::default());
		let neuron = Neuron::random(&mut rng, 4);
		assert_relative_eq!(neuron.bias, -0.6255188);
		assert_relative_eq!(neuron.weights.as_slice(), [0.67383957, 0.8181262, 0.26284897, 0.5238807].as_ref());
	}

	#[test]
	fn propagate() {
		let neuron = Neuron {
			bias: 0.5,
			weights: vec![-0.3, 0.8],
		};

		assert_relative_eq!(neuron.propagate(&[-10.0, -10.0]), 0.0);
		assert_relative_eq!(neuron.propagate(&[0.5, 1.0]), (0.5 * -0.3 + 1.0 * 0.8 + 0.5));
	} }