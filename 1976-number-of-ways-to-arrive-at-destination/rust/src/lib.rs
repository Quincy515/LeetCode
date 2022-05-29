impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        use std::collections::BinaryHeap;
        const MOD: i64 = 1e9 as i64 + 7;
        let _len_r: usize = roads.len();
        // to build the graph
        let graph: Vec<Vec<(usize, i64)>> = {
            let mut graph: Vec<Vec<(usize, i64)>> = vec![Vec::new(); n as usize];
            for road in roads {
                let u: usize = road[0] as usize;
                let v: usize = road[1] as usize;
                let time: i64 = road[2] as i64;
                graph[u].push((v, time));
                graph[v].push((u, time));
            }
            graph
        };
        // to apply Djikstra's Algorithm
        let mut times: Vec<i64> = vec![i64::MAX; n as usize];
        times[0] = 0;
        let mut ways: Vec<i64> = vec![0; n as usize];
        ways[0] = 1;
        // used as a min heap
        let mut heap: BinaryHeap<(i64, usize)> = BinaryHeap::new();
        heap.push((0, 0));
        while !heap.is_empty() {
            if let Some((time_u, u)) = heap.pop() {
                let time_u = -time_u;
                if time_u > times[u] {
                    continue;
                }
                for &(v, time_v) in graph[u].iter() {
                    if times[v] > time_u + time_v {
                        times[v] = time_u + time_v;
                        ways[v] = ways[u];
                        heap.push((-times[v], v));
                    } else if times[v] == time_u + time_v {
                        ways[v] = (ways[v] + ways[u]) % MOD;
                    }
                }
            }
        }
        ways[n as usize - 1] as i32
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::count_paths(
                7,
                vec![
                    vec![0, 6, 7],
                    vec![0, 1, 2],
                    vec![1, 2, 3],
                    vec![1, 3, 3],
                    vec![6, 3, 3],
                    vec![3, 5, 1],
                    vec![6, 5, 1],
                    vec![2, 5, 1],
                    vec![0, 4, 5],
                    vec![4, 6, 2]
                ]
            ),
            4
        );
    }
}
