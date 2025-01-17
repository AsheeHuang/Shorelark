use rand::{Rng, RngCore};

#[derive(Debug)]
pub struct Network {
	layers: Vec<Layer>,
}

impl Network {
	pub fn new(layers: Vec<Layer>) -> Self {
		Self { layers }
	}

	pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
		assert!(layers.len() > 1);
		let mut built_layers = Vec::new();

		for i in 0..(layers.len() - 1) {
			let input_size = layers[i].neurons;
			let output_size = layers[i + 1].neurons;

			built_layers.push(Layer::random(
				rng,
				input_size,
				output_size,
			));
		}

		Self { layers: built_layers }
	}

	pub fn propagate(&self, mut inputs: Vec<f32>) -> Vec<f32> {
		for layer in &self.layers {
			inputs = layer.propagate(inputs);
		}

		inputs
	}

	pub fn weights(&self) -> Vec<f32> {
		let mut weights = Vec::new();

		for layer in &self.layers {
			for neuron in &layer.neurons {
				weights.push(neuron.bias);

				for weight in &neuron.weights {
					weights.push(*weight);
				}
			}
		}

		weights
	}

	pub fn from_weights(
		layers: &[LayerTopology],
		weight: impl IntoIterator<Item = f32>,
	) -> Self {
		assert!(layers.len() > 1);

		let mut weights = weight.into_iter();

		let layers = layers
			.windows(2)
			.map(|layers| {
				Layer::from_weights(
					layers[0].neurons,
					layers[1].neurons,
					&mut weights,
				)
			})
			.collect();
		
		if weights.next().is_some() {
			panic!("got too many weights");
		}

		Self {layers}
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

	fn from_weights(
		input_size: usize,
		output_size: usize,
		weights: &mut impl Iterator<Item = f32>,
	) -> Self {
		let neurons = (0..output_size)
			.map(|_| Neuron::from_weights(input_size, weights))
			.collect();

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

	fn from_weights(
		input_size: usize,
		weights: &mut dyn Iterator<Item = f32>,
	) -> Self {
		let bias = weights.next().expect("got not enough weights");

		let weights = (0..input_size)
			.map(|_| weights.next().expect("got not enough weights"))
			.collect();

		Self { bias, weights }
	}
}

#[derive(Debug)]
pub struct LayerTopology {
	pub neurons: usize,
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
	} 
	// TODO: test weight
}