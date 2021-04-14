// Definition for singly-linked list.
package mai

type ListNode struct {
	Val  int
	Next *ListNode
}

// 迭代方法
// https://github.com/aQuaYi/LeetCode-in-Go/blob/master/Algorithms/0206.reverse-linked-list/reverse-linked-list.go
func reverseList(head *ListNode) *ListNode {
	cur, pre := head, &ListNode{} // pre 是所有已经逆转的节点
	for cur != nil {              // cur 是当前被逆转的节点
		next := cur.Next // 先保存下一个节点
		cur.Next = pre   // 逆转当前节点，使当前节点指向前一个节点
		pre = cur        // 该节点已经完成逆转
		cur = next       // 下一个将被逆转的节点
	}
	return pre // 返回完成逆转之后的头节点
}

// 递归方法
func reverseList(head *ListNode) *ListNode {
	return reverse(nil, head)
}

func reverse(pre, cur *ListNode) *ListNode {
	if cur == nil {
		return pre
	}
	head := reverse(cur, cur.Next)
	cur.Next = pre
	return head
}
