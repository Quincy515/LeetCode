# [307.区域和检索 - 数组可修改](https://leetcode.cn/problems/range-sum-query-mutable/description/)

给你一个数组 `nums` ，请你完成两类查询。

1. 其中一类查询要求 **更新** 数组 `nums` 下标对应的值
2. 另一类查询要求返回数组 `nums` 中索引 `left` 和索引 `right` 之间（ **包含** ）的nums元素的 **和** ，其中 `left <= right`

实现 `NumArray` 类：

- `NumArray(int[] nums)` 用整数数组 `nums` 初始化对象
- `void update(int index, int val)` 将 `nums[index]` 的值 **更新** 为 `val`
- `int sumRange(int left, int right)` 返回数组 `nums` 中索引 `left` 和索引 `right` 之间（ **包含** ）的nums元素的 **和** （即，`nums[left] + nums[left + 1], ..., nums[right]`）

 

**示例 1：**

```
输入：
["NumArray", "sumRange", "update", "sumRange"]
[[[1, 3, 5]], [0, 2], [1, 2], [0, 2]]
输出：
[null, 9, null, 8]

解释：
NumArray numArray = new NumArray([1, 3, 5]);
numArray.sumRange(0, 2); // 返回 1 + 3 + 5 = 9
numArray.update(1, 2);   // nums = [1,2,5]
numArray.sumRange(0, 2); // 返回 1 + 2 + 5 = 8
```

 

**提示：**

- `1 <= nums.length <= 3 * 104`
- `-100 <= nums[i] <= 100`
- `0 <= index < nums.length`
- `-100 <= val <= 100`
- `0 <= left <= right < nums.length`
- 调用 `update` 和 `sumRange` 方法次数不大于 `3 * 104` 

------

[Discussion](https://leetcode.cn/problems/range-sum-query-mutable/comments/) | [Solution](https://leetcode.cn/problems/range-sum-query-mutable/solution/)

**思路**

线段树 [303.线段树的查询](../0303-range-sum-query-immutable)

**题解**

```rust
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

    fn update(&mut self, index: i32, val: i32) {
        self.segment_tree.set(index as usize, val);
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
 * obj.update(index, val);
 * let ret_2: i32 = obj.sum_range(left, right);
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

    // 将 index 位置的值，更新为 val
    pub fn set(&mut self, index: usize, val: i32) {
        if index >= self.data.len() {
            return;
        }
        self.data[index] = val;
        self.recursive_set(0, 0, self.data.len() - 1, index, val);
    }

    // 在以 tree_index 为根的线段树中更新 index 的值为 val
    fn recursive_set(&mut self, tree_index: usize, l: usize, r: usize, index: usize, val: i32) {
        if l == r {
            self.tree[tree_index] = val;
            return;
        }

        let mid = l + (r - l) / 2;
        let left_tree_index = tree_index * 2 + 1;
        let right_tree_index = tree_index * 2 + 2;

        if index > mid {
            self.recursive_set(right_tree_index, mid + 1, r, index, val);
        } else {
            self.recursive_set(left_tree_index, l, mid, index, val); // index <= mid
        }

        self.tree[tree_index] = self.tree[left_tree_index] + self.tree[right_tree_index];
    }
}
```

### Binary indexed tree 树状数组

```rust
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
        let mut result = 0;
        while i > 0 {
            result += self.sums[i];
            i -= self.lowbit(i);
        }
        result
    }

    fn lowbit(&self, x: usize) -> usize {
        x & (!x + 1) // x & (-x)
    }
}

struct NumArray {
    nums: Vec<i32>,
    fenwick_tree: FenwickTree,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut fenwick_tree = FenwickTree {
            sums: vec![0; nums.len() + 1],
        };
        for i in 0..nums.len() {
            fenwick_tree.update(i + 1, nums[i]);
        }

        Self { nums, fenwick_tree }
    }

    fn update(&mut self, index: i32, val: i32) {
        self.fenwick_tree
            .update(index as usize + 1, val - self.nums[index as usize]);
        self.nums[index as usize] = val;
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.fenwick_tree.query(right as usize + 1) - self.fenwick_tree.query(left as usize)
    }
}
```

