# [401.二进制手表](https://leetcode.cn/problems/binary-watch/description/)

二进制手表顶部有 4 个 LED 代表 **小时（0-11）**，底部的 6 个 LED 代表 **分钟（0-59）**。每个 LED 代表一个 0 或 1，最低位在右侧。

- 例如，下面的二进制手表读取 `"3:25"` 。

![img](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2021/03/29/binary_clock_samui_moon.jpg)

*（图源：[WikiMedia - Binary clock samui moon.jpg](https://commons.m.wikimedia.org/wiki/File:Binary_clock_samui_moon.jpg) ，许可协议：[Attribution-ShareAlike 3.0 Unported (CC BY-SA 3.0)](https://creativecommons.org/licenses/by-sa/3.0/deed.en) ）*

给你一个整数 `turnedOn` ，表示当前亮着的 LED 的数量，返回二进制手表可以表示的所有可能时间。你可以 **按任意顺序** 返回答案。

小时不会以零开头：

- 例如，`"01:00"` 是无效的时间，正确的写法应该是 `"1:00"` 。

分钟必须由两位数组成，可能会以零开头：

- 例如，`"10:2"` 是无效的时间，正确的写法应该是 `"10:02"` 。

 

**示例 1：**

```
输入：turnedOn = 1
输出：["0:01","0:02","0:04","0:08","0:16","0:32","1:00","2:00","4:00","8:00"]
```

**示例 2：**

```
输入：turnedOn = 9
输出：[]
```

 

**提示：**

- `0 <= turnedOn <= 10`

------

[Discussion](https://leetcode.cn/problems/binary-watch/comments/) | [Solution](https://leetcode.cn/problems/binary-watch/solution/)

**思路**

1、总共10个位置，每个位置可以选择“选”或者“不选”，最多1024种情况

2、暴力枚举这些情况以后，分别计算小时和分钟，再转换成字符串即可

**题解**

```rust
impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let mut result = Vec::new();
        for i in 0..12 {
            for j in 0..60 {
                if turned_on as u32 == (i as i32).count_ones() + (j as i32).count_ones() {
                    let mut s = String::new();
                    s.push_str(&(i.to_string()));
                    s.push(':');
                    if j < 10 {
                        s.push('0');
                    }
                    s.push_str(&(j.to_string()));
                    result.push(s);
                }
            }
        }
        result
    }
}
```

