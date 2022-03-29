/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 * this.val = (val===undefined ? 0 : val)
 * this.left = (left===undefined ? null : left)
 * this.right = (right===undefined ? null : right)
 * }
 */
/**
 * @param {TreeNode} root
 * @return {number}
 */
var rob = function(root) {
    let postorder = node => {
        if (node == null) {
            return [0, 0]
        }
        let leftMoney = postorder(node.left)
        let rightMoney = postorder(node.right)
        let money = new Array(2)
        money[0] = Math.max(leftMoney[0], leftMoney[1]) + Math.max(rightMoney[0],
            rightMoney[1])
        money[1] = (leftMoney[0] + rightMoney[0]) + node.val
        return money
    }
    let money = postorder(root)
    return Math.max(money[0], money[1])
};