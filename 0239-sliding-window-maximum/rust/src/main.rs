use std::collections::VecDeque;

struct Solution;
/// 1. 算法原理
/// 使用双端队列实现一个单调递减队列，元素从队尾压入，从队尾或者队首弹出，直接取出队首元素即可得到最大值
/// 2. 算法流程
/// 1）给定的数组nums为空或者k为1，直接返回原数组
/// 2）处理前k个元素，初始化单调递减队列
/// 3）遍历数组，区间[k, nums.len()]，在每一步执行3个操作
/// - 清理队列，弹出所有小于当前值的元素（它们不可能是最大值），维持队列的单调递减
/// - 将当前元素从队尾压入队列
/// - 将最大值加入输出数组
/// 4）单调递减队列的处理函数如下。
/// push函数：当队尾元素小于当前值，弹出队尾元素，重复此步骤，直至队列为空，然后再将当前值从队尾压入
/// pop函数：当队首元素等于传入元素，弹出队首元素
/// max函数：返回队列中的最大值，即队首元素
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // 数组为空或者k为1，直接返回原数组
        if nums.len() == 0 || k == 1 { return nums; }

        let mut res: Vec<i32> = Vec::new();
        let mut deque: VecDeque<i32> = VecDeque::new();
        for i in 0..nums.len() {
            // 弹出队列中所有小于当前值的元素，再将当前值从队尾压入
            push(&mut deque, nums[i]);

            if (i as i32) > k - 1 {
                // 弹出队首元素，让滑动窗口内保持k个数字
                pop(&mut deque, nums[i - k as usize]);

                // 将最大值加入输出数组
                res.push(max(&deque));
            } else if (i as i32) == k - 1 {
                // 将前k个元素的最大值加入输出数组
                res.push(max(&deque));
            }
        }
        return res;
    }
}

fn push(deque: &mut VecDeque<i32>, n: i32) {
    // 当队列不为空且队尾元素小于当前值时，弹出队尾元素，直到队列为空
    while !deque.is_empty() && *deque.back().unwrap() < n {
        deque.pop_back();
    }
    deque.push_back(n);
}

fn pop(deque: &mut VecDeque<i32>, n: i32) {
    // 当队列不为空且队首元素等于传入元素，弹出队首元素
    if !deque.is_empty() && *deque.front().unwrap() == n {
        deque.pop_front();
    }
}

fn max(deque: &VecDeque<i32>) -> i32 {
    // 返回队列中的最大值，即队首元素
    return *deque.front().unwrap();
}


fn main() {
    let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
    println!("{:?}", Solution::max_sliding_window(nums, 3));
}
