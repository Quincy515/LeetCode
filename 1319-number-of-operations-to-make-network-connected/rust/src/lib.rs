struct Solution;

impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let (mut edge, mut count) = (0, 0);
        let mut hash = std::collections::HashMap::new();
        let mut uf = UnionFindSet::new(n as usize);
        for connection in connections.iter() {
            let (a, b) = (connection[0], connection[1]);
            if !uf.union(a as usize, b as usize) {
                edge += 1;
            }
        }

        for i in 0..n {
            let set_id = uf.find(i as usize);
            if !hash.contains_key(&set_id) {
                hash.insert(set_id, 1);
                count += 1;
            }
        }

        if edge >= count - 1 {
            return count - 1;
        }
        -1
    }
}

struct UnionFindSet {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFindSet {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect::<Vec<_>>(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if x != self.parent[x] {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let (i_root, j_root) = (self.find(i), self.find(j));
        if i_root == j_root {
            return false;
        }
        if self.rank[i_root] < self.rank[j_root] {
            self.parent[i_root] = j_root;
        } else if self.rank[i_root] > self.rank[j_root] {
            self.parent[j_root] = i_root;
        } else {
            self.parent[j_root] = i_root;
            self.rank[i_root] += 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::make_connected(4, vec![vec![0, 1], vec![0, 2], vec![1, 2]]),
            1
        );
        assert_eq!(
            Solution::make_connected(
                6,
                vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2], vec![1, 3]]
            ),
            2
        );
        assert_eq!(
            Solution::make_connected(6, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2]]),
            -1
        );
        assert_eq!(
            Solution::make_connected(5, vec![vec![0, 1], vec![0, 2], vec![3, 4], vec![2, 3]]),
            0
        );
    }
}
