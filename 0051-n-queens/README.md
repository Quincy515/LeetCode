<p><strong>n&nbsp;皇后问题</strong> 研究的是如何将 <code>n</code>&nbsp;个皇后放置在 <code>n×n</code> 的棋盘上，并且使皇后彼此之间不能相互攻击。</p>

<p>给你一个整数 <code>n</code> ，返回所有不同的&nbsp;<strong>n<em>&nbsp;</em>皇后问题</strong> 的解决方案。</p>

<div class="original__bRMd">
<div>
<p>每一种解法包含一个不同的&nbsp;<strong>n 皇后问题</strong> 的棋子放置方案，该方案中 <code>'Q'</code> 和 <code>'.'</code> 分别代表了皇后和空位。</p>

<p>&nbsp;</p>

<p><strong>示例 1：</strong></p>
<img alt="" src="https://assets.leetcode.com/uploads/2020/11/13/queens.jpg" style="width: 600px; height: 268px;" />
<pre>
<strong>输入：</strong>n = 4
<strong>输出：</strong>[[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
<strong>解释：</strong>如上图所示，4 皇后问题存在两个不同的解法。
</pre>

<p><strong>示例 2：</strong></p>

<pre>
<strong>输入：</strong>n = 1
<strong>输出：</strong>[["Q"]]
</pre>

<p>&nbsp;</p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>1 &lt;= n &lt;= 9</code></li>
</ul>
</div>
</div>

## 题解

### Rust

```rust
impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = vec![];
        let mut board: Vec<Vec<char>> = vec![vec!['.'; n as usize]; n as usize];
        Self::backtrack(0, n as usize, &mut board, &mut result);
        result
    }

    // row: 阶段
    // board: 路径，记录已经做出的决策
    // 可选列表：通过 board 推导出来，没有显示记录
    fn backtrack(row: usize, n: usize, board: &mut [Vec<char>], result: &mut Vec<Vec<String>>) {
        // 结束条件，得到可行解
        if row == n {
            let mut snapshot = vec![];
            for row in board {
                let mut line = String::new();
                for c in row {
                    line.push(*c);
                }
                snapshot.push(line);
            }
            result.push(snapshot);
            // result.push(board.iter().map(|vec| vec.iter().collect()).collect());
            return;
        }
        for col in 0..n {
            // 每一行都有 n 种放法
            if Self::is_ok(board, n, row, col) {
                // 可选列表
                board[row][col] = 'Q'; // 做选择，第 row 行的棋子放在 col 列
                Self::backtrack(row + 1, n, board, result); // 考察下一行
                board[row][col] = '.'; // 恢复选择
            }
        }
    }

    // 判断 row 行 column 列放置是否合适
    fn is_ok(board: &[Vec<char>], n: usize, row: usize, col: usize) -> bool {
        // 检查是否有冲突
        for i in 0..n {
            if board[i][col] == 'Q' {
                return false;
            }
        }
        // 检查右上对角线是否有冲突
        let (mut i, mut j) = (row as i32 - 1, col as i32 + 1);
        while i >= 0 && j < n as i32 {
            if board[i as usize][j as usize] == 'Q' {
                return false;
            }
            i -= 1;
            j += 1;
        }
        // 检查左上对角线是否有冲突
        let (mut i, mut j) = (row as i32 - 1, col as i32 - 1);
        while i >= 0 && j >= 0 {
            if board[i as usize][j as usize] == 'Q' {
                return false;
            }
            i -= 1;
            j -= 1;
        }
        true
    }
}
```

### Go

```go
package main

var result [][]string

func solveNQueens(n int) [][]string {
	result = make([][]string, 0)
	board := make([][]byte, n)
	for i := 0; i < n; i++ {
		board[i] = make([]byte, n)
		for j := 0; j < n; j++ {
			board[i][j] = '.'
		}
	}
	backtrack(0, board, n)
	return result
}

//row: 阶段
//board: 路径，记录已经做出的决策
//可选列表： 通过 board 推导出来， 没有显示记录
func backtrack(row int, board [][]byte, n int) {
	//结束条件,得到可行解
	if row == n {
		snapshot := make([]string, len(board))
		for i := 0; i < n; i++ {
			snapshot[i] = string(board[i])
		}
		result = append(result, snapshot)
		return
	}
	for col := 0; col < n; col++ { //每一行都有 n 种放法
		if isOk(board, n, row, col) { //可选列表
			board[row][col] = 'Q'      //做选择，第 row 行的棋子放在 col 列
			backtrack(row+1, board, n) //考察下一行
			board[row][col] = '.'      //恢复选择
		}
	}
}

//判断 row 行 column 列放置是否合适
func isOk(board [][]byte, n, row, col int) bool { //检查是否有冲突
	for i := 0; i < n; i++ {
		if board[i][col] == 'Q' {
			return false
		}
	}
	//检查右上对角线是否有冲突
	i := row - 1
	j := col + 1
	for i >= 0 && j < n {
		if board[i][j] == 'Q' {
			return false
		}
		i--
		j++
	}

	//检查左上对角线是否有冲突
	i = row - 1
	j = col - 1
	for i >= 0 && j >= 0 {
		if board[i][j] == 'Q' {
			return false
		}
		i--
		j--
	}
	return true
}
```

### JavaScript

```js
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
```

### Python

```py
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

```
