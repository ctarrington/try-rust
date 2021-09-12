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

  let buffer = new ArrayBuffer(width * height);
  let cells = new Uint8Array(buffer);

  for (let index = 0; index < cells.length; index++) {
    cells[index] = index % 2 == 0 || index % 7 == 0 ? 1 : 0;
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
    const oldCells = cells;
    buffer = new ArrayBuffer(width * height);
    cells = new Uint8Array(buffer);

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
    cells: () => cells,
    tick,
  };
};

export {createJSUniverse};
