class Solution {
public:
    int findClosest(vector<string>& words, string word1, string word2) {
        vector<int> w1ps;
        vector<int> w2ps;
        for (int i = 0; i < words.size(); ++i) {
            if (words[i] == word1) {
                w1ps.push_back(i);
            } else if (words[i] == word2) {
                w2ps.push_back(i);
            }
        }
        int p1 = 0;
        int p2 = 0;
        int minRet = INT_MAX;
        while (p1 < w1ps.size() && p2 < w2ps.size()) {
            if (w1ps[p1] < w2ps[p2]) {
                if (w2ps[p2] - w1ps[p1] < minRet) minRet = std::min(minRet, w2ps[p2] -
                                                                            w1ps[p1]);
                p1++;
            } else {
                if (w1ps[p1] - w2ps[p2] < minRet) minRet = std::min(minRet, w1ps[p1] -
                                                                            w2ps[p2]);
                p2++;
            }
        }
        return minRet;
    }
};