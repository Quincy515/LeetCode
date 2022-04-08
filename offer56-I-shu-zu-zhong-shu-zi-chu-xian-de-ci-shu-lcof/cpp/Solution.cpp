class Solution {
public:
    vector<int> singleNumbers(vector<int>& nums) {
        int xorResult = 0;
        int n = nums.size();
        for (int i = 0; i < n; ++i) {
            xorResult ^= nums[i];
        }
        int tag = 1;
        while ((xorResult & tag) == 0) {
            tag = tag << 1;
        }
        int a = 0;
        int b = 0;
        for (int i = 0; i < n; ++i) {
            if ((nums[i] & tag) == 0) {
                a ^= nums[i];
            } else {
                b ^= nums[i];
            }
        }
        return {a, b};
    }
};
