struct Solution;

#[derive(Eq, PartialEq)]
struct Class {
    passing: i32,
    total: i32,
}

impl Class {
    pub fn delta(&self) -> f64 {
        let p = self.passing as f64;
        let t = self.total as f64;
        (p + 1.0) / (t + 1.0) - p / t
    }
}

impl std::cmp::Ord for Class {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl std::cmp::PartialOrd for Class {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.delta().partial_cmp(&other.delta())
    }
}

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, mut extra_students: i32) -> f64 {
        let mut heap = std::collections::BinaryHeap::new();
        for class in &classes {
            heap.push(Class {
                passing: class[0],
                total: class[1],
            })
        }
        while extra_students > 0 {
            let ord = heap.pop().unwrap();
            heap.push(Class {
                passing: ord.passing + 1,
                total: ord.total + 1,
            });
            extra_students -= 1;
        }

        let mut result = 0.0;
        while !heap.is_empty() {
            let class = heap.pop().unwrap();
            result += class.passing as f64 / class.total as f64;
        }
        result /= classes.len() as f64;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::max_average_ratio(vec![vec![1, 2], vec![3, 5], vec![2, 2]], 2),
            0.78333
        );
        assert_eq!(
            Solution::max_average_ratio(vec![vec![2, 4], vec![3, 9], vec![4, 5], vec![2, 10]], 4),
            0.53485
        );
    }
}
