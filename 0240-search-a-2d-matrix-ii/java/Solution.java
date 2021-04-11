class Solution {
  // Time: O(m+n), Space: O(1)
  public boolean searchMatrix(int[][] matrix, int target) {
    if (matrix == null || matrix.length == 0 ||
      matrix[0] == null || matrix[0].length == 0)
      return new int[]{-1, -1};

    int m = matrix.length, n = matrix[0].length;
    int i = 0, j = n - 1;
    while (i < m && j >= 0) {
      if (target < matrix[i][j]) --j;
      else if (target > matrix[i][j]) ++i;
      else return new int[]{i, j};
    }
    return new int[]{-1, -1};
  }
}