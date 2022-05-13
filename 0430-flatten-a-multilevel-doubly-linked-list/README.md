# [430.扁平化多级双向链表](https://leetcode.cn/problems/flatten-a-multilevel-doubly-linked-list/description/)

你会得到一个双链表，其中包含的节点有一个下一个指针、一个前一个指针和一个额外的 **子指针** 。这个子指针可能指向一个单独的双向链表，也包含这些特殊的节点。这些子列表可以有一个或多个自己的子列表，以此类推，以生成如下面的示例所示的 **多层数据结构** 。

给定链表的头节点 head ，将链表 **扁平化** ，以便所有节点都出现在单层双链表中。让 `curr` 是一个带有子列表的节点。子列表中的节点应该出现在**扁平化列表**中的 `curr` **之后** 和 `curr.next` **之前** 。

返回 *扁平列表的 `head` 。列表中的节点必须将其 **所有** 子指针设置为 `null` 。*

 

**示例 1：**

![img](https://assets.leetcode.com/uploads/2021/11/09/flatten11.jpg)

```
输入：head = [1,2,3,4,5,6,null,null,null,7,8,9,10,null,null,11,12]
输出：[1,2,3,7,8,11,12,9,10,4,5,6]
解释：输入的多级列表如上图所示。
扁平化后的链表如下图：
```

**示例 2：**

![img](https://assets.leetcode.com/uploads/2021/11/09/flatten2.1jpg)

```
输入：head = [1,2,null,3]
输出：[1,3,2]
解释：输入的多级列表如上图所示。
扁平化后的链表如下图：
```

**示例 3：**

```
输入：head = []
输出：[]
说明：输入中可能存在空列表。
```

 

**提示：**

- 节点数目不超过 `1000`
- `1 <= Node.val <= 105`

 

**如何表示测试用例中的多级链表？**

以 **示例 1** 为例：

```
 1---2---3---4---5---6--NULL
         |
         7---8---9---10--NULL
             |
             11--12--NULL
```

序列化其中的每一级之后：

```
[1,2,3,4,5,6,null]
[7,8,9,10,null]
[11,12,null]
```

为了将每一级都序列化到一起，我们需要每一级中添加值为 null 的元素，以表示没有节点连接到上一级的上级节点。

```
[1,2,3,4,5,6,null]
[null,null,7,8,9,10,null]
[null,11,12,null]
```

合并所有序列化结果，并去除末尾的 null 。

```
[1,2,3,4,5,6,null,null,null,7,8,9,10,null,null,11,12]
```



[Discussion](https://leetcode.cn/problems/flatten-a-multilevel-doubly-linked-list/comments/) | [Solution](https://leetcode.cn/problems/flatten-a-multilevel-doubly-linked-list/solution/)

**思路**

1、利用一个递归函数 last = dfs(now)，一旦遇到 child 域非空的节点，则递归计算 clast = dfs(now->child)，返回值是递归函数展平后的最后一个节点，然后进行双向链表的链接操作

2、例如，当前有 child 域的节点为 now，它的下一个节点是 next，递归计算以后得到展平的链表的最后一个节点为 clast，则有如下关系：`now <-> now <-> child ... clast <-> next`

3、根据以上关系调整双向链表，注意不要忘记将 child 域置空

4、当遍历到这个双向链表的最后一个节点的时候，如果它有 child 域，则当前链表的最后一个节点就是 clast，否则就是它自己 now

**题解**

```c++
// Definition for a Node.
class Node {
public:
    int val;
    Node *prev;
    Node *next;
    Node *child;
};

class Solution {
    Node *dfs(Node *head) {
        Node *now = head;
        Node *last = nullptr;

        while (now) {
            Node *cLast;
            if (now->child) {
                cLast = dfs(now->child);
                Node *next = now->next;

                now->next = now->child;
                now->child = nullptr;
                now->next->prev = now;

                if (next) {
                    next->prev = cLast;
                }
                cLast->next = next;
            }
            if (now->next == nullptr) {
                if (now->child) {
                    last = cLast;
                } else {
                    last = now;
                }
            }
            now = now->next;
        }
        return last;
    }

public:
    Node *flatten(Node *head) {
        if (head == nullptr) {
            return nullptr;
        }
        Node *last = dfs(head);
        last->next = nullptr;
        return head;
    }
};
```

