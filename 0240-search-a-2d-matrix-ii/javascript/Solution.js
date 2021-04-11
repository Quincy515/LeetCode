/**
 * @param {number[][]} matrix
 * @param {number} target
 * @return {boolean}
 */
var searchMatrix = function (matrix, target) {
  if (
    !matrix ||
    !matrix.length ||
    !matrix[0] ||
    !matrix[0].length
  ) {
    return false;
  }

  const m = matrix.length, n = matrix[0].length;
  let i = 0, j = n - 1;
  while (i < m && j >= 0) {
    if (target < matrix[i][j]) {
      --j;
    } else if (target > matrix[i][j]) {
      ++i;
    } else {
      return true;
    }
  }
  return false;
};