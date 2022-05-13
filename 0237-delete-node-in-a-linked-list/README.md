# [237.删除链表中的节点](https://leetcode.cn/problems/delete-node-in-a-linked-list/description/)

请编写一个函数，用于 **删除单链表中某个特定节点** 。在设计函数时需要注意，你无法访问链表的头节点 `head` ，只能直接访问 **要被删除的节点** 。

题目数据保证需要删除的节点 **不是末尾节点** 。

 

**示例 1：**

![img](https://assets.leetcode.com/uploads/2020/09/01/node1.jpg)

```
输入：head = [4,5,1,9], node = 5
输出：[4,1,9]
解释：指定链表中值为 5 的第二个节点，那么在调用了你的函数之后，该链表应变为 4 -> 1 -> 9
```

**示例 2：**

![img](https://assets.leetcode.com/uploads/2020/09/01/node2.jpg)

```
输入：head = [4,5,1,9], node = 1
输出：[4,5,9]
解释：指定链表中值为 1 的第三个节点，那么在调用了你的函数之后，该链表应变为 4 -> 5 -> 9
```

 

**提示：**

- 链表中节点的数目范围是 `[2, 1000]`
- `-1000 <= Node.val <= 1000`
- 链表中每个节点的值都是 **唯一** 的
- 需要删除的节点 `node` 是 **链表中的节点** ，且 **不是末尾节点**

------

[Discussion](https://leetcode.cn/problems/delete-node-in-a-linked-list/comments/) | [Solution](https://leetcode.cn/problems/delete-node-in-a-linked-list/solution/)

**思路**

1、这是一道脑筋急转弯题

2、我们知道删除一个链表节点，一定要知道它的前驱节点，但是这题不知道，所以我们可以进行一个取巧。将 **给定节点** 的 **下一个节点** 的值赋值给 **给定节点**，然后删除 **给点节点** 的 **下一个节点**，这样就达到了删除 **给定节点** 的目的

**题解**

```go
package main

//Definition for singly-linked list.
type ListNode struct {
	Val int
	Next *ListNode
}

func deleteNode(node *ListNode) {
    node.Val = node.Next.Val
    node.Next = node.Next.Next
}
```

```c++
 struct ListNode {
    int val;
    ListNode *next;
    ListNode(int x) : val(x), next(NULL) {}
};

class Solution {
public:
    void deleteNode(ListNode* node) {
        node->val = node->next->val;    
        node->next = node->next->next;
    }
};
```

