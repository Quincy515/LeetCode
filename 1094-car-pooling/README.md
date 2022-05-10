# [1094.拼车](https://leetcode-cn.com/problems/car-pooling/description/)

车上最初有 `capacity` 个空座位。车 **只能** 向一个方向行驶（也就是说，**不允许掉头或改变方向**）

给定整数 `capacity` 和一个数组 `trips` ,  `trip[i] = [numPassengersi, fromi, toi]` 表示第 `i` 次旅行有 `numPassengersi` 乘客，接他们和放他们的位置分别是 `fromi` 和 `toi` 。这些位置是从汽车的初始位置向东的公里数。

当且仅当你可以在所有给定的行程中接送所有乘客时，返回 `true`，否则请返回 `false`。

 

**示例 1：**

```
输入：trips = [[2,1,5],[3,3,7]], capacity = 4
输出：false
```

**示例 2：**

```
输入：trips = [[2,1,5],[3,3,7]], capacity = 5
输出：true
```

 

**提示：**

- `1 <= trips.length <= 1000`
- `trips[i].length == 3`
- `1 <= numPassengersi <= 100`
- `0 <= fromi < toi <= 1000`
- `1 <= capacity <= 105`

------

[Discussion](https://leetcode-cn.com/problems/car-pooling/comments/) | [Solution](https://leetcode-cn.com/problems/car-pooling/solution/)

**思路**

1、用一个数数组来标记上下车的情况

2、一次线性扫描置上标记

3、然后累加标记，就是当前时间点列车上的人数，如果某个时间点人数超过了给定值，返回 **false**，一直没有超过，则返回 **true**

**题解**

```rust
impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut nums = vec![0; 1002];
        for i in 0..trips.len() {
            let cnt = trips[i][0];
            let from = trips[i][1];
            let to = trips[i][2];
            nums[from as usize] += cnt;
            nums[to as usize] -= cnt;
        }
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i];
            if sum > capacity {
                return false;
            }
        }
        true
    }
}
```

