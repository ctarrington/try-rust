import * as wasm from "population";

const mean = (count) => {
  let sum = 0;

  for (let ctr = 0; ctr < count; ctr++) {
    sum += Math.random();
  }

  return sum / count;
};

const rustMeanSpan = document.getElementById('rust-mean');
const rustElapsedSpan = document.getElementById('rust-elapsed');
const jsMeanSpan = document.getElementById('js-mean');
const jsElapsedSpan = document.getElementById('js-elapsed');

const count = 1000*1000*50;

const rustStart = performance.now();
const rustMean = wasm.mean(count);
const rustElapsed = performance.now() - rustStart;

const jsStart = performance.now();
const jsMean = mean(count);
const jsElapsed = performance.now() - jsStart;

rustMeanSpan.textContent = rustMean.toFixed(5);
rustElapsedSpan.textContent = rustElapsed.toFixed(5);
jsMeanSpan.textContent = jsMean.toFixed(5);
jsElapsedSpan.textContent = jsElapsed.toFixed(5);
