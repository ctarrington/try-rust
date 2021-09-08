import { Universe } from "life-comparison";

const pre = document.getElementById("game-of-life-text");
const elapsedSpan = document.getElementById("elapsed-span");
const averageElapsedSpan = document.getElementById("average-elapsed-span");
const universe = Universe.new();

const createMovingAverage = (thesize) => {
  const size = thesize;
  let currentPosition = 0;
  let sum = 0;
  const values = [];

  const add = (value) => {

    // out with the old, in with the new
    if (currentPosition < values.length) {
      sum -= values[currentPosition];
    }
    values[currentPosition] = value;
    sum += value;
    currentPosition++;
    currentPosition = currentPosition % size;

    return sum/values.length;
  };
  
  return {add};
};

const movingAverage = createMovingAverage(1000);

const renderLoop = () => {
  const start = performance.now();
  pre.textContent = universe.render_as_text();
  universe.tick();

  const elapsed = performance.now() - start;
  const average = movingAverage.add(elapsed);

  elapsedSpan.textContent = elapsed.toFixed(2);
  averageElapsedSpan.textContent = average.toFixed(2);

  requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
