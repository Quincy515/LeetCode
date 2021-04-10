class Solution {
  // Time: O(n), Space: O(1)
  public boolean canJump(int[] nums) {
  if (nums == null || nums.length == 0) return false;
    int n = nums.length, max = 0;
    for (int i = 0; i < n; ++i) {
      if (max >= n-1) return true;
      if (i > max) return false;
      max = Math.max(max, i+nums[i]);
    }
    return false;
  }
}