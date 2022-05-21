struct Solution;

#[derive(Eq, PartialEq)]
struct Pair {
    index: usize,
    value: i32,
}

impl std::cmp::Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value
            .cmp(&other.value)
            .then_with(|| other.index.cmp(&self.index))
    }
}
impl std::cmp::PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, mut k: i32) -> Vec<i32> {
        let mut heap = std::collections::BinaryHeap::new();
        nums.into_iter().enumerate().for_each(|(i, num)| {
            heap.push(Pair {
                index: i,
                value: num,
            })
        });
        let mut vec_heap = vec![];
        while k > 0 {
            vec_heap.push(heap.pop().unwrap());
            k -= 1;
        }
        vec_heap.sort_unstable_by_key(|item| item.index);
        vec_heap.into_iter().map(|x| x.value).collect()
    }

    pub fn max_subsequence_sort(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len());
        nums.into_iter()
            .enumerate()
            .for_each(|(i, num)| result.push((i, num)));
        result.sort_unstable_by(|x, y| y.1.cmp(&x.1));
        result.truncate(k as usize);
        result.sort_unstable_by_key(|item| item.0);
        result.into_iter().map(|x| x.1).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::max_subsequence(vec![2, 1, 3, 3], 2), vec![3, 3]);
        assert_eq!(
            Solution::max_subsequence(vec![-1, -2, 3, 4], 3),
            vec![-1, 3, 4]
        );
        assert_eq!(Solution::max_subsequence(vec![3, 4, 3, 3], 2), vec![3, 4]);
    }
}
