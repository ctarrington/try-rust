import { memory } from "life-comparison/life_comparison_bg";
import { Universe, Cell } from 'life-comparison';
import { createMovingAverage } from './utils';

const CELL_SIZE = 5;
const GRID_COLOR = '#CCCCCC';
const DEAD_COLOR = '#FFFFFF';
const ALIVE_COLOR = '#000000';

const defaultUniverse = Universe.new();
const width = defaultUniverse.width();
const height = defaultUniverse.height();

const getIndex = (row, column) => {
    return row * width + column;
};

const drawGrid = (ctx) => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let columnIndex = 0; columnIndex <= width; columnIndex++) {
    ctx.moveTo(columnIndex * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(columnIndex * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }
  
   // Horizontal lines.
   for (let rowIndex = 0; rowIndex <= height; rowIndex++) {
     ctx.moveTo(0, rowIndex * (CELL_SIZE + 1) + 1);
     ctx.lineTo((CELL_SIZE + 1) * width + 1, rowIndex * (CELL_SIZE + 1) + 1);
   }
 
  ctx.stroke();
};

const drawCells = (ctx, universe) => {

  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col);

      ctx.fillStyle = cells[idx] === Cell.Dead
        ? DEAD_COLOR
        : ALIVE_COLOR;

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
};

const pre = document.getElementById('game-of-life-text');
const canvas = document.getElementById('game-of-life-canvas');
canvas.style.display = 'none';

const elapsedSpan = document.getElementById('elapsed-span');
const averageElapsedSpan = document.getElementById('average-elapsed-span');
const minimumElapsedSpan = document.getElementById('minimum-elapsed-span');
const maximumElapsedSpan = document.getElementById('maximum-elapsed-span');
const currentScenarioSpan = document.getElementById('current-scenario-span');
const incrementScenarioButton = document.getElementById('increment-scenario-button');

const rustPassResultsScenario = {
  createUniverse: () => Universe.new(),
  render: () => {
    pre.textContent = universe.render_as_text();
    universe.tick();
  },
  clear: () => {
    pre.textContent = '';
  },
  name: 'Pass the results as a string',
};

let currentScenarioCounter = -1;
let universe = null;
const movingAverage = createMovingAverage(1000);

const rustPassResultsReferenceScenario = {
  createUniverse: () => {
    const universe = Universe.new();
    canvas.style.display = '';
    canvas.height = (CELL_SIZE + 1) * universe.height() + 1;
    canvas.width = (CELL_SIZE + 1) * universe.width() + 1;
    return universe;
  },
  render: () => {
    const ctx = canvas.getContext('2d');
    drawGrid(ctx);
    drawCells(ctx, universe);
    
    universe.tick();
  },
  clear: () => {
    canvas.style.display = 'none';
  },
  name: 'Pass a reference to the results',
};

const scenarios = [rustPassResultsScenario, rustPassResultsReferenceScenario];

const incrementScenario = () => {
  if (currentScenarioCounter >= 0) {
    scenarios[currentScenarioCounter].clear();
    movingAverage.clear();
  }

  currentScenarioCounter = (currentScenarioCounter + 1) % scenarios.length;
  const scenario = scenarios[currentScenarioCounter];
  universe = scenario.createUniverse();

};

incrementScenario();

const renderLoop = () => {
  const scenario = scenarios[currentScenarioCounter];
  const start = performance.now();
  scenario.render();

  const elapsed = performance.now() - start;
  const average = movingAverage.add(elapsed);
  const minimum = movingAverage.min();
  const maximum = movingAverage.max();

  elapsedSpan.textContent = elapsed.toFixed(2);
  averageElapsedSpan.textContent = average.toFixed(2);
  minimumElapsedSpan.textContent = minimum.toFixed(2);
  maximumElapsedSpan.textContent = maximum.toFixed(2);
  currentScenarioSpan.textContent = scenario.name;

  requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);

incrementScenarioButton.onclick = incrementScenario;
