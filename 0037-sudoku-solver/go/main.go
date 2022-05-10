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
