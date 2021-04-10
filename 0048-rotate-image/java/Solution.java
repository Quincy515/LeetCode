class Solution {
    // Time: O(n^2), Space: O(1)
    public void rotate(int[][] matrix) {
        // 1. 检查数组边界，数组为空，或长度为0就直接返回
        if (matrix == null || matrix.length == 0 || matrix[0] == null || matrix[0].length == 0)
            return;
        int n = matrix.length;
        // 2. 沿主对角线交换数字
        for (int i = 0; i < n; i++) {
            for (int j = i; j < n; j++) {
                int tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }
        // 2. 水平翻转二维数组
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n / 2; j++) {
                int tmp = matrix[i][j];
                matrix[i][j] = matrix[i][n - 1 - j];
                matrix[i][n - 1 - j] = tmp;
            }
        }
    }
}