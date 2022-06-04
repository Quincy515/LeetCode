struct Solution;

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // 记录所有路径
        let mut result = vec![];
        traverse(&graph, 0, &mut vec![], &mut result);
        result
    }

    pub fn all_paths_source_target_dfs(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
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

// path: 维护递归过程中经过的路径
fn traverse(graph: &[Vec<i32>], s: i32, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
    // 添加节点 s 到路径
    path.push(s);

    let n = graph.len() as i32;
    if s == n - 1 {
        // 到达终点
        result.push(path.clone());
        // 可以在这直接 return，但要 removeLast 正确维护 path
        // path.pop();
        // return;
        // 不 return 也可以，因为图中不包含环，不会出现无限递归
    }

    // 递归每个相邻节点
    for &v in graph[s as usize].iter() {
        traverse(graph, v, path, result);
    }

    // 从路径移出节点 s
    path.pop();
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
