# [303.区域和检索 - 数组不可变](https://leetcode.cn/problems/range-sum-query-immutable/description/)

给定一个整数数组  `nums`，处理以下类型的多个查询:

1. 计算索引 `left` 和 `right` （包含 `left` 和 `right`）之间的 `nums` 元素的 **和** ，其中 `left <= right`

实现 `NumArray` 类：

- `NumArray(int[] nums)` 使用数组 `nums` 初始化对象
- `int sumRange(int i, int j)` 返回数组 `nums` 中索引 `left` 和 `right` 之间的元素的 **总和** ，包含 `left` 和 `right` 两点（也就是 `nums[left] + nums[left + 1] + ... + nums[right]` )

 

**示例 1：**

```
输入：
["NumArray", "sumRange", "sumRange", "sumRange"]
[[[-2, 0, 3, -5, 2, -1]], [0, 2], [2, 5], [0, 5]]
输出：
[null, 1, -1, -3]

解释：
NumArray numArray = new NumArray([-2, 0, 3, -5, 2, -1]);
numArray.sumRange(0, 2); // return 1 ((-2) + 0 + 3)
numArray.sumRange(2, 5); // return -1 (3 + (-5) + 2 + (-1)) 
numArray.sumRange(0, 5); // return -3 ((-2) + 0 + 3 + (-5) + 2 + (-1))
```

 

**提示：**

- `1 <= nums.length <= 104`
- `-105 <= nums[i] <= 105`
- `0 <= i <= j < nums.length`
- 最多调用 `104` 次 `sumRange` 方法

------

[Discussion](https://leetcode.cn/problems/range-sum-query-immutable/comments/) | [Solution](https://leetcode.cn/problems/range-sum-query-immutable/solution/)

**思路**

线段树

**题解**

```rust
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
```

因为数据不可变，不是动态数据，可以不使用线段树

```rust
struct NumArray {
    sum: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut sum = vec![0; nums.len() + 1];
        for i in 1..sum.len() {
            sum[i] = sum[i - 1] + nums[i - 1];
        }
        Self { sum }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.sum[right as usize + 1] - self.sum[left as usize]
    }
}
```

