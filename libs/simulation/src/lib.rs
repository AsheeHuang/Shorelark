mod animal;
mod animal_individual;
mod food;
mod world;
mod eyes;
mod brain;

pub use self::{animal::*, brain::*, eyes::*, food::*, world::*};
use self::animal_individual::*;
use lib_neural_network as nn;
use lib_genetic_algorithm as ga;
use nalgebra as na;
use rand::{Rng, RngCore};

use std::f32::consts::FRAC_PI_2;
const SPEED_MIN: f32 = 0.001;
const SPEED_MAX: f32 = 0.005;
const SPEED_ACCEL: f32 = 0.2;
const ROTATION_ACCEL: f32 = FRAC_PI_2;

const STEP_EACH_GENERATION: usize = 1000;

pub struct Simulation {
	world: World,
	ga: ga::GeneticAlgorithm<ga::RouletteWheelSelection>,
	pub age: usize,
}

impl Simulation {
	pub fn random(rng: &mut dyn RngCore) -> Self {
		let world = World::random(rng);

		let ga = ga::GeneticAlgorithm::new(
			ga::RouletteWheelSelection,
			ga::UniformCrossover,
			ga::GaussianMutation::new(0.005, 0.5),
		);
		Self {
			world,
			ga,
			age: 0,
		}
	}
	
	pub fn world(&self) -> &World {
		&self.world
	}

	pub fn age(&self) -> usize {
		self.age
	}

	pub fn generation(&self) -> usize {
		self.ga.generation()
	}

	pub fn is_last_run(&self) -> bool {
		self.age == STEP_EACH_GENERATION - 1
	}

	pub fn step(&mut self, rng: &mut dyn RngCore) {
		self.process_collision(rng);
		self.process_brains();
		self.process_movement();

		self.age += 1;
		if self.age >= STEP_EACH_GENERATION {
			self.age = 0;
			self.evolve(rng);
		}
	}

	fn process_movement(&mut self) {
		for animal in &mut self.world.animals {
			animal.position += animal.rotation * na::Vector2::new(0.0, animal.speed);

			animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
			animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
		}
	}

	fn process_collision(&mut self, rng: &mut dyn RngCore) {
		for animal in &mut self.world.animals {
			for food in &mut self.world.foods {
				let distance = na::distance(&animal.position, &food.position);

				if distance < 0.01 {
					animal.satiation += 1;
					food.position = rng.gen();
				}
			}
		}
	}

	fn process_brains(&mut self) {
		for animal in &mut self.world.animals {
			let vision = animal.eye.process_vision(
				animal.position,
				animal.rotation,
				&self.world.foods
			);

			let response = animal.brain.nn.propagate(vision);

			let speed = response[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);
			let rotation = response[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);

			animal.speed = (animal.speed + speed).clamp(SPEED_MIN, SPEED_MAX);
			animal.rotation = na::Rotation2::new(animal.rotation.angle() + rotation);
		}

	}

	fn evolve(&mut self, rng: &mut dyn RngCore) {
		self.age = 0;
		let current_population: Vec<_> = self.world.animals.iter().map(AnimalIndividual::from_animal).collect();
		let evovled_population = self.ga.evolve(rng, &current_population);
		self.world.animals = evovled_population.into_iter().map(|individual| individual.into_animal(rng)).collect();

		for food in &mut self.world.foods {
			food.position = rng.gen();
		}
	}

}

