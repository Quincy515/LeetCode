class Solution {
public:
    int insertBits(int N, int M, int i, int j) {
        vector<int> nbits(32);
        vector<int> mbits(32);
        int nbitsNum = 0;
        int mbitsNum = 0;
        long mask = 1;
        for (int k = 0; k < 32; ++k) {
            if ((N & mask) != 0) {
                nbits[k] = 1;
            }
            mask <<= 1;
        }
        mask = 1L;
        for (int k = 0; k < 32; ++k) {
            if ((M & mask) != 0) {
                mbits[k] = 1;
            }
            mask <<= 1;
        }
        for (int k = i; k <= j; ++k) {
            nbits[k] = mbits[k - i];
        }
        mask = 1;
        int ret = 0;
        for (int k = 0; k < 32; ++k) {
            ret += (nbits[k] * mask);
            mask <<= 1;
        }
        return ret;
    }
};
