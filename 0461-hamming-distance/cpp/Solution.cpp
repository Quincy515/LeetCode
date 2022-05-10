class Solution {
public:
    int hammingDistance(int x, int y) {
        int r = x ^ y;
        long mask = 1;
        int count = 0;
        for (int i = 0; i < 31; ++i) {
            if ((r & mask) != 0) {
                count++;
            }
            mask *= 2;
        }
        return count;
    }
};
