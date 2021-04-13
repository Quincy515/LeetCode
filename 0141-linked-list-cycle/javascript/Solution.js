
// Definition for singly-linked list.
function ListNode(val) {
  this.val = val;
  this.next = null;
}


/**
 * @param {ListNode} head
 * @return {boolean}
 */
// 双指针
var hasCycle = function (head) {
  let fast = head, slow = head;
  while (fast && fast.next) {
    fast = fast.next.next;
    slow = slow.next;
    if (fast == slow) {
      return true;
    }
  }
  return false;
};

// hash表
var hasCycle = function (head) {
  const set = new Set();
  let p = head;
  while (p) {
    if (set.has(p)) {
      return true;
    }
    set.add(p);
    p = p.next;
  }
  return false;
};