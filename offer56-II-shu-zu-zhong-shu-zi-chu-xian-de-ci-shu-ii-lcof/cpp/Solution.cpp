class Solution {
public:
    int singleNumber(vector<int>& nums) {
        int n = nums.size();
        vector<int> bits(32);
        long mask = 1;
        for (int i = 0; i < 32; ++i) {
            for (int j = 0; j < n; ++j) {
                if ((nums[j] & mask) != 0) {
                    bits[i] = (bits[i] + 1) % 3;
                }
            }
            mask <<= 1;
        }
        int result = 0;
        mask = 1;
        for (int i = 0; i < 32; ++i) {
            if (bits[i] == 1) {
                result += mask;
            }
            mask <<= 1;
        }
        return result;;
    }
};
