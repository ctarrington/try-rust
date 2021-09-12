import { Universe, Cell } from 'life-comparison';
import { createJSUniverse, JSCell } from './js-universe';
import { createMovingAverage, drawGrid, drawCellsFromReference, formatCellsFromReference, formatCellsFromView} from './utils';

const CELL_SIZE = 5;

const defaultUniverse = Universe.new();
const width = defaultUniverse.width();
const height = defaultUniverse.height();

const pre = document.getElementById('game-of-life-text');
const canvas = document.getElementById('game-of-life-canvas');
canvas.height = (CELL_SIZE + 1) * defaultUniverse.height() + 1;
canvas.width = (CELL_SIZE + 1) * defaultUniverse.width() + 1;
canvas.style.display = 'none';

const elapsedSpan = document.getElementById('elapsed-span');
const averageElapsedSpan = document.getElementById('average-elapsed-span');
const minimumElapsedSpan = document.getElementById('minimum-elapsed-span');
const maximumElapsedSpan = document.getElementById('maximum-elapsed-span');
const currentScenarioSpan = document.getElementById('current-scenario-span');
const incrementScenarioButton = document.getElementById('increment-scenario-button');

const rustPassResultsAsTextScenario = {
  universe: Universe.new(),
  render: () => {
    pre.textContent = rustPassResultsAsTextScenario.universe.render_as_text();
    rustPassResultsAsTextScenario.universe.tick();
  },
  clear: () => {
    pre.textContent = '';
  },
  name: 'Pass the results as a string',
  movingAverage: createMovingAverage(1000),
};

const rustPassResultsReferenceTextInJavaScriptScenario = {
  universe: Universe.new(),
  render: () => {

    pre.textContent = formatCellsFromReference(rustPassResultsReferenceTextInJavaScriptScenario.universe, width, height);
    rustPassResultsReferenceTextInJavaScriptScenario.universe.tick();
  },
  clear: () => {
    pre.textContent = '';
  },
  name: 'Pass a reference to the results, build the text in JavaScript',
  movingAverage: createMovingAverage(1000),
};


const rustPassResultsReferenceToCanvasScenario = {
  universe: Universe.new(),
  render: () => {
    canvas.style.display = '';
    const ctx = canvas.getContext('2d');
    drawGrid(ctx, width, height);
    drawCellsFromReference(ctx, rustPassResultsReferenceToCanvasScenario.universe, width, height);
    
    rustPassResultsReferenceToCanvasScenario.universe.tick();
  },
  clear: () => {
    canvas.style.display = 'none';
  },
  name: 'Pass a reference to the results',
  movingAverage: createMovingAverage(1000),
};

const jsUniverseJSText = {
  universe: createJSUniverse(width, height),
  render: () => {
    pre.textContent = formatCellsFromView(jsUniverseJSText.universe, width, height);
    jsUniverseJSText.universe.tick();
  },
  clear: () => {
    pre.textContent = '';
  },
  name: 'Universe in JavaScript, build the text in JavaScript',
  movingAverage: createMovingAverage(1000),
};

const jsResultsToCanvasScenario = {
  universe: createJSUniverse(width, height),
  render: () => {
    canvas.style.display = '';
    const ctx = canvas.getContext('2d');
    drawGrid(ctx, width, height);
    drawCellsFromReference(ctx, jsResultsToCanvasScenario.universe, width, height);
    
    jsResultsToCanvasScenario.universe.tick();
  },
  clear: () => {
    canvas.style.display = 'none';
  },
  name: 'Pass a reference to the results',
  movingAverage: createMovingAverage(1000),
};

let currentScenarioCounter = -1;
const scenarios = [rustPassResultsAsTextScenario, rustPassResultsReferenceTextInJavaScriptScenario, rustPassResultsReferenceToCanvasScenario, jsUniverseJSText];

const incrementScenario = () => {
  if (currentScenarioCounter >= 0) {
    scenarios[currentScenarioCounter].clear();
  }

  currentScenarioCounter = (currentScenarioCounter + 1) % scenarios.length;
  const scenario = scenarios[currentScenarioCounter];
};

incrementScenario();

const renderLoop = () => {
  const scenario = scenarios[currentScenarioCounter];
  const start = performance.now();
  scenario.render();

  const elapsed = performance.now() - start;
  const average = scenario.movingAverage.add(elapsed);
  const minimum = scenario.movingAverage.min();
  const maximum = scenario.movingAverage.max();

  elapsedSpan.textContent = elapsed.toFixed(2);
  averageElapsedSpan.textContent = average.toFixed(2);
  minimumElapsedSpan.textContent = minimum.toFixed(2);
  maximumElapsedSpan.textContent = maximum.toFixed(2);
  currentScenarioSpan.textContent = scenario.name;

  requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);

incrementScenarioButton.onclick = incrementScenario;
