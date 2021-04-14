
// Definition for singly-linked list.
function ListNode(val, next) {
  this.val = (val === undefined ? 0 : val)
  this.next = (next === undefined ? null : next)
}

/**
 * @param {ListNode} head
 * @return {boolean}
 */
var isPalindrome = function (head) {
  const stack = [];
  let p = head;
  while (p) {
    stack.push(p);
    p = p.next;
  }
  p = head;
  while (p) {
    if (p.val !== stack.pop().val) {
      return false;
    }
    p = p.next;
  }
  return true;
}

function isPalindromeReverseList(head) {
  let len = 0;
  let p = head;
  while (p) {
    len++;
    p = p.next;
  }

  let cur = head, prev = null;
  const halfLen = Math.floor(len / 2);
  for (let i = 0; i < halfLen; i++) {
    let next = cur.next;
    cur.next = prev;
    prev = cur;
    cur = next;
  }

  if (len % 2 === 1) {
    cur = cur.next;
  }

  while (cur && prev) {
    if (cur.val !== prev.val) {
      return false;
    }
    cur = cur.next;
    prev = prev.next;
  }
  return true;
}