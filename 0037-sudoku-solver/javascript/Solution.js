/**
 * @param {character[][]} board
 * @return {void} Do not return anything, modify board in-place instead.
 */
var solveSudoku = function (board) {
  let rows = Array(9)
    .fill()
    .map(() => Array(10).fill(false));
  let cols = Array(9)
    .fill()
    .map(() => Array(10).fill(false));
  let blocks = Array(3)
    .fill()
    .map(() =>
      Array(3)
        .fill()
        .map(() => Array(10).fill(false))
    );
  let solved = false;
  for (let i = 0; i < 9; ++i) {
    for (let j = 0; j < 9; ++j) {
      if (board[i][j] != ".") {
        let num = board[i][j].charCodeAt(0) - "0".charCodeAt(0);
        let blocksRow = Math.floor(i / 3);
        let blocksCol = Math.floor(j / 3);
        rows[i][num] = true;
        cols[j][num] = true;
        blocks[blocksRow][blocksCol][num] = true;
      }
    }
  }
  let backtrack = (row, col) => {
    if (row == 9) {
      solved = true;
      return;
    }
    if (board[row][col] != ".") {
      let nextRow = row;
      let nextCol = col + 1;
      if (col == 8) {
        nextRow = row + 1;
        nextCol = 0;
      }
      backtrack(nextRow, nextCol);
      if (solved) {
        return;
      }
    } else {
      for (let num = 1; num <= 9; ++num) {
        let blocksRow = Math.floor(row / 3);
        let blocksCol = Math.floor(col / 3);
        if (
          !rows[row][num] &&
          !cols[col][num] &&
          !blocks[blocksRow][blocksCol][num]
        ) {
          board[row][col] = `${num}`;
          rows[row][num] = true;
          cols[col][num] = true;
          blocks[blocksRow][blocksCol][num] = true;
          let nextRow = row;
          let nextCol = col + 1;
          if (col == 8) {
            nextRow = row + 1;
            nextCol = 0;
          }
          backtrack(nextRow, nextCol);
          if (solved) {
            return;
          }
          board[row][col] = ".";
          rows[row][num] = false;
          cols[col][num] = false;
          blocks[blocksRow][blocksCol][num] = false;
        }
      }
    }
  };
  backtrack(0, 0);
};
