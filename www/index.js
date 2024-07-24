import * as sim from "lib-simulation-wasm"

const simulation = new sim.Simulation();
const world = simulation.world();

const viewport = document.getElementById("viewport");
const viewportWidth = viewport.width;
const viewportHeight = viewport.height;

const viewportScale = window.devicePixelRatio || 1;
viewport.width = viewportWidth * viewportScale;
viewport.height = viewportHeight * viewportScale;
viewport.style.width = viewportWidth + "px";
viewport.style.height = viewportHeight + "px";

const ctxt = viewport.getContext("2d");
ctxt.scale(viewportScale, viewportScale);

function drawTriangle(ctxt, x, y, size, rotation) {
  ctxt.beginPath();
  ctxt.moveTo(x - Math.sin(rotation) * 1.5, y + Math.cos(rotation) * 1.5);
  ctxt.lineTo(
    x - Math.sin(rotation + 2.0 / 3.0 * Math.PI) * size,
    y + Math.cos(rotation + 2.0 / 3.0 * Math.PI) * size,
  );
  ctxt.lineTo(
    x - Math.sin(rotation + 4.0 / 3.0 * Math.PI) * size,
    y + Math.cos(rotation + 4.0 / 3.0 * Math.PI) * size,
  );
  ctxt.lineTo(x - Math.sin(rotation) * 1.5, y + Math.cos(rotation) * 1.5);
  ctxt.fillStyle = 'rgb(255, 255, 255)';
  ctxt.fill();
}

function drawCircle(ctxt, x, y, radius) {
  ctxt.beginPath();
  ctxt.arc(x, y, radius, 0, 2.0 * Math.PI);
  ctxt.fillStyle = 'rgb(0, 255, 128)';
  ctxt.fill();
}

function redraw() {
  ctxt.clearRect(0, 0, viewportWidth, viewportHeight);
  const world = simulation.world();
  if (simulation.is_last_run()) {
    let avg_fitness = 0;
    let max_fitness = 0;
    for (const animal of world.animals) {
      avg_fitness += animal.fitness;
      max_fitness = Math.max(max_fitness, animal.fitness);
    }
    avg_fitness /= world.animals.length;
    console.log(`Generation ${simulation.generation()} - Average fitness: ${avg_fitness}, Max fitness: ${max_fitness}`);
  }
  simulation.step();

  for (const food of world.foods) {
    drawCircle(
      ctxt,
      food.x * viewportWidth,
      food.y * viewportHeight,
      0.003 * viewportWidth,
    );
  }

  for (const animal of world.animals) {
    drawTriangle(
      ctxt,
      animal.x * viewportWidth,
      animal.y * viewportHeight,
      0.01 * viewportWidth,
      animal.rotation,
    );
  }

  requestAnimationFrame(redraw);
}

console.log("Starting shorelark");
redraw();