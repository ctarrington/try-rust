import * as wasm from "population";

const random_mean = (count) => {
  let sum = 0;

  for (let ctr = 0; ctr < count; ctr++) {
    sum += Math.random();
  }

  return sum / count;
};

const random_lcg = (count) => {
  let current = 123456789.0;
  const a = 8121.0;
  const c = 28411.0;
  const m = 134456.0;
  const lcg_next = () => {
        current = (a * current + c) % m;
        return current / m;
  };

  let sum = 0;

  for (let ctr = 0; ctr < count; ctr++) {
    sum += lcg_next();
  }

  return sum / count;
}

const rustRandomMeanSpan = document.getElementById('rust-random-mean');
const rustRandomElapsedSpan = document.getElementById('rust-random-elapsed');

const jsRandomMeanSpan = document.getElementById('js-random-mean');
const jsRandomElapsedSpan = document.getElementById('js-random-elapsed');

const rustLcgMeanSpan = document.getElementById('rust-lcg-mean');
const rustLcgElapsedSpan = document.getElementById('rust-lcg-elapsed');

const jsLcgMeanSpan = document.getElementById('js-lcg-mean');
const jsLcgElapsedSpan = document.getElementById('js-lcg-elapsed');

const countDiv = document.getElementById('count');
const runButton = document.getElementById('run-button');

let count = 1000*1000;

const tick = () => {
  const rustRandomStart = performance.now();
  const rustRandomMean = wasm.random_mean(count);
  const rustRandomElapsed = performance.now() - rustRandomStart;

  const jsRandomStart = performance.now();
  const jsRandomMean = random_mean(count);
  const jsRandomElapsed = performance.now() - jsRandomStart;

  const rustLcgStart = performance.now();
  const rustLcgMean = wasm.lcg_mean(count);
  const rustLcgElapsed = performance.now() - rustLcgStart;

  const jsLcgStart = performance.now();
  const jsLcgMean = random_lcg(count);
  const jsLcgElapsed = performance.now() - jsLcgStart;

  rustRandomMeanSpan.textContent = rustRandomMean.toFixed(5);
  rustRandomElapsedSpan.textContent = rustRandomElapsed.toFixed(2);

  jsRandomMeanSpan.textContent = jsRandomMean.toFixed(5);
  jsRandomElapsedSpan.textContent = jsRandomElapsed.toFixed(2);

  rustLcgMeanSpan.textContent = rustLcgMean.toFixed(5);
  rustLcgElapsedSpan.textContent = rustLcgElapsed.toFixed(2);

  jsLcgMeanSpan.textContent = jsLcgMean.toFixed(5);
  jsLcgElapsedSpan.textContent = jsLcgElapsed.toFixed(2);

  countDiv.textContent = count.toLocaleString();

  count += 1000*1000;

};


runButton.addEventListener('click', tick, false);
tick();
