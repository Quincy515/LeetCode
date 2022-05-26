struct Solution;

impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        let mut ufs = UnionFindSet::new(26);
        for equation in &equations {
            let str = equation.to_string().into_bytes();
            if str[1] == b'=' {
                ufs.union(
                    str[0] as usize - 'a' as usize,
                    str[3] as usize - 'a' as usize,
                );
            }
        }

        for equation in equations {
            let str = equation.chars().collect::<Vec<char>>();
            if str[1] == '!'
                && ufs.find(str[0] as usize - 'a' as usize)
                    == ufs.find(str[3] as usize - 'a' as usize)
            {
                return false;
            }
        }
        true
    }
}

struct UnionFindSet {
    parent: Vec<usize>,
    rank: Vec<i32>, // rank[i] 表示以 i 为根节点为 i 的树的高度
}

impl UnionFindSet {
    fn new(n: usize) -> Self {
        Self {
            rank: vec![0; n],
            parent: (0..n).collect::<Vec<_>>(),
        }
    }
    // 合并元素 i 和元素 j 所属的集合
    fn union(&mut self, i: usize, j: usize) -> bool {
        let (i_root, j_root) = (self.find(i), self.find(j));
        if i_root == j_root {
            return false;
        }

        if self.rank[i_root] > self.rank[j_root] {
            self.parent[j_root] = i_root;
        } else if self.rank[i_root] < self.rank[j_root] {
            self.parent[i_root] = j_root;
        } else {
            self.parent[i_root] = j_root;
            self.rank[j_root] += 1;
        }

        true
    }
    // 查找元素 i 所对应的集合编号
    fn find(&mut self, i: usize) -> usize {
        if i != self.parent[i] {
            self.parent[i] = self.find(self.parent[i]);
        }
        self.parent[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert!(!Solution::equations_possible(vec![
            "a==b".to_string(),
            "b!=a".to_string()
        ]));
        assert!(Solution::equations_possible(vec![
            "b==a".to_string(),
            "a==b".to_string()
        ]));
        assert!(Solution::equations_possible(vec![
            "a==b".to_string(),
            "b==c".to_string(),
            "a==c".to_string()
        ]));
        assert!(!Solution::equations_possible(vec![
            "a==b".to_string(),
            "b!=c".to_string(),
            "c==a".to_string()
        ]));
        assert!(Solution::equations_possible(vec![
            "c==c".to_string(),
            "b==d".to_string(),
            "x!=z".to_string()
        ]));
    }
}
