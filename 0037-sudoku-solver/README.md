<p>编写一个程序，通过填充空格来解决数独问题。</p>

<p>数独的解法需<strong> 遵循如下规则</strong>：</p>

<ol>
	<li>数字 <code>1-9</code> 在每一行只能出现一次。</li>
	<li>数字 <code>1-9</code> 在每一列只能出现一次。</li>
	<li>数字 <code>1-9</code> 在每一个以粗实线分隔的 <code>3x3</code> 宫内只能出现一次。（请参考示例图）</li>
</ol>

<p>数独部分空格内已填入了数字，空白格用 <code>'.'</code> 表示。</p>

<p> </p>

<div class="top-view__1vxA">
<div class="original__bRMd">
<div>
<p><strong>示例：</strong></p>
<img src="https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2021/04/12/250px-sudoku-by-l2g-20050714svg.png" style="height:250px; width:250px" />
<pre>
<strong>输入：</strong>board = [["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]
<strong>输出：</strong>[["5","3","4","6","7","8","9","1","2"],["6","7","2","1","9","5","3","4","8"],["1","9","8","3","4","2","5","6","7"],["8","5","9","7","6","1","4","2","3"],["4","2","6","8","5","3","7","9","1"],["7","1","3","9","2","4","8","5","6"],["9","6","1","5","3","7","2","8","4"],["2","8","7","4","1","9","6","3","5"],["3","4","5","2","8","6","1","7","9"]]
<strong>解释：</strong>输入的数独如上图所示，唯一有效的解决方案如下所示：

<img src=" https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2021/04/12/250px-sudoku-by-l2g-20050714_solutionsvg.png" style="height:250px; width:250px" />
</pre>

<p> </p>

<p><strong>提示：</strong></p>

<ul>
	<li><code>board.length == 9</code></li>
	<li><code>board[i].length == 9</code></li>
	<li><code>board[i][j]</code> 是一位数字或者 <code>'.'</code></li>
	<li>题目数据 <strong>保证</strong> 输入数独仅有一个解</li>
</ul>
</div>
</div>
</div>
<div><div>Related Topics</div><div><li>数组</li><li>回溯</li><li>矩阵</li></div></div>

## 题解

### Rust 未完成
```rust
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut rows = vec![vec![false; 10]; 9];
        let mut cols = vec![vec![false; 10]; 9];
        let mut blocks = vec![vec![vec![false; 10]; 3]; 3];
        let mut solved = false;
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] != '.' {
                    let num = (board[i][j] as u8 - b'0') as usize;
                    // let num = board[i][j].to_digit(10).unwrap() as usize -1;
                    rows[i][num] = true;
                    cols[j][num] = true;
                    blocks[i / 3][j / 3][num] = true;
                }
            }
        }
        Self::backtrack(0, 0, board, &mut solved, &mut rows, &mut cols, &mut blocks);
    }

    fn backtrack(
        row: i32,
        col: i32,
        board: &mut Vec<Vec<char>>,
        solved: &mut bool,
        rows: &mut Vec<Vec<bool>>,
        cols: &mut Vec<Vec<bool>>,
        blocks: &mut Vec<Vec<Vec<bool>>>,
    ) {
        if row == 9 {
            *solved = true;
            return;
        }
        if board[row as usize][col as usize] != '.' {
            let mut next_row = row;
            let mut next_col = col + 1;
            if col == 8 {
                next_row = row + 1;
                next_col = 0;
            }
            Self::backtrack(next_row, next_col, board, solved, rows, cols, blocks);
            if *solved {
                return;
            }
        } else {
            for num in 1..=9 {
                if !rows[row as usize][num as usize]
                    && !cols[col as usize][num as usize]
                    && !blocks[row as usize / 3][col as usize / 3][num as usize]
                {
                    // board[row as usize][col as usize] = (num + b'0') as char; // 数字转换为字符
                    board[row as usize][col as usize] = std::char::from_digit(num, 10).unwrap(); // 数字转换为字符
                    rows[row as usize][num as usize] = true;
                    cols[col as usize][num as usize] = true;
                    blocks[row as usize / 3][col as usize / 3][num as usize] = true;
                    let mut next_row = row;
                    let mut next_col = col + 1;
                    if col == 8 {
                        next_row = row + 1;
                        next_col = 0;
                    }
                    Self::backtrack(next_row, next_col, board, solved, rows, cols, blocks);
                    if *solved {
                        return;
                    }
                    board[row as usize][col as usize] = '.';
                    rows[row as usize][num as usize] = false;
                    cols[row as usize][num as usize] = false;
                    blocks[row as usize / 3][col as usize / 3][num as usize] = false;
                }
            }
        }
    }
}
```

### Go
```go
package main

var rows [][]bool
var cols [][]bool
var blocks [][][]bool
var solved bool

func solveSudoku(board [][]byte) {
	initParam()
	for i := 0; i < 9; i++ {
		for j := 0; j < 9; j++ {
			if board[i][j] != '.' {
				num := board[i][j] - '0'
				rows[i][num] = true
				cols[j][num] = true
				blocks[i/3][j/3][num] = true
			}
		}
	}
	backtrack(0, 0, board)
}
func initParam() {
	rows = make([][]bool, 9)
	for i := 0; i < len(rows); i++ {
		rows[i] = make([]bool, 10)
	}
	cols = make([][]bool, 9)
	for i := 0; i < len(cols); i++ {
		cols[i] = make([]bool, 10)
	}
	blocks = make([][][]bool, 3)
	for i := 0; i < len(blocks); i++ {
		blocks[i] = make([][]bool, 3)
		for j := 0; j < len(blocks); j++ {
			blocks[i][j] = make([]bool, 10)
		}
	}
	solved = false
}

func backtrack(row, col int, board [][]byte) {
	if row == 9 {
		solved = true
		return
	}
	if board[row][col] != '.' {
		nextRow := row
		nextCol := col + 1
		if col == 8 {
			nextRow = row + 1
			nextCol = 0
		}
		backtrack(nextRow, nextCol, board)
		if solved {
			return
		}
	} else {
		for num := 1; num <= 9; num++ {
			if !rows[row][num] && !cols[col][num] && !blocks[row/3][col/3][num] {
				board[row][col] = byte(num + '0')
				rows[row][num] = true
				cols[col][num] = true
				blocks[row/3][col/3][num] = true
				nextRow := row
				nextCol := col + 1
				if col == 8 {
					nextRow = row + 1
					nextCol = 0
				}
				backtrack(nextRow, nextCol, board)
				if solved {
					return
				}
				board[row][col] = '.'
				rows[row][num] = false
				cols[col][num] = false
				blocks[row/3][col/3][num] = false
			}
		}
	}
}
```
### Python
```python
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

```

### JavaScript
```js
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

```

### C++
```c++
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
```