#[derive(Debug)]
pub struct Network {
	layers: Vec<Layer>,
}

impl Network {
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
struct Layer {
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
		
		output.max(0.0)

	}
}