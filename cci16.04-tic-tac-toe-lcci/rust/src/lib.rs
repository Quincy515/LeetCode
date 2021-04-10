struct Solution {}

impl Solution {
  pub fn tictactoe(board: Vec<String>) -> String {
    let n = board.len();
    let (mut rowx, mut colx, mut rowo, mut colo) = (vec![0; n], vec![0; n], vec![0; n], vec![0; n]);
    let (mut leftx, mut rightx, mut lefto, mut righto, mut flag) = (0, 0, 0, 0, 0);
    for i in 0..n {
      let temp: Vec<char> = board[i].chars().collect();
      for j in 0..n {
        if temp[j] == 'X' {
          rowx[i] += 1;
          colx[j] += 1;
          if i == j {
            leftx += 1;
          }
          if i + j + 1 == n {
            rightx += 1;
          }
        } else if temp[j] == 'O' {
          rowo[i] += 1;
          colo[j] += 1;
          if i == j {
            lefto += 1;
          }
          if i + j + 1 == n {
            righto += 1;
          }
        } else {
          flag = 1;
        }
      }
    }
    if leftx == n || rightx == n {
      return "X".to_string();
    }
    if lefto == n || righto == n {
      return "O".to_string();
    }
    for i in 0..n {
      if rowx[i] == n || colx[i] == n {
        return "X".to_string();
      }
      if rowo[i] == n || colo[i] == n {
        return "O".to_string();
      }
    }
    if flag == 1 {
      return "Pending".to_string();
    }
    return "Draw".to_string();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    assert_eq!("X", Solution::tictactoe(vec!["O X".to_string(), " XO".to_string(), "X O".to_string()]));
    assert_eq!("Draw", Solution::tictactoe(vec!["OOX".to_string(), "XXO".to_string(), "OXO".to_string()]));
    assert_eq!("Pending", Solution::tictactoe(vec!["OOX".to_string(), "XXO".to_string(), "OX ".to_string()]));
  }
}
