struct Solution;

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        Self::dfs(&graph, 0, &mut vec![0], &mut result);
        result
    }

    fn dfs(graph: &[Vec<i32>], now: usize, stack: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if now == graph.len() - 1 {
            result.push(stack.clone());
            return;
        }

        for i in 0..graph[now].len() {
            stack.push(graph[now][i]);
            Self::dfs(graph, graph[now][i] as usize, stack, result);
            stack.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::all_paths_source_target(vec![vec![1, 2], vec![3], vec![3], vec![]]),
            vec![vec![0, 1, 3], vec![0, 2, 3]]
        );
        assert_eq!(
            Solution::all_paths_source_target(vec![
                vec![4, 3, 1],
                vec![3, 2, 4],
                vec![3],
                vec![4],
                vec![]
            ]),
            vec![
                vec![0, 4],
                vec![0, 3, 4],
                vec![0, 1, 3, 4],
                vec![0, 1, 2, 3, 4],
                vec![0, 1, 4]
            ]
        );
    }
}
