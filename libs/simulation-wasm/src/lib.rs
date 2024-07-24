use lib_simulation as sim;
use rand::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Simulation {
	rng: ThreadRng,
	sim: sim::Simulation,
}

#[wasm_bindgen]
impl Simulation {
	#[wasm_bindgen(constructor)]
	pub fn new() -> Self {
		let mut rng = thread_rng();
		let sim = sim::Simulation::random(&mut rng);

		Self { rng, sim }
	}

	pub fn world(&self) -> World {
		World::from(self.sim.world())
	}

	pub fn step(&mut self) {
		self.sim.step(&mut self.rng);
	}
}

impl From<&sim::World> for World {
	fn from(world: &sim::World) -> Self {
		let animals = world.animals().iter().map(Animal::from).collect();
		let foods = world.food().iter().map(Food::from).collect();

		Self { animals, foods }
	}
}

impl From<&sim::Animal> for Animal {
	fn from(animal: &sim::Animal) -> Self {
		Self {
			x: animal.position().x,
			y: animal.position().y,
			rotation: animal.rotation().angle(),
		}
	}
}

impl From<&sim::Food> for Food {
	fn from(food: &sim::Food) -> Self {
		Self {
			x: food.position().x,
			y: food.position().y
		}
	}
}
#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct World {
	#[wasm_bindgen(getter_with_clone)]
	pub animals: Vec<Animal>,
	#[wasm_bindgen(getter_with_clone)]
	pub foods: Vec<Food>,
}


#[wasm_bindgen]
#[derive(Clone, Debug, Copy)]
pub struct Animal {
	pub x: f32,
	pub y: f32,
	pub rotation: f32,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Copy)]
pub struct Food {
	pub x: f32,
	pub y: f32,
}