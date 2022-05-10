from typing import List


class Solution:
    def __init__(self):
        self.solved = False
        self.rows = [[None] * 10 for i in range(9)]
        self.cols = [[None] * 10 for i in range(9)]
        self.blocks = [[[None] * 10 for i in range(3)]for i in range(3)]

    def solveSudoku(self, board: List[List[str]]) -> None:
        """
        Do not return anything, modify board in-place instead.
        """
        for i in range(9):
            for j in range(9):
                if board[i][j] != '.':
                    num = int(board[i][j])
                    self.rows[i][num] = True
                    self.cols[j][num] = True
                    self.blocks[i//3][j//3][num] = True
        self.backtrack(0, 0, board)
        return self.solved

    def backtrack(self, row, col, board):
        if row == 9:
            self.solved = True
            return
        if board[row][col] != ".":
            nextRow = row
            nextCol = col + 1
            if col == 8:
                nextRow = row + 1
                nextCol = 0
            self.backtrack(nextRow, nextCol, board)
            if self.solved:
                return
        else:
            for num in range(1, 10):
                if not self.rows[row][num] and not self.cols[col][num] and not self.blocks[row//3][col//3][num]:
                    board[row][col] = str(num)
                    self.rows[row][num] = True
                    self.cols[col][num] = True
                    self.blocks[row//3][col//3][num] = True
                    nextRow = row
                    nextCol = col + 1
                    if col == 8:
                        nextRow = row + 1
                        nextCol = 0
                    self.backtrack(nextRow, nextCol, board)
                    if self.solved:
                        return
                    board[row][col] = "."
                    self.rows[row][num] = False
                    self.cols[col][num] = False
                    self.blocks[row//3][col//3][num] = False
