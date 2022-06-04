struct Solution;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = num_courses as usize;
        let graph = build_graph(num_courses, prerequisites);
        let mut cycle = false;
        let mut postorder = vec![];
        let (mut path, mut visited) = (vec![false; n], vec![false; n]);
        // 遍历图
        for i in 0..num_courses {
            traverse(
                &graph,
                i as usize,
                &mut path,
                &mut visited,
                &mut postorder,
                &mut cycle,
            )
        }

        // 有环图无法进行拓扑排序
        if cycle {
            return vec![];
        }

        // 逆后序遍历结果即为拓扑排序结果
        postorder.into_iter().rev().collect()
    }
}

// 使用邻接表构建图
fn build_graph(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<Vec<usize>> {
    let mut graph = vec![Vec::new(); num_courses as usize];
    for edge in prerequisites.iter() {
        let (from, to) = (edge[1] as usize, edge[0] as usize);
        graph[from].push(to);
    }
    graph
}

// 遍历图
fn traverse(
    graph: &[Vec<usize>],
    v: usize,
    path: &mut Vec<bool>,
    visited: &mut Vec<bool>,
    postorder: &mut Vec<i32>,
    cycle: &mut bool,
) {
    if path[v] {
        *cycle = true;
    }
    if visited[v] || *cycle {
        return;
    }
    visited[v] = true;
    path[v] = true;
    for &w in graph[v].iter() {
        traverse(graph, w, path, visited, postorder, cycle);
    }
    postorder.push(v as i32);
    path[v] = false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::find_order(2, vec![vec![1, 0]]), vec![0, 1]);
        assert_eq!(
            Solution::find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1]]),
            vec![0, 2, 1, 3]
        );
        assert_eq!(Solution::find_order(1, vec![vec![]]), vec![0]);
        assert_eq!(Solution::find_order(2, vec![vec![]]), vec![1, 0]);
    }
}
