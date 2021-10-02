import { Universe, Cell } from 'life-comparison';
import { createJSUniverse, JSCell } from './js-universe';
import { createMovingAverage, drawGrid, drawCellsFromReference, formatCellsFromReference, drawCellsFromView, formatCellsFromView, copyImageFromBuffer} from './utils';

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
const summaryPre = document.getElementById('summary');

const rustPassResultsAsTextScenario = {
  universe: Universe.new(),
  render: () => {
    pre.textContent = rustPassResultsAsTextScenario.universe.render_as_text();
    rustPassResultsAsTextScenario.universe.tick();
  },
  clear: () => {
    pre.textContent = '';
  },
  name: 'Rust runs the universe and builds the string which it passes to JavaScript for display',
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
  name: 'Rust passes a reference to the cells to JavaScript which builds the text output',
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
  name: 'Rust passes a reference to the cells to JavaScript which paints the  canvas',
  movingAverage: createMovingAverage(1000),
};

const rustPassImageToCanvasScenario = {
  universe: Universe.new(),
  render: () => {
    canvas.style.display = '';
    const ctx = canvas.getContext('2d');
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    copyImageFromBuffer(ctx, rustPassImageToCanvasScenario.universe, width, height);
    rustPassImageToCanvasScenario.universe.tick();
  },
  clear: () => {
    canvas.style.display = 'none';
  },
  name: 'Rust passes a reference to an image to JavaScript which fills in the  canvas',
  movingAverage: createMovingAverage(1000),
};

const rustWriteResultsToCanvas = {
  universe: Universe.new(),
  render: () => {
    canvas.style.display = '';
    rustWriteResultsToCanvas.universe.render_to_canvas();
    rustWriteResultsToCanvas.universe.tick();
  },
  clear: () => {
    canvas.style.display = 'none';
  },
  name: 'Rust writes the canvas',
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
  name: 'JavaScript runs the universe and builds the text output',
  movingAverage: createMovingAverage(1000),
};

const jsResultsToCanvasScenario = {
  universe: createJSUniverse(width, height),
  render: () => {
    canvas.style.display = '';
    const ctx = canvas.getContext('2d');
    drawGrid(ctx, width, height);
    drawCellsFromView(ctx, jsResultsToCanvasScenario.universe, width, height);
    
    jsResultsToCanvasScenario.universe.tick();
  },
  clear: () => {
    canvas.style.display = 'none';
  },
  name: 'JavaScript runs the universe and paints the canvas',
  movingAverage: createMovingAverage(1000),
};

let currentScenarioCounter = -1;
const scenarios = [jsUniverseJSText, rustPassResultsAsTextScenario, rustPassResultsReferenceTextInJavaScriptScenario, jsResultsToCanvasScenario, rustPassResultsReferenceToCanvasScenario, rustWriteResultsToCanvas, rustPassImageToCanvasScenario];

const incrementScenario = () => {
  if (currentScenarioCounter >= 0) {
    scenarios[currentScenarioCounter].clear();
  }

  currentScenarioCounter = (currentScenarioCounter + 1) % scenarios.length;
  const scenario = scenarios[currentScenarioCounter];

  let summaryContents = '';
  for (const scenario of scenarios) {
    summaryContents += '' + scenario.movingAverage.average().toFixed(2) + ' ' + scenario.movingAverage.max().toFixed(2) + ' ' + scenario.name + '\n';
  }

  summaryPre.textContent = summaryContents;
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
