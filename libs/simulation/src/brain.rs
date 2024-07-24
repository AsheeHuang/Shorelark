use crate::*;

#[derive(Debug)]
pub struct Brain {
	pub(crate) nn: nn::Network,
}

impl Brain {
	pub fn random(rng: &mut dyn RngCore, eye: &Eye) -> Self {
		Self {
			nn: nn::Network::random(rng, &Self::topology(eye)),
		}
	}

	pub(crate) fn from_chromosome(
		chromosome: ga::Chromosome,
		eye: &Eye,
	) -> Self {
		Self {
			nn: nn::Network::from_weights(
				&Self::topology(eye),
				chromosome,
			),
		}
	}

	pub(crate) fn as_chromosome(&self) -> ga::Chromosome {
		ga::Chromosome::new(self.nn.weights())
	}

	fn topology(eye: &Eye) -> Vec<nn::LayerTopology> {
		vec![
			nn::LayerTopology {
				neurons: eye.cells(),
			},
			nn::LayerTopology {
				neurons: 2 * eye.cells(),
			},
			nn::LayerTopology { neurons: 2 },
		]
	}
}

