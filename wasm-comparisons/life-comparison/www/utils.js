
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

export {createMovingAverage};
