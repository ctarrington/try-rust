import { Universe, Cell } from 'life-comparison';
import { memory } from "life-comparison/life_comparison_bg";

// use a circular buffer of the passed size to calculate a moving or windowed average
const createMovingAverage = (thesize) => {
  const size = thesize;
  let currentPosition = 0;
  let sum = 0;
  let minValue = Number.MAX_VALUE;
  let maxValue = Number.MIN_VALUE;
  let values = [];

  const add = (value) => {

    // out with the old, in with the new
    if (currentPosition < values.length) {
      sum -= values[currentPosition];
    }
    values[currentPosition] = value;
    sum += value;

    minValue = Math.min(minValue, value);
    maxValue = Math.max(maxValue, value);

    // increment and circle around if at the end 
    currentPosition++;
    currentPosition = currentPosition % size;

    return sum/values.length;
  };

  const min = () => { return minValue; };
  const max = () => { return maxValue; };
  const clear = () => {
    currentPosition = 0;
    sum = 0;
    minValue = Number.MAX_VALUE;
    maxValue = Number.MIN_VALUE;
    values = [];
  };
    
  
  return {add, clear, min, max};
};


const CELL_SIZE = 5;
const GRID_COLOR = '#CCCCCC';
const DEAD_COLOR = '#FFFFFF';
const ALIVE_COLOR = '#000000';

const getIndex = (row, column, width, height) => {
    return row * width + column;
};

const drawGrid = (ctx, width, height) => {

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

const drawCellsFromReference = (ctx, universe, width, height) => {

  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();

  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col, width, height);

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

const formatCellsFromReference = (universe, width, height) => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  let results = '';
  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const idx = getIndex(row, col, width, height);
      results += cells[idx] === Cell.Dead ? '◻' : '◼';
    }
    results += '\n';
  }

  return results;
};

export {createMovingAverage, drawCellsFromReference, drawGrid, formatCellsFromReference};
