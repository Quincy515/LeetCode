mod num_array;

struct NumArray {
    segment_tree: SegmentTree,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let segment_tree = SegmentTree::new(nums);
        Self { segment_tree }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.segment_tree
            .query(left as usize, right as usize)
            .unwrap()
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */

pub struct SegmentTree {
    pub data: Vec<i32>,
    pub tree: Vec<i32>,
}

impl SegmentTree {
    pub fn new(arr: Vec<i32>) -> Self {
        let mut data = vec![i32::MIN; arr.len()];
        arr.iter().enumerate().for_each(|(i, &num)| data[i] = num);

        let tree = vec![i32::MIN; 4 * arr.len()];
        let mut segment_tree = SegmentTree { data, tree };
        segment_tree.build_segment_tree(0, 0, arr.len() - 1);
        segment_tree
    }

    // 在 treeIndex 的位置创建表示区间[l, r]的线段树
    fn build_segment_tree(&mut self, tree_index: usize, l: usize, r: usize) {
        // 递归终止条件，区间长度为1，只有1个元素
        if l == r {
            self.tree[tree_index] = self.data[l];
            return;
        }

        // 表示区间的左右孩子对应的索引
        let left_tree_index = tree_index * 2 + 1;
        let right_tree_index = tree_index * 2 + 2;

        // 先创建该节点的左右子树
        let mid = l + (r - l) / 2;
        self.build_segment_tree(left_tree_index, l, mid);
        self.build_segment_tree(right_tree_index, mid + 1, r);

        // 综合左右两段区间信息
        self.tree[tree_index] = self.tree[left_tree_index] + self.tree[right_tree_index];
    }

    // 返回区间 [query_left, query_right] 的值
    pub fn query(&self, query_left: usize, query_right: usize) -> Option<i32> {
        if query_left > self.data.len() || query_right > self.data.len() || query_left > query_right
        {
            return None;
        }

        Some(self.recursive(0, 0, self.data.len() - 1, query_left, query_right))
    }

    // 在以 treeIndex 为根的线段树中 [l...r] 的范围里，搜索区间 [query_left, query_right] 的值
    fn recursive(
        &self,
        tree_index: usize,
        l: usize,
        r: usize,
        query_left: usize,
        query_right: usize,
    ) -> i32 {
        if l == query_left && r == query_right {
            return self.tree[tree_index];
        }

        let mid = l + (r - l) / 2;
        let left_tree_index = tree_index * 2 + 1;
        let right_tree_index = tree_index * 2 + 2;

        if query_left > mid {
            return self.recursive(right_tree_index, mid + 1, r, query_left, query_right);
        } else if query_right <= mid {
            return self.recursive(left_tree_index, l, mid, query_left, query_right);
        }

        let left_result = self.recursive(left_tree_index, l, mid, query_left, mid);
        let right_result = self.recursive(right_tree_index, mid + 1, r, mid + 1, query_right);
        left_result + right_result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let nums = vec![-2, 0, 3, -5, 2, -1];
        let segment_tree = SegmentTree::new(nums);
        println!("{:?}", segment_tree.query(0, 2));
        println!("{:?}", segment_tree.query(2, 5));
        println!("{:?}", segment_tree.query(0, 5));
    }
}
