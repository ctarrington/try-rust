const createJSUniverse = (width, height) => {
  const getIndex = (row, col) => row * width + col;

  const getValue = (cells, row, col) => {
    let adjustedRow = row;
    let adjustedCol = col;

    if (row < 0) {
      adjustedRow = height - 1;
    }

    if (row >= height) {
      adjustedRow = 0;
    }

    if (col < 0) {
      adjustedCol = width - 1;
    }

    if (col >= width) {
      adjustedCol = 0;
    }

    return cells[getIndex(adjustedRow, adjustedCol)];
  };

  let current = 0;
  const buffers = [new ArrayBuffer(width * height), new ArrayBuffer(width * height)];
  const cells_list = [new Uint8Array(buffers[0]), new Uint8Array(buffers[1])];

  for (let index = 0; index < cells_list[0].length; index++) {
    cells_list[0][index] = index % 2 == 0 || index % 7 == 0 ? 1 : 0;
  }

  const countNeighbors = (cells, row_ctr, col_ctr) => {
    let count = 0;
    for (let ri = row_ctr - 1; ri <= row_ctr + 1; ri++) {
      for (let ci = col_ctr - 1; ci <= col_ctr + 1; ci++) {
        if (ri != row_ctr || ci != col_ctr) {
          count += getValue(cells, ri, ci);
        }
      }
    }

    return count;
  }

  const tick = () => {
    const oldCells = cells_list[current];
    current = (current + 1) % 2;
    const cells = cells_list[current];

    for (let row_ctr = 0; row_ctr < height; row_ctr++) {
      for (let col_ctr = 0; col_ctr < width; col_ctr++) {
        const index = getIndex(row_ctr, col_ctr);
        cells[index] = oldCells[index]; // default to existing value

        const neighborCount = countNeighbors(oldCells, row_ctr, col_ctr);
        if (cells[index] === 1) {
          if (neighborCount < 2 || neighborCount > 3) {
            cells[index] = 0;
          }
        } else {
          if (neighborCount === 3) {
            cells[index] = 1;
          }
        }
      }
    }
  };

  return {
    cells: () => cells_list[current],
    tick,
  };
};

export {createJSUniverse};
