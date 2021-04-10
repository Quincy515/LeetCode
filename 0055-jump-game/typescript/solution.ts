function canJump(nums: number[]): boolean {
    let max: number = 0;
    let n: number = nums.length;
    for (let i = 0; i < n; i++) {
        if (max >= n - 1) return true;
        if (i > max) return false;
        max = Math.max(max, i + nums[i]);
    }
    return false;
};