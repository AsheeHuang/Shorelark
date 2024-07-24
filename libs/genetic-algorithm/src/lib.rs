use rand::seq::SliceRandom;
use rand::{RngCore, Rng};
use std::ops::Index;

pub struct GeneticAlgorithm<S> {
	selection_method: S,
	crossover_method: Box<dyn CrossoverMethod>,
	mutation_method: Box<dyn MutationMethod>,
}

impl<S> GeneticAlgorithm<S>
where 
	S: SelectionMethod,
{
	pub fn new(selection_method: S,
		crossover_method: impl CrossoverMethod + 'static,
		mutation_method: impl MutationMethod + 'static,
	) -> Self {
		Self { 
			selection_method,
			crossover_method: Box::new(crossover_method),
			mutation_method: Box::new(mutation_method),
		}
	}
	pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I>
	where I: Individual
	{
		assert!(!population.is_empty());
		(0..population.len())
			.map(|_| {
				let parent_a = self.selection_method.select(rng, population).chromosome();
				let parent_b = self.selection_method.select(rng, population).chromosome();
				let mut child = self.crossover_method.crossover(rng, parent_a, parent_b);
				self.mutation_method.mutate(rng, &mut child);

				I::create(child)
			})
			.collect()
	}
}

pub trait Individual {
	fn create(chromosome: Chromosome) -> Self;
	fn fitness(&self) -> f32;
	fn chromosome(&self) -> &Chromosome;
}

pub trait SelectionMethod {
	fn select<'a, I>(&self, rng: &mut dyn RngCore,  population: &'a [I]) -> &'a I
	where 
		I: Individual;
}

pub struct RouletteWheelSelection;

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

pub trait CrossoverMethod {
	fn crossover(
		&self,
		rng: &mut dyn RngCore,
		parent_a: &Chromosome,
		parent_b: &Chromosome,
	) -> Chromosome;
}

#[derive(Clone, Debug)]
pub struct UniformCrossover;

impl CrossoverMethod for UniformCrossover {
	fn crossover(
		&self,
		rng: &mut dyn RngCore,
		parent_a: &Chromosome,
		parent_b: &Chromosome,
	) -> Chromosome {
		assert_eq!(parent_a.len(), parent_b.len());
		let mut child = Vec::new();
		let gene_count = parent_a.len();

		for idx in 0..gene_count {
			let gene= if rng.gen_bool(0.5) {
				parent_a[idx]
			} else {
				parent_b[idx]
			};
			child.push(gene);
		}

		child.into_iter().collect()
	}
}

pub trait MutationMethod {
	fn mutate(&self, rng: &mut dyn RngCore, chromosome: &mut Chromosome);
}

#[derive(Clone, Debug)]
pub struct GaussianMutation {
	// The probability of a gene being mutated, 0 <= chance <= 1
	chance: f32,
	// Magnitude of the mutation, 0 <= coeff <= 3
	coeff: f32,
}

impl GaussianMutation { 
	pub fn new(chance: f32, coeff: f32) -> Self {
		assert!(0.0 <= chance && chance <= 1.0);
		assert!(0.0 <= coeff && coeff <= 3.0);
		Self { chance, coeff }
	}
}

impl MutationMethod for GaussianMutation {
	fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
		for gene in child.iter_mut() {
			let sign = if rng.gen_bool(0.5) {-1.0} else {1.0};

			if rng.gen_bool(self.chance as f64) {
				*gene += sign * self.coeff * rng.gen::<f32>();
			}
		}

	}
}

#[derive(Clone, Debug)]
pub struct Chromosome {
	genes: Vec<f32>,
}

impl Chromosome {
	pub fn new(genes: Vec<f32>) -> Self {
		Self { genes }
	}

	pub fn len(&self) -> usize {
		self.genes.len()
	}

	pub fn iter(&self) -> impl Iterator<Item = &f32> {
		self.genes.iter()
	}

	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
		self.genes.iter_mut()
	}
	
}

impl Index<usize> for Chromosome {
	type Output = f32;

	fn index(&self, index:usize) -> &Self::Output {
		&self.genes[index]
	}
}

impl FromIterator<f32> for Chromosome {
	fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
		Self {
			genes: iter.into_iter().collect()
		}
	}
}

impl IntoIterator for Chromosome {
	type Item = f32;
	type IntoIter = std::vec::IntoIter<f32>;

