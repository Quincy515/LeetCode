/**
 * @param {number} n
 * @return {string[][]}
 */
var solveNQueens = function (n) {
  let result = [];
  let board = Array(n)
    .fill()
    .map(() => Array(n).fill("."));
  let backtrack = (row) => {
    if (row == n) {
      result.push(board.map((r) => r.join("")));
      return;
    }
    for (let col = 0; col < n; ++col) {
      if (isOk(board, n, row, col)) {
        board[row][col] = "Q";
        backtrack(row + 1);
        board[row][col] = ".";
      }
    }
  };
  backtrack(0);
  return result;
};
let isOk = (board, n, row, col) => {
  for (let i = 0; i < n; i++) {
    if (board[i][col] == "Q") {
      return false;
    }
  }
  let i = row - 1;
  let j = col + 1;
  while (i >= 0 && j < n) {
    if (board[i][j] == "Q") {
      return false;
    }
    i--;
    j++;
  }
  i = row - 1;
  j = col - 1;
  while (i >= 0 && j >= 0) {
    if (board[i][j] == "Q") {
      return false;
    }
    i--;
    j--;
  }
  return true;
};
