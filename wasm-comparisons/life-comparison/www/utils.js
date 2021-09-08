
// use a circular buffer of the passed size to calculate a moving or windowed average
const createMovingAverage = (thesize) => {
  const size = thesize;
  let currentPosition = 0;
  let sum = 0;
  let minValue = Number.MAX_VALUE;
  let maxValue = Number.MIN_VALUE;
  const values = [];

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
  
  return {add, min, max};
};

export {createMovingAverage};
