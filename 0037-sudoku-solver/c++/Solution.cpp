class Solution {
public:
    vector<vector<bool>> rows;
    vector<vector<bool>> cols;
    vector<vector<vector<bool>>> blocks;
    bool solved = false;
    void solveSudoku(vector<vector<char>>& board) {
        rows.assign(9, vector<bool>(10, false));
        cols.assign(9, vector<bool>(10, false));
        blocks.assign(3, vector<vector<bool>>(3, vector<bool>(10, false)));
        for (int i = 0; i < 9; ++i) {
            for (int j = 0; j < 9; ++j) {
                if (board[i][j] != '.') {
                    int num = board[i][j] - '0';
                    rows[i][num] = true;
                    cols[j][num] = true;
                    blocks[i/3][j/3][num] = true;
                }
            }
        }
        backtrack(0, 0, board);
    }
    void backtrack(int row, int col, vector<vector<char>>& board) {
        if (row == 9) {
            solved = true;
            return;
        }
        if (board[row][col] != '.') {
            int nextRow = row;
            int nextCol = col + 1;
            if (col == 8) {
                nextRow = row + 1;
                nextCol = 0;
            }
            backtrack(nextRow, nextCol, board);
            if (solved) return;
        } else {
            for (int num = 1; num <= 9; ++num) {
                if (!rows[row][num] && !cols[col][num]
                    && !blocks[row/3][col/3][num]) {
                    board[row][col] = to_string(num)[0];
                    rows[row][num] = true;
                    cols[col][num] = true;
                    blocks[row/3][col/3][num] = true;
                    int nextRow = row;
                    int nextCol = col + 1;
                    if (col == 8) {
                        nextRow = row + 1;
                        nextCol = 0;
                    }
                    backtrack(nextRow, nextCol, board);
                    if (solved) return;
                    board[row][col] = '.';
                    rows[row][num] = false;
                    cols[col][num] = false;
                    blocks[row/3][col/3][num] = false;
                }
            }
        }
    }
};