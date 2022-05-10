impl Solution {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let n = arr.len();
        let mut visited: Vec<bool> = vec![false; n];
        let mut reached = false;
        Solution::dfs(&arr, start, &mut visited, &mut reached);
        reached
    }

    fn dfs(arr: &[i32], curi: i32, visited: &mut [bool], reached: &mut bool) {
        if *reached {
            return;
        }
        if arr[curi as usize] == 0 {
            *reached = true;
            return;
        }
        visited[curi as usize] = true;
        let move2left = curi - arr[curi as usize];
        if move2left >= 0 && move2left < arr.len() as i32 && !visited[move2left as usize] {
            Solution::dfs(arr, move2left, visited, reached)
        }
        let move2right = curi + arr[curi as usize];
        if move2right >= 0 && move2right < arr.len() as i32 && !visited[move2right as usize] {
            Solution::dfs(arr, move2right, visited, reached)
        }
    }
}

impl Solution2 {
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut visited = vec![false; arr.len()];
        Self::visit(&arr, start, &mut visited)
    }

    fn visit(arr: &[i32], start: i32, visited: &mut Vec<bool>) -> bool {
        if start < 0 || start >= arr.len() as i32 {
            return false;
        }
        if arr[start as usize] == 0 {
            return true;
        }
        if visited[start as usize] {
            return false;
        }
        visited[start as usize] = true;
        Self::visit(arr, start + arr[start as usize], visited)
            || Self::visit(arr, start - arr[start as usize], visited)
    }
}
struct Solution;
struct Solution2;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert!(Solution2::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 5));
        assert!(Solution2::can_reach(vec![4, 2, 3, 0, 3, 1, 2], 0));
        assert!(!Solution::can_reach(vec![3, 0, 2, 1, 2], 2));
        assert!(Solution::can_reach(vec![4, 4, 1, 3, 0, 3], 2));
    }
}
