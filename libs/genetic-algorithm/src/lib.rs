use rand::seq::SliceRandom;
use rand::RngCore;

pub struct GeneticAlgorithm<S> {
	selection_method: S,
}

impl<S> GeneticAlgorithm<S>
where 
	S: SelectionMethod,
{
	pub fn new(selection_method: S) -> Self {
		Self { selection_method }
	}
	pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I>
	where I: Individual
	{
		assert!(!population.is_empty());
		(0..population.len())
			.map(|_| {
				let parent_a = self.selection_method.select(rng, population);
				let parent_b = self.selection_method.select(rng, population);
				// TODO crossover
				// TODO mutation
				todo!()
			})
			.collect()
	}
}

pub trait Individual {
	fn fitness(&self) -> f32;
}

pub trait SelectionMethod {
	fn select<'a, I>(&self, rng: &mut dyn RngCore,  population: &'a [I]) -> &'a I
	where 
		I: Individual;
}

pub struct RouletteWheelSelection;

impl RouletteWheelSelection {
	pub fn new() -> Self {
		Self
	}
}

impl SelectionMethod for RouletteWheelSelection {
	fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
	where 
		I: Individual
	{
		population
			.choose_weighted(rng, |individual| individual.fitness())
			.expect("get an empty population")
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use rand::SeedableRng;
	use rand_chacha::ChaCha8Rng;
	use std::collections::BTreeMap;
	use std::iter::FromIterator;

	#[test]
	fn roulette_wheel_selection() {
		let mut rng = ChaCha8Rng::from_seed(Default::default());

		let population = vec![
			TestIndividual::new(1.0),
			TestIndividual::new(2.0),
			TestIndividual::new(3.0),
			TestIndividual::new(4.0),
		];
		let actual = RouletteWheelSelection::new().select(&mut rng, &population);
		assert!(actual.fitness == (&population[1]).fitness);

		let mut action_histogram = BTreeMap::new();

		for _ in 0..1000 {
			let fitness = RouletteWheelSelection::new()
				.select(&mut rng, &population).fitness() as i32;
			*action_histogram.entry(fitness).or_insert(0) += 1;
		}

		let expected_histogram = BTreeMap::from_iter(vec![
			(1, 102),
			(2, 198),
			(3, 301),
			(4, 399),
		]);
		assert_eq!(action_histogram, expected_histogram);
	}

	#[derive(Clone, Debug)]
	struct TestIndividual {
		fitness: f32,
	}

	impl TestIndividual {
		fn new(fitness: f32) -> Self {
			Self { fitness }
		}
	}

	impl Individual for TestIndividual {
		fn fitness(&self) -> f32 {
			self.fitness
		}
	}

}