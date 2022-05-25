struct Solution;

impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        // 离散化
        let mut pre_sums = vec![0];
        let mut s: i64 = 0;
        // 前缀和
        nums.iter().for_each(|&num| {
            s += num as i64;
            pre_sums.push(s);
        });

        let values: std::collections::BTreeSet<i64> = pre_sums
            .iter()
            .flat_map(|&x| vec![x, x - lower as i64, x - upper as i64])
            .collect();

        let rank: std::collections::HashMap<i64, usize> = values
            .iter()
            .enumerate()
            .map(|(i, &x)| (x, i))
            .collect();

        // 树状数组
        let mut result = 0;
        let mut tree = FenwickTree {
            sums: vec![0; rank.len() + 1],
        };
        for &x in &pre_sums {
            result +=
                tree.query(rank[&(x - lower as i64)]) - tree.query(rank[&(x - upper as i64)] - 1);
            tree.update(rank[&x], 1);
        }
        result
    }
}

#[derive(Debug)]
struct FenwickTree {
    sums: Vec<i32>,
}

impl FenwickTree {
    fn update(&mut self, mut i: usize, delta: i32) {
        while i < self.sums.len() {
            self.sums[i] += delta;
            i += self.lowbit(i);
        }
    }

    fn query(&self, mut i: usize) -> i32 {
        let mut res = 0;
        while i > 0 {
            res += self.sums[i];
            i -= self.lowbit(i);
        }
        res
    }

    fn lowbit(&self, x: usize) -> usize {
        x & (!x + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::count_range_sum(vec![1, 2, 3, 4, 5, 6], 1, 6), 9);
        assert_eq!(Solution::count_range_sum(vec![-2, 5, -1], -2, 2), 3);
        assert_eq!(Solution::count_range_sum(vec![0], 0, 0), 1);
    }
}
