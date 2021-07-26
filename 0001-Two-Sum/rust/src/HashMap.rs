use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();

        // 遍历数组，将每个元素的值和它的索引添加到哈希表中
        for i in 0..nums.len() {
            map.insert(nums[i], i);
        }

        // 遍历数组，检查每个元素所对应的目标元素(target-nums[i])是否存在于哈希表中
        for i in 0..nums.len() {
            let complement = target - nums[i];
            if map.contains_key(&complement) && map[&complement] != i{
                return vec![i as i32, map[&complement] as i32];
            }
        }
        return vec![];
    }
}

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::new();

        for i in 0..nums.len() {
            let complement = target - nums[i];
            if map.contains_key(&complement) {
                // 存在目标元素，立即返回结果
                return vecc![i as i32, map[&complement] as i32];
            }
            // 不存在目标元素，将键值对(nums[i], i)插入哈希表，继续遍历
            map.insert(nums[i], i);
        }
        return vec![];
    }
}
