/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number[][]}
 */
var pairSums = function (nums, target) {
  let results = [];
  if (nums.length === 0) {
    return results;
  }
  nums.sort((a, b) => a - b);
  let i = 0;
  let j = nums.length - 1;
  while (i < j) {
    if (nums[i] + nums[j] === target) {
      results.push([nums[i], nums[j]]);
      i++;
      j--;
    } else if (nums[i] + nums[j] < target) {
      i++;
    } else {
      j--;
    }
  }
  return results;
};
