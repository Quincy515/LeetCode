class Solution:
    def __init__(self):
        self.result = []

    def solveNQueens(self, n: int) -> List[List[str]]:
        board = [["."] * n for i in range(n)]
        self.backtrack(0, board, n)
        return self.result

    def backtrack(self, row, board, n):
        if row == n:
            snapshot = []
            for i in range(n):
                snapshot.append("".join(board[i]))
            self.result.append(snapshot)
        for col in range(n):
            if self.isOK(board, n, row, col):
                board[row][col] = "Q"
                self.backtrack(row+1, board, n)
                board[row][col] = "."
        return

    def isOK(self, board, n, row, col):
        # 检查列判断
        for i in range(n):
            if board[i][col] == "Q":
                return False
        # 检查⼜对⻆线
        i = row - 1
        j = col + 1
        while i >= 0 and j < n:
            if board[i][j] == "Q":
                return False
            i -= 1
            j += 1
        # 检查左对⻆线
        i = row - 1
        j = col - 1
        while i >= 0 and j >= 0:
            if board[i][j] == "Q":
                return False
            i -= 1
            j -= 1
        return True
