use rand::Rng;

#[derive(Debug)]
pub struct Network {
	layers: Vec<Layer>,
}

impl Network {
	pub fn new(layers: Vec<Layer>) -> Self {
		Self { layers }
	}

	pub fn random(neurons_per_layer: Vec<usize>) -> Self {
		assert!(neurons_per_layer.len() > 1);
		let mut built_layers = Vec::new();

		for i in 0..neurons_per_layer.len() - 1 {
			let input_size = neurons_per_layer[i];
			let output_size = neurons_per_layer[i + 1];

			built_layers.push(Layer::random(input_size, output_size));
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

	fn random(input_size: usize, output_size: usize) -> Self {
		let mut neurons = Vec::new();
		for _ in 0..output_size {
			neurons.push(Neuron::random(input_size));
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

	fn random(input_size: usize) -> Self {
		let mut rng = rand::thread_rng();
		let bias = rng.gen_range(-1.0..=1.0);

		let weights = (0..input_size)
			.map(|_| rng.gen_range(-1.0..=1.0))
			.collect();

		Self {bias, weights}
	}
}