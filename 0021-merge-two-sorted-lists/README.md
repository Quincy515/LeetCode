# [21.合并两个有序链表](https://leetcode.cn/problems/merge-two-sorted-lists/description/)

将两个升序链表合并为一个新的 **升序** 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。 

 

**示例 1：**

![img](https://assets.leetcode.com/uploads/2020/10/03/merge_ex1.jpg)

```
输入：l1 = [1,2,4], l2 = [1,3,4]
输出：[1,1,2,3,4,4]
```

**示例 2：**

```
输入：l1 = [], l2 = []
输出：[]
```

**示例 3：**

```
输入：l1 = [], l2 = [0]
输出：[0]
```

 

**提示：**

- 两个链表的节点数目范围是 `[0, 50]`
- `-100 <= Node.val <= 100`
- `l1` 和 `l2` 均按 **非递减顺序** 排列

------

[Discussion](https://leetcode.cn/problems/merge-two-sorted-lists/comments/) | [Solution](https://leetcode.cn/problems/merge-two-sorted-lists/solution/)

```rust
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution {
  pub fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
  ) -> Option<Box<ListNode>> {
    let (mut l1, mut l2) = (l1, l2);
    // 一开始设置一个虚拟节点，它的值为 -1，它的值可以设置为任何的数，因为我们根本不需要使用它的值
    let mut dummy = Some(Box::new(ListNode::new(0)));
    // 设置一个指针，指向虚拟节点
    let mut p = &mut dummy;

    // 通过一个循环，不断的比较 l1 和 l2 中当前节点值的大小，直到 l1 或者 l2 遍历完毕为止
    while l1.is_some() && l2.is_some() {
      // 如果 l1 当前节点的值小于等于 l2 当前节点的值
      if l1.as_ref().unwrap().val < l2.as_ref().unwrap().val {
        // 让 pre 指向节点的 next 指针指向这个更小值的节点，即指向 l1
        p.as_mut().unwrap().next = l1.take();
        // 让 l1 向后移动
        l1 = p.as_mut().unwrap().next.as_mut().unwrap().next.take();
      } else {
        // 让 pre 指向节点的 next 指针指向这个更小值的节点，即指向 l2
        p.as_mut().unwrap().next = l2.take();
        // 让 l2 向后移动
        l2 = p.as_mut().unwrap().next.as_mut().unwrap().next.take();
      }
      // 让 pre 向后移动
      p = &mut p.as_mut().unwrap().next;
    }

    // 跳出循环后，l1 或者 l2 中可能有剩余的节点没有被观察过
    // 直接把剩下的节点加入到 pre 的 next 指针位置

    // 如果 l1 中还有节点
    if !l1.is_none() {
      // 把 l1 中剩下的节点全部加入到 pre 的 next 指针位置
      p.as_mut().unwrap().next = l1;
    }

    // 如果 l2 中海油节点
    if !l2.is_none() {
      // 把 l2 中剩下的节点全部加入到 pre 的 next 指针位置
      p.as_mut().unwrap().next = l2;
    }

    // 最后返回虚拟节点的 next 指针
    dummy.unwrap().next
  }
}
```

## 解题思路
将两个有序链表合并成一个有序链表，可以采用每次比较 list1 和 list2 所指向的节点值大小，获取值较小的节点，直到两个节点都为空时合并完成。

## 算法流程
1、如果 list1 或 list2 为空，直接返回非空列表
2、以递归的方式，判断 list1 和 list2 所指向的节点值哪个较小，
  - 较小节点的 next 指针指向其余节点的合并结果。
  - 若 list1.val 小于等于 list2.val，继续比较 list1.next 和 list2
  - 若 list1.val 大于 list2.val，继续比较 list1 和 list2.next
3、直到两个节点都为空，递归种植，链表合并完成

```rust
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(node1), None) => Some(node1),
            (None, Some(node2)) => Some(node2),
            (Some(mut node1), Some(mut node2)) => {
                if node1.val < node2.val {
                    let n = node1.next.take();
                    node1.next = Solution::merge_two_lists_(n, Some(node2));
                    Some(node1)
                } else {
                    let n = node2.next.take();
                    node2.next = Solution::merge_two_lists_(Some(node1), n);
                    Some(node2)
                }
            }
            _ => None,
        }
    }
```

