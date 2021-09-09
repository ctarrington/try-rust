import { Universe, Cell } from 'life-comparison';
import { createMovingAverage, drawGrid, drawCellsFromReference } from './utils';

const CELL_SIZE = 5;

const defaultUniverse = Universe.new();
const width = defaultUniverse.width();
const height = defaultUniverse.height();

const pre = document.getElementById('game-of-life-text');
const canvas = document.getElementById('game-of-life-canvas');
canvas.style.display = 'none';

const elapsedSpan = document.getElementById('elapsed-span');
const averageElapsedSpan = document.getElementById('average-elapsed-span');
const minimumElapsedSpan = document.getElementById('minimum-elapsed-span');
const maximumElapsedSpan = document.getElementById('maximum-elapsed-span');
const currentScenarioSpan = document.getElementById('current-scenario-span');
const incrementScenarioButton = document.getElementById('increment-scenario-button');

const rustPassResultsAsTextScenario = {
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
    drawGrid(ctx, width, height);
    drawCellsFromReference(ctx, universe, width, height);
    
    universe.tick();
  },
  clear: () => {
    canvas.style.display = 'none';
  },
  name: 'Pass a reference to the results',
};

const scenarios = [rustPassResultsAsTextScenario, rustPassResultsReferenceScenario];

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
