struct Solution;
impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ufs = UnionFindSet::new(edges.len());
        for edge in edges {
            if !ufs.union(edge[0] as usize, edge[1] as usize) {
                return edge;
            }
        }
        vec![]
    }
}

struct UnionFindSet {
    parents: Vec<usize>,
    ranks: Vec<i32>,
}

impl UnionFindSet {
    fn new(n: usize) -> Self {
        Self {
            ranks: vec![0; n + 1],
            parents: (0..n + 1).collect::<Vec<_>>(),
        }
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let (root_i, root_j) = (self.find(i), self.find(j));
        if root_i == root_j {
            return false;
        }

        if self.ranks[root_i] > self.ranks[root_j] {
            self.parents[root_j] = root_i;
        } else if self.ranks[root_i] < self.ranks[root_j] {
            self.parents[root_i] = root_j;
        } else {
            self.parents[root_i] = root_j;
            self.ranks[root_j] += 1;
        }

        true
    }

    fn find(&mut self, i: usize) -> usize {
        if i != self.parents[i] {
            self.parents[i] = self.find(self.parents[i]);
        }
        self.parents[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
            vec![2, 3]
        );
        assert_eq!(
            Solution::find_redundant_connection(vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![1, 4],
                vec![1, 5]
            ]),
            vec![1, 4]
        );
    }
}
