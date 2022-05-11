# [1290.二进制链表转整数](https://leetcode.cn/problems/convert-binary-number-in-a-linked-list-to-integer/description/)

给你一个单链表的引用结点 `head`。链表中每个结点的值不是 0 就是 1。已知此链表是一个整数数字的二进制表示形式。

请你返回该链表所表示数字的 **十进制值** 。

 

**示例 1：**

![img](https://assets.leetcode-cn.com/aliyun-lc-upload/uploads/2019/12/15/graph-1.png)

```
输入：head = [1,0,1]
输出：5
解释：二进制数 (101) 转化为十进制数 (5)
```

**示例 2：**

```
输入：head = [0]
输出：0
```

**示例 3：**

```
输入：head = [1]
输出：1
```

**示例 4：**

```
输入：head = [1,0,0,1,0,0,1,1,1,0,0,0,0,0,0]
输出：18880
```

**示例 5：**

```
输入：head = [0,0]
输出：0
```

 

**提示：**

- 链表不为空。
- 链表的结点总数不超过 `30`。
- 每个结点的值不是 `0` 就是 `1`。

------

[Discussion](https://leetcode.cn/problems/convert-binary-number-in-a-linked-list-to-integer/comments/) | [Solution](https://leetcode.cn/problems/convert-binary-number-in-a-linked-list-to-integer/solution/)

**思路**

- 本题考察的是链表的遍历

**题解**

**Rust 实战要点**

1. 基础概念
   1. 可变变量声明
   2. 智能指针
   3. Option类型使用
   4. while let
2. 数据结构
   1. 自定义结构 ListNode
3. 流程控制
   1. while 循环
4. 要点提示
   1. 使用 mut 声明可变变量
   2. 使用 Box 智能指针
   3. 使用 Option 的 take 方法取值
   4. 使用 while let 简化模式匹配处理

```rust
impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut curr = head;
        let mut result = 0;
        while let Some(mut curr_node) = curr.take() {
            result = result * 2 + curr_node.val;
            curr = curr_node.next.take();
        }

        result
    }
}
```

**C++**

```c++
class Solution {
public:
    int getDecimalValue(ListNode* head) {
        int sum = 0;
        while (head) {
            sum = sum * 2 + head->val;
            head = head->next;
        }
        return sum;
    }
};
```

