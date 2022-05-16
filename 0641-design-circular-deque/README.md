# [641.设计循环双端队列](https://leetcode.cn/problems/design-circular-deque/description/)

设计实现双端队列。

实现 `MyCircularDeque` 类:

- `MyCircularDeque(int k)` ：构造函数,双端队列最大为 `k` 。
- `boolean insertFront()`：将一个元素添加到双端队列头部。 如果操作成功返回 `true` ，否则返回 `false` 。
- `boolean insertLast()` ：将一个元素添加到双端队列尾部。如果操作成功返回 `true` ，否则返回 `false` 。
- `boolean deleteFront()` ：从双端队列头部删除一个元素。 如果操作成功返回 `true` ，否则返回 `false` 。
- `boolean deleteLast()` ：从双端队列尾部删除一个元素。如果操作成功返回 `true` ，否则返回 `false` 。
- `int getFront()` )：从双端队列头部获得一个元素。如果双端队列为空，返回 `-1` 。
- `int getRear()` ：获得双端队列的最后一个元素。 如果双端队列为空，返回 `-1` 。
- `boolean isEmpty()` ：若双端队列为空，则返回 `true` ，否则返回 `false` 。
- `boolean isFull()` ：若双端队列满了，则返回 `true` ，否则返回 `false` 。

 

**示例 1：**

```
输入
["MyCircularDeque", "insertLast", "insertLast", "insertFront", "insertFront", "getRear", "isFull", "deleteLast", "insertFront", "getFront"]
[[3], [1], [2], [3], [4], [], [], [], [4], []]
输出
[null, true, true, true, false, 2, true, true, true, 4]

解释
MyCircularDeque circularDeque = new MycircularDeque(3); // 设置容量大小为3
circularDeque.insertLast(1);			        // 返回 true
circularDeque.insertLast(2);			        // 返回 true
circularDeque.insertFront(3);			        // 返回 true
circularDeque.insertFront(4);			        // 已经满了，返回 false
circularDeque.getRear();  				// 返回 2
circularDeque.isFull();				        // 返回 true
circularDeque.deleteLast();			        // 返回 true
circularDeque.insertFront(4);			        // 返回 true
circularDeque.getFront();				// 返回 4
 
```

 

**提示：**

- `1 <= k <= 1000`
- `0 <= value <= 1000`
- `insertFront`, `insertLast`, `deleteFront`, `deleteLast`, `getFront`, `getRear`, `isEmpty`, `isFull` 调用次数不大于 `2000` 次

------

[Discussion](https://leetcode.cn/problems/design-circular-deque/comments/) | [Solution](https://leetcode.cn/problems/design-circular-deque/solution/)



**题解**

```rust
struct MyCircularDeque {
    val: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {
    fn new(k: i32) -> Self {
        Self {
            val: Vec::with_capacity(k as usize),
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.val.len() >= self.val.capacity() {
            return false;
        }
        self.val.insert(0, value);
        true
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.val.len() >= self.val.capacity() {
            return false;
        }
        self.val.push(value);
        true
    }

    fn delete_front(&mut self) -> bool {
        if !self.val.is_empty() {
            self.val.remove(0);
            return true;
        }
        false
    }

    fn delete_last(&mut self) -> bool {
        if !self.val.is_empty() {
            self.val.pop();
            return true;
        }
        false
    }

    fn get_front(&self) -> i32 {
        if self.val.is_empty() {
            return -1;
        }
        self.val[0]
    }

    fn get_rear(&self) -> i32 {
        if self.val.is_empty() {
            return -1;
        }
        *self.val.last().unwrap()
    }

    fn is_empty(&self) -> bool {
        self.val.is_empty()
    }

    fn is_full(&self) -> bool {
        self.val.len() == self.val.capacity()
    }
}

/**
 * Your MyCircularDeque object will be instantiated and called as such:
 * let obj = MyCircularDeque::new(k);
 * let ret_1: bool = obj.insert_front(value);
 * let ret_2: bool = obj.insert_last(value);
 * let ret_3: bool = obj.delete_front();
 * let ret_4: bool = obj.delete_last();
 * let ret_5: i32 = obj.get_front();
 * let ret_6: i32 = obj.get_rear();
 * let ret_7: bool = obj.is_empty();
 * let ret_8: bool = obj.is_full();
 */
```

