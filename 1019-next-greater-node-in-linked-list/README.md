# [1019.链表中的下一个更大节点](https://leetcode.cn/problems/next-greater-node-in-linked-list/description/)

给定一个长度为 `n` 的链表 `head`

对于列表中的每个节点，查找下一个 **更大节点** 的值。也就是说，对于每个节点，找到它旁边的第一个节点的值，这个节点的值 **严格大于** 它的值。

返回一个整数数组 `answer` ，其中 `answer[i]` 是第 `i` 个节点( **从1开始** )的下一个更大的节点的值。如果第 `i` 个节点没有下一个更大的节点，设置 `answer[i] = 0` 。

 

**示例 1：**

![img](https://assets.leetcode.com/uploads/2021/08/05/linkedlistnext1.jpg)

```
输入：head = [2,1,5]
输出：[5,5,0]
```

**示例 2：**

![img](https://assets.leetcode.com/uploads/2021/08/05/linkedlistnext2.jpg)

```
输入：head = [2,7,4,3,5]
输出：[7,0,5,5,0]
```

 

**提示：**

- 链表中节点数为 `n`
- `1 <= n <= 10e4`
- `1 <= Node.val <= 10e9`

------

[Discussion](https://leetcode.cn/problems/next-greater-node-in-linked-list/comments/) | [Solution](https://leetcode.cn/problems/next-greater-node-in-linked-list/solution/)

**思路**

1、维护一个单调不增的栈

2、遍历链表，每次将 **链表元素**和 **栈顶元素** 进行比较，如果它比栈顶元素大，则栈顶元素的下一个更大元素就找到了，记录下来，弹出栈顶，继续判断，直到找不到为止

3、然后将 **链表元素**入栈

4、最后返回记录的结果即可

**题解**

```rust
impl Solution {
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut stk: Vec<(i32, i32)> = Vec::new();
        let mut result = Vec::new();

        let mut curr = head;
        let mut i = 0;
        while let Some(mut curr_node) = curr {
            result.push(0);
            while !stk.is_empty() && curr_node.val > stk.last().unwrap().0 {
                let index = stk.last().unwrap().1;
                result[index as usize] = curr_node.val;
                stk.pop();
            }
            stk.push((curr_node.val, i as i32));
            curr = curr_node.next.take();
            i += 1;
        }
        result
    }
}
```

