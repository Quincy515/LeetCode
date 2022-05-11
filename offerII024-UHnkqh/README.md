# [剑指 Offer II 024. 反转链表](https://leetcode.cn/problems/UHnkqh/)

给定单链表的头节点 head ，请反转链表，并返回反转后的链表的头节点。

 

**示例 1：**

![](https://assets.leetcode.com/uploads/2021/02/19/rev1ex1.jpg)

```bash
输入：head = [1,2,3,4,5]
输出：[5,4,3,2,1]
```



**示例 2：**

![](https://assets.leetcode.com/uploads/2021/02/19/rev1ex2.jpg)

```bash
输入：head = [1,2]
输出：[2,1]
```



**示例 3：**

```bash
输入：head = []
输出：[]
```



**提示：**

- 链表中节点的数目范围是 [0, 5000]
- -5000 <= Node.val <= 5000



**进阶：** 链表可以选用迭代或递归方式完成反转。你能否用两种方法解决这道题？

 

注意：本题与主站 206 题相同： https://leetcode-cn.com/problems/reverse-linked-list/



**思路**

1、如果链表长度为 **n**，则执行 **n-1** 次迭代，每次迭代将一个节点插入到链表头，此所谓 **头插法**；

2、有一个常用技巧，就是生成一个 **伪头节点**，指向当前的头节点

3、遍历链表的过程中，不断将遍历到的节点删除，插入到 **伪头节点** 的下一个节点

**题解**

1. 定义两个节点，`prev` 代表前一个节点，`curr` 代表当前节点
2. 初始将 `prev` 节点设置为空，`curr` 节点设置为头节点 `head`
3. 遍历链表，将 `curr` 节点的 `next` 指针改为指向 `prev` 节点，再分别将 `prev` 节点和 `curr` 节点往后移动一个节点
4. 遍历完成后，`curr` 节点为空，`prev` 节点就是新的头节点

```rust
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut curr = head;

        while let Some(mut curr_node) = curr.take() {
            // 保存当前节点的下一个节点
            let next_temp = curr_node.next.take();
            // 将当前节点指向 prev 节点
            curr_node.next = prev.take();

            // prev 和 curr 分别往后移动一个节点，即
            // 把当前节点 curr_node 赋值给 prev
            // 把之前保存的当前节点的下一个节点 next_temp 赋值给 curr
            prev = Some(curr_node);
            curr = next_temp;
        }

        prev
    }
}
```

