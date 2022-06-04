struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let graph = build_graph(num_courses, prerequisites);
        let mut result = false;
        for i in 0..num_courses {
            traverse(
                &graph,
                i as usize,
                &mut vec![],
                &mut vec![false; num_courses as usize],
                &mut result,
            );
        }
        !result
    }
    pub fn can_finis_dfsh(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut adjs = vec![vec![]; num_courses as usize];
        let mut indegrees = vec![0; num_courses as usize];
        for i in 0..prerequisites.len() {
            adjs[prerequisites[i][1] as usize].push(prerequisites[i][0]);
            indegrees[prerequisites[i][0] as usize] += 1;
        }
        let mut zero_in_degrees = vec![];
        for (i, _) in indegrees.iter().enumerate() {
            if indegrees[i] == 0 {
                zero_in_degrees.push(i);
            }
        }
        let mut zero_in_degrees_count = 0;
        while !zero_in_degrees.is_empty() {
            let coursei = zero_in_degrees.pop().unwrap();
            zero_in_degrees_count += 1;
            for coursej in &adjs[coursei] {
                indegrees[*coursej as usize] -= 1;
                if indegrees[*coursej as usize] == 0 {
                    zero_in_degrees.push(*coursej as usize);
                }
            }
        }

        zero_in_degrees_count == num_courses
    }
}

// 构建具有依赖关系的有向图
fn build_graph(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<Vec<usize>> {
    // 图中共有 numCourses 个节点
    let mut graph = vec![Vec::new(); num_courses as usize];

    for edge in prerequisites.iter() {
        let (from, to) = (edge[1] as usize, edge[0] as usize);
        // 添加一条从 from 指向 to 的有向边
        // 边的方向是「被依赖」关系，即修完课程 from 才能修课程 to
        graph[from].push(to);
    }

    graph
}
fn traverse(
    graph: &[Vec<usize>],
    v: usize,
    path: &mut Vec<usize>,
    visited: &mut Vec<bool>,
    result: &mut bool,
) {
    if path.iter().any(|x| x == &v) {
        *result = true;
    }
    if visited[v] || *result {
        return;
    }
    visited[v] = true;
    path.push(v);

    for &w in graph[v].iter() {
        traverse(graph, w, path, visited, result);
    }
    path.pop();
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_207() {
        assert!(Solution::can_finish(2, vec![vec![1, 0]]));
        assert!(!Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]));
        assert!(Solution::can_finish(
            8,
            vec![
                vec![1, 0],
                vec![2, 6],
                vec![1, 7],
                vec![6, 4],
                vec![7, 0],
                vec![0, 5]
            ]
        ));
    }
}
