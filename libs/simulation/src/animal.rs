use lib_genetic_algorithm::Chromosome;

use crate::*;

#[derive(Debug)]
pub struct Animal {
	pub(crate) position: na::Point2<f32>,
	pub(crate) rotation: na::Rotation2<f32>,
	pub(crate) speed: f32,
	pub(crate) eye: Eye,
	pub(crate) brain: brain::Brain,
	// Number of foods
	pub(crate) satiation: usize
}

impl Animal {
	pub fn random(rng: &mut dyn RngCore) -> Self {
		let eye = Eye::default();
		let brain = Brain::random(rng, &eye);
		Self::new(eye, brain, rng)
	}

	pub(crate) fn from_chromosome(
		chromosome: ga::Chromosome,
		rng: &mut dyn RngCore,
	) -> Self {
		let eye = Eye::default();
		let brain = Brain::from_chromosome(chromosome, &eye);

		Self::new(eye, brain, rng)
	}

	pub(crate) fn as_chromosome(&self) -> Chromosome {
		self.brain.as_chromosome()
	}

	fn new(eye: Eye, brain: Brain, rng: &mut dyn RngCore) -> Self {
		Self {
			position: rng.gen(),
			rotation: rng.gen(),
			speed: 0.002,
			eye,
			brain,
			satiation: 0,
		}
	}

	pub fn position(&self) -> na::Point2<f32> {
		self.position
	}

	pub fn rotation(&self) -> na::Rotation2<f32> {
		self.rotation
	}

	pub fn speed(&self) -> f32 {
		self.speed
	}

	pub fn fitness(&self) -> usize {
		self.satiation
	}

}
