# [215.数组中的第K个最大元素](https://leetcode.cn/problems/kth-largest-element-in-an-array/description/)

给定整数数组 `nums` 和整数 `k`，请返回数组中第 `**k**` 个最大的元素。

请注意，你需要找的是数组排序后的第 `k` 个最大的元素，而不是第 `k` 个不同的元素。

 

**示例 1:**

```
输入: [3,2,1,5,6,4] 和 k = 2
输出: 5
```

**示例 2:**

```
输入: [3,2,3,1,2,4,5,5,6] 和 k = 4
输出: 4
```

 

**提示：**

- `1 <= k <= nums.length <= 104`
- `-104 <= nums[i] <= 104`

------

[Discussion](https://leetcode.cn/problems/kth-largest-element-in-an-array/comments/) | [Solution](https://leetcode.cn/problems/kth-largest-element-in-an-array/solution/) | [博客](https://editor.csdn.net/md/?articleId=125027099)

## 思想
归并排序 MergeSort，是使用分治算法思想，将待排序数组从中间分解成前后两部分，然后对前后两部分分别进行排序，最后将排好序的两部分合并在一起，这样整个数组就有序了。

分治算法 3 个操作：
- **分解**：将待排序的元素拆分成两个或多个子序列，拆分后的子序列以相同的方式继续拆分，直到每个序列中都只包含一个元素。
- **求解**：然后开始排序，先使每个子序列有序，再使子序列段间有序。
- **合并**：最后逐步将已经有序的子序列合并，最终得到完全有序的序列。
## 算法流程
如何用递归来实现归并排序：

编写递归代码的技巧是：**写递推公式**，**寻找终止条件**，最后将递推公式和终止条件“翻译”成代码。

因此要想写出归并排序的递归代码，先要写出它的递推公式和终止条件：

- **递推公式**：`merge_sort(l, r) = merge(merge_sort(l, mid), merge_sort(mid+1, r))`
- **终止条件**：`l >= r`，不用再继续分解

算法流程：

1. 把长度为 n 的待排序元素拆分成两个长度为 $n/2$ 的子序列
2. 针对这两个子序列继续重复步骤1，直到拆分成的子序列中只包含一个元素
3. 开始排序，排序的方法是按照大小顺序合并两个元素
4. 重复步骤3，按照大小顺序不断地合并排好序的子序列，直到最终将两个排好序的子序列合并成一个完全有序的序列

## Rust 代码模版

```rust
impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let nums = merge_sort(nums);
        // 第 k 个最大元素就是第 n -k 个最小元素
        nums[nums.len() - k as usize]
    }
}

pub fn merge_sort(mut nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() {
        return nums;
    }

    let n = nums.len() - 1;
    merge_sort_recursion(&mut nums, 0, n);
    nums
}

fn merge_sort_recursion(nums: &mut Vec<i32>, left: usize, right: usize) {
    // 判断是否只剩下最后一个元素
    if left >= right {
        return;
    }

    // 从中间将数组分成两个序列
    let mid = left + (right - left) / 2;

    // 分别以递归方式将左右两个序列排好序
    merge_sort_recursion(nums, left, mid);
    merge_sort_recursion(nums, mid + 1, right);

    // 将已有序的两个序列合并
    merge(nums, left, mid, right);
}

fn merge(nums: &mut Vec<i32>, left: usize, mid: usize, right: usize) {
    // 定义索引 i 表示左序列的起始位置
    let mut i = left;

    // 定义索引 j 表示右序列的起始位置
    let mut j = mid + 1;

    // 定义索引 k 表示开始排序原数组的位置
    let mut k = left;

    // 定义用于排序过程中临时存放元素的数组
    let mut tmp = vec![];

    while k <= right {
        if i > mid {
            // 左序列元素处理完毕，剩下右序列元素，将右序列元素逐个添加
            tmp.push(nums[j]);
            j += 1;
            k += 1;
        } else if j > right {
            // 右序列元素处理完毕，剩下左序列元素，将左序列元素逐个添加
            tmp.push(nums[i]);
            i += 1;
            k += 1;
        } else if nums[i] < nums[j] {
            // 左序列元素小于右序列元素，将左序列元素添加，索引 i 往前移动一位
            tmp.push(nums[i]);
            i += 1;
            k += 1;
        } else {
            // 左序列元素大于右序列元素，将右序列元素添加，索引 j 往前移动一位
            tmp.push(nums[j]);
            j += 1;
            k += 1;
        }
    }

    // 将已排序的元素按对应位置替换原数组元素
    for i in 0..=(right - left) {
        nums[left + i] = tmp[i];
    }
}
```

