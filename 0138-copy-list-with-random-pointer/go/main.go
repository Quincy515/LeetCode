package main

/**
 * Definition for a Node.
 */
type Node struct {
	Val    int
	Next   *Node
	Random *Node
}

func copyRandomList(head *Node) *Node {
	// 边界判断，一般链表的题目都需要判断头节点是否为空
	if head == nil {
		return nil
	}
	// 从链表的头节点开始遍历
	cur := head
	// 使用一一对应的哈希表结构 Map 存放已经创建的节点
	hash := make(map[*Node]*Node)

	// 遍历原链表
	for cur != nil {
		// 以原链表的节点为 Key，构建一个 Map
		// Map 的 Value 为一个新链表中的节点
		// 新节点的值 val 和原链表的值 val 一样
		// 但原链表中的每个节点都有 next 和 random 指针，而 Map 中的 Value 没有 next 和 random 指针
		// map.put(Key,Value)
		newNode := Node{Val: cur.Val}
		hash[cur] = &newNode
		// 查看下一个节点的情况
		cur = cur.Next
	}

	// 再次从链表的头节点开始遍历
	cur = head

	// 遍历原链表
	for cur != nil {

		// 原链表节点 ----  新链表节点
		// key      ----- value
		// cur      ----- map.get(cur)

		// 0、在字典中找到一个 cur 为 key 对应的那个 value 值
		valueCur := hash[cur]

		// 接下来，需要去寻找 valueCur 的 next 节点和 random 节点

		// 寻找 valueCur 的 next 节点
		// 1、获取当前节点 cur 在原链表中的 next 指针指向的节点
		keyNextNode := cur.Next

		// 2、在字典中找到以 keyNextNode 为 key 对应的那个 value 值
		valueNextNode := hash[keyNextNode]

		// 3、那么新链表中的这个节点的 next 指针就是 valueNextNode
		valueCur.Next = valueNextNode

		// 寻找 valueCur 的 random 节点
		// 1、获取当前节点 cur 在原链表中的 random 指针指向的节点
		keyRandomNode := cur.Random

		// 2、在字典中找到以 valueRandomNode 为 key 对应的那个 value 值
		valueRandomNode := hash[keyRandomNode]

		// 4、那么新链表中的这个节点的 next 指针就是 valueNextNode
		valueCur.Random = valueRandomNode

		//遍历下去，查看下一个节点
		cur = cur.Next
	}
	// 原链表节点 ----  新链表节点
	// key      ----- value
	// cur      ----- map.get(cur)
	// head     ----- map.get(head)
	return hash[head]
}

func main() {

}
