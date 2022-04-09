// 写法⼀：使⽤string
class Solution {
public:
    string defangIPaddr(string s) {
        string res;
        for (int i = 0; i < s.size(); ++i) {
            if (s[i] != '.') {
                res += s[i];
            } else {
                res += "[.]";
            }
        }
        return res;
    }
};

// 写法⼆：使⽤数组
class Solution {
public:
    string defangIPaddr(string s) {
        // 栈内存, ⾃动释放
        char str[s.size() + 2 * 3 + 1];// 注意：要多留⼀个位置给结束符
        int k = 0;
        for (int i = 0; i < s.size(); ++i) {
            if (s[i] != '.') {
                str[k++] = s[i];
            } else {
                str[k++] = '[';
                str[k++] = '.';
                str[k++] = ']';
            }
        }
        str[k] = '\0'; // 必须有结束符
        return str;
    }
};

// 写法三: 使⽤数组
class Solution {
public:
    string defangIPaddr(string s) {
        // 堆内存, 需要最后⼿动释放
        char *str = new char[s.size() + 2 * 3 + 1];
        int k = 0;
        for (int i = 0; i < s.size(); ++i) {
            if (s[i] != '.') {
                str[k++] = s[i];
            } else {
                str[k++] = '[';
                str[k++] = '.';
                str[k++] = ']';
            }
        }
        str[k] = '\0';
        string res(str);
        delete[] str;// 记得释放指针
        return res;
    }
};

// 写法四: 使⽤vector<char>
class Solution {
public:
    string defangIPaddr(string s) {
        vector<char> vec;
        for (int i = 0; i < s.size(); ++i) {
            if (s[i] != '.') {
                vec.push_back(s[i]);
            } else {
                vec.push_back('[');
                vec.push_back('.');
                vec.push_back(']');
            }
        }
        string str(vec.begin(), vec.end());
        return str;
    }
}

