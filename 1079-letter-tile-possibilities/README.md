# [1079.活字印刷](https://leetcode.cn/problems/letter-tile-possibilities/description/)

你有一套活字字模 `tiles`，其中每个字模上都刻有一个字母 `tiles[i]`。返回你可以印出的非空字母序列的数目。

**注意：**本题中，每个活字字模只能使用一次。

 

**示例 1：**

```
输入："AAB"
输出：8
解释：可能的序列为 "A", "B", "AA", "AB", "BA", "AAB", "ABA", "BAA"。
```

**示例 2：**

```
输入："AAABBC"
输出：188
```

**示例 3：**

```
输入："V"
输出：1
```

 

**提示：**

- `1 <= tiles.length <= 7`
- `tiles` 由大写英文字母组成本插件 V1.1.2 最新版支持复制代码块及小灯泡注释，详情 [见这里](https://mp.weixin.qq.com/s/z4mqiiJV9pZ3t6SIPa2kTA)**。

------

[Discussion](https://leetcode.cn/problems/letter-tile-possibilities/comments/) | [Solution](https://leetcode.cn/problems/letter-tile-possibilities/solution/)

**思路**



**题解**

```rust
impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        let mut dict = [0; 26];
        for c in tiles.chars() {
            dict[c as usize - 'A' as usize] += 1;
        }
        let mut count = 0;
        Solution::blacktrack(&mut dict, &mut count);
        count
    }
    fn blacktrack(dict: &mut [i32; 26], count: &mut i32) {
        for i in 0..dict.len() {
            if dict[i] > 0 {
                *count += 1;
                dict[i] -= 1;
                Solution::blacktrack(dict, count);
                dict[i] += 1;
            }
        }
    }
}
```