	fn into_iter(self) -> Self::IntoIter {
		self.genes.into_iter()
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
		let actual = RouletteWheelSelection.select(&mut rng, &population);
		assert!(actual.fitness() == (&population[1]).fitness());

		let mut action_histogram = BTreeMap::new();

		for _ in 0..1000 {
			let fitness = RouletteWheelSelection.select(&mut rng, &population).fitness() as i32;
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

	#[test]
	fn uniform_crossover() {
		let mut rng = ChaCha8Rng::from_seed(Default::default());
		let parent_a: Chromosome = (1..=100).map(|n| n as f32).collect();
		let parent_b: Chromosome = (1..=100).map(|n| -n as f32).collect();
		let child = UniformCrossover.crossover(&mut rng, &parent_a, &parent_b);

		 let diff_a = child.iter().zip(parent_a.iter()).filter(|(c, p)| c != p).count();
		 let diff_b = child.iter().zip(parent_b.iter()).filter(|(c, p)| c != p).count();

		assert_eq!(diff_a, 49);
		assert_eq!(diff_b, 51);
	}

	#[derive(Clone, Debug, PartialEq)]
	enum TestIndividual {
		WithChromosome { chromosome: Chromosome },
		WithFitness { fitness: f32 },
	}

	impl TestIndividual {
		fn new(fitness: f32) -> Self {
			Self::WithFitness { fitness }
		}
	}

	impl Individual for TestIndividual {
		fn create(chromosome: Chromosome) -> Self {
			Self::WithChromosome { chromosome }
		}

		fn chromosome(&self) -> &Chromosome {
			match self {
				Self::WithChromosome { chromosome } => chromosome,
				Self::WithFitness { .. } => {
					panic!("No chromosome")
				}
			}
		}

		fn fitness(&self) -> f32 {
			match self {
				Self::WithChromosome { .. } => {
					self.chromosome().iter().sum::<f32>() / self.chromosome().len() as f32
				}
				Self::WithFitness { fitness } => *fitness,
			}
		}
	}

	impl PartialEq for Chromosome {
		fn eq(&self, other: &Self) -> bool {
			approx::relative_eq!(self.genes.as_slice(), other.genes.as_slice())
		}
	}

	mod gausssian_mutation {
		use super::*;

		fn actual(chance: f32, coeff: f32) -> Vec<f32>{
			let mut rng = ChaCha8Rng::from_seed(Default::default());
			let mut child = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_iter().collect();
			GaussianMutation::new(chance, coeff).mutate(&mut rng, &mut child);

			child.iter().copied().collect()
		}
		mod given_zero_chance {
			use approx::assert_relative_eq;

			fn actual(coeff: f32) -> Vec<f32> {
				super::actual(0.0, coeff)
			}
			mod and_zero_coefficient {
				use super::*;
				#[test]
				fn does_not_change_the_original_chromosome() {
					let actual: Vec<f32> = actual(0.0);
					let expected: Vec<f32>= vec![1.0, 2.0, 3.0, 4.0, 5.0];
					assert_relative_eq!(actual.as_slice(), expected.as_slice());
				}
			}
			mod and_none_zero_coefficient {
				use super::*;
				#[test]
				fn does_not_change_the_original_chromosome() {
					let actual: Vec<f32> = actual(0.5);
					let expected: Vec<f32>= vec![1.0, 2.0, 3.0, 4.0, 5.0];
					assert_relative_eq!(actual.as_slice(), expected.as_slice());
				}
			}
		}

		mod given_fifty_fifty_chance {
			use approx::assert_relative_eq;

			fn actual(coeff: f32) -> Vec<f32> {
				super::actual(0.5, coeff)
			}
			mod and_zero_coefficient {
				use super::*;
				#[test]
				fn does_not_change_the_original_chromosome() {
					let actual: Vec<f32> = actual(0.0);
					let expected: Vec<f32>= vec![1.0, 2.0, 3.0, 4.0, 5.0];
					assert_relative_eq!(actual.as_slice(), expected.as_slice());
				}
			}
			mod and_none_zero_coefficient {
				use super::*;
				#[test]
				fn slightly_changes_the_original_chromosome() {
					let actual: Vec<f32> = actual(0.5);
					let expected: Vec<f32>= vec![1.0, 2.0, 3.0, 4.0, 5.0];
					assert_relative_eq!(actual.as_slice(), expected.as_slice(), epsilon = 0.5);
				}
			}

		}

		mod given_max_chance {
			use approx::assert_relative_eq;

			fn actual(coeff: f32) -> Vec<f32> {
				super::actual(1.0, coeff)
			}
			mod and_zero_coefficient {
				use super::*;
				#[test]
				fn does_not_change_the_original_chromosome() {
					let actual: Vec<f32> = actual(0.0);
					let expected: Vec<f32>= vec![1.0, 2.0, 3.0, 4.0, 5.0];
					assert_relative_eq!(actual.as_slice(), expected.as_slice());
				}
			}
			mod and_none_zero_coefficient {
				use super::*;
				#[test]
				fn entirely_change_the_original_chromosome() {
					let actual: Vec<f32> = actual(0.5);
					let expected: Vec<f32>= vec![1.0, 2.0, 3.0, 4.0, 5.0];
					assert_relative_eq!(actual.as_slice(), expected.as_slice(), epsilon = 0.5);
				}
			}
		}
	}
	#[test]
	fn genetic_algorithm() {
		fn individual(gene: &[f32]) -> TestIndividual {
			TestIndividual::create(gene.iter().cloned().collect())
		}
		let mut rng = ChaCha8Rng::from_seed(Default::default());
		let ga = GeneticAlgorithm::new(
			RouletteWheelSelection,
			UniformCrossover,
			GaussianMutation::new(0.5, 0.5),
		);

		let mut population: Vec<TestIndividual> = vec![
			individual(&[0.0, 0.0, 0.0]),
			individual(&[1.0, 1.0, 1.0]),
			individual(&[1.0, 2.0, 1.0]),
			individual(&[1.0, 2.0, 4.0]),
		];

		for _ in 0..10 {
			population = ga.evolve(&mut rng, &population);
		}

		let expect_population: Vec<TestIndividual> = vec![
			individual(&[0.4476949, 2.0648358, 4.3058133]),
			individual(&[1.2126867, 1.5538777, 2.886911]),
			individual(&[1.0617678, 2.265739, 4.428764]),
			individual(&[0.95909685, 2.4618788, 4.024733]),
		];

		assert_eq!(population, expect_population);

	}

}