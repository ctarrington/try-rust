import { Universe } from 'life-comparison';
import { createMovingAverage } from './utils';

const pre = document.getElementById('game-of-life-text');
const elapsedSpan = document.getElementById('elapsed-span');
const averageElapsedSpan = document.getElementById('average-elapsed-span');
const minimumElapsedSpan = document.getElementById('minimum-elapsed-span');
const maximumElapsedSpan = document.getElementById('maximum-elapsed-span');

const universe = Universe.new();
const movingAverage = createMovingAverage(1000);

const renderLoop = () => {
  const start = performance.now();
  pre.textContent = universe.render_as_text();
  universe.tick();

  const elapsed = performance.now() - start;
  const average = movingAverage.add(elapsed);
  const minimum = movingAverage.min();
  const maximum = movingAverage.max();

  elapsedSpan.textContent = elapsed.toFixed(2);
  averageElapsedSpan.textContent = average.toFixed(2);
  minimumElapsedSpan.textContent = minimum.toFixed(2);
  maximumElapsedSpan.textContent = maximum.toFixed(2);

  requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);