struct Solution;
impl Solution {
    pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        let n = quiet.len();
        let mut edges = vec![Vec::new(); n];
        for i in richer {
            let (a, b) = (i[0], i[1]);
            edges[b as usize].push(a);
        }

        let mut result = vec![];
        for i in 0..n {
            let mut hash = std::collections::HashMap::new();
            let mut ans = i as i32;
            dfs(&quiet, i, &mut ans, &edges, &mut hash);
            result.push(ans);
        }
        result
    }
}

fn dfs(
    quiet: &[i32],
    u: usize,
    ans: &mut i32,
    edges: &[Vec<i32>],
    hash: &mut std::collections::HashMap<usize, usize>,
) {
    if hash.contains_key(&u) {
        return;
    }
    hash.entry(u).or_insert(1);
    if quiet[u] < quiet[*ans as usize] {
        *ans = u as i32;
    }
    for &edge in &edges[u] {
        dfs(quiet, edge as usize, ans, edges, hash);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::loud_and_rich(
                vec![
                    vec![1, 0],
                    vec![2, 1],
                    vec![3, 1],
                    vec![3, 7],
                    vec![4, 3],
                    vec![5, 3],
                    vec![6, 3]
                ],
                vec![3, 2, 5, 4, 6, 1, 7, 0]
            ),
            vec![5, 5, 2, 5, 4, 5, 6, 7]
        );

        assert_eq!(Solution::loud_and_rich(vec![], vec![0]), vec![0]);
    }
}
