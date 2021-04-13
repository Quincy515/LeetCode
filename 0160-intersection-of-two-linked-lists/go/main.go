package main

// Definition for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

// 计算链表长度 Time: O(m+n), Space: O(1)
func getIntersectionNode(headA, headB *ListNode) *ListNode {
	lenA, lenB := 0, 0 // 首先初始化两个链表长度都为0
	// 遍历两个链表求它们的长度
	for p := headA; p != nil; p = p.Next {
		lenA++
	}
	for p := headB; p != nil; p = p.Next {
		lenB++
	}
	// 接着定义游标p和q分别指向两个链表
	p, q := headA, headB
	if lenA > lenB { // 如果链表A比较长
		for i := 0; i < lenA-lenB; i++ {
			p = p.Next // 就让游标p先移动链表长度之差
		}
	} else {
		for i := 0; i < lenB-lenA; i++ {
			q = q.Next // 否则让q先移动链表长度之差
		}
	}
	// 接下来只要p和q不相等就一起移动
	for p != q {
		p = p.Next
		q = q.Next
	}
	return p // 最后返回p即可
}

// 不计算链表长度 Time: O(m+n), Space: O(1)
func getIntersectionNodeWithoutLen(headA, headB *ListNode) *ListNode {
	// 边界情况，如果其中一条链表为空，返回空链表即可
	if headA == nil || headB == nil {
		return nil
	}
	// 否则定义游标p和q分别指向两个链表
	p, q := headA, headB
	// 当p和q不相等时执行以下操作
	for p != q {
		if p == nil { // 如果p为空
			p = headB // 就跳转到另一条链表的表头
		} else {
			p = p.Next // 否则p向前移动一个节点
		}
		// 类似的使用同样的方法处理q
		if q == nil { // 如果q为空
			q = headA // 就跳转到另一条链表的表头
		} else {
			q = q.Next // 否则p向前移动一个节点
		}
	}
	// 循环结束后p和q要么指向相交节点，要么都指向空
	return p // 因此只要返回p即可。
}
