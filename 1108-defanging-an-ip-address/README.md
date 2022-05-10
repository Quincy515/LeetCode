# 1108. IP 地址无效化
给你一个有效的 [IPv4](https://baike.baidu.com/item/IPv4) 地址 ```address```，返回这个 IP 地址的无效化版本。

所谓无效化 IP 地址，其实就是用 ```"[.]"``` 代替了每个 ```"."```。

#### 示例 1:
<pre>
<strong>输入:</strong> address = "1.1.1.1"
<strong>输出:</strong> "1[.]1[.]1[.]1"
</pre>

#### 示例 2:
<pre>
<strong>输入:</strong> address = "255.100.50.0"
<strong>输出:</strong> "255[.]100[.]50[.]0"
</pre>

#### 提示:
* 给出的 ```address``` 是一个有效的 IPv4 地址

## 题解：
### Rust
```rust
impl Solution {
    pub fn defang_i_paddr1(address: String) -> String {
        address.replace('.', "[.]")
    }

    pub fn defang_i_paddr2(address: String) -> String {
        let mut result = String::new();
        for c in address.chars() {
            if c == '.' {
                result.push_str("[.]");
            } else {
                result.push(c);
            }
        }
        result
    }

    pub fn defang_i_paddr3(address: String) -> String {
        let origin = address.as_bytes();
        let n = origin.len();
        let new_n = n + 2 * 3;
        let mut new = vec![' '; new_n];
        let mut k = 0;
        for i in 0..n {
            if origin[i] != b'.' {
                new[k] = origin[i] as char;
                k += 1;
            } else {
                new[k] = '[';
                k += 1;
                new[k] = '.';
                k += 1;
                new[k] = ']';
                k += 1;
            }
        }
        new.iter().collect::<String>()
    }
}
```

### Go
```go
package main

import "strings"

func defangIPaddr(address string) string {
	return strings.Replace(address, ".", "[.]", -1)
}

func defangIPaddr(address string) string {
	sb := strings.Builder{}
	for i := 0; i < len(address); i++ {
		c := address[i]
		if c != '.' {
			sb.WriteByte(c)
		} else {
			sb.Write([]byte("[.]"))
		}
	}
	return sb.String()
}

//用[]byte 实现 : byte 是 uint8 的别名，占 8 位，所以处理 ASCII 没问题
func defangIPaddr(address string) string {
	origin := []byte(address)
	n := len(origin)
	newN := n + 2*3
	newString := make([]byte, newN)
	k := 0
	for i := 0; i < n; i++ {
		if origin[i] != '.' {
			newString[k] = origin[i]
			k++
		} else {
			newString[k] = '[' //go 中 newString[k++] 编译不通过
			k++
			newString[k] = '.'
			k++
			newString[k] = ']'
			k++
		}
	}
	return string(newString)
}

//用[]rune：rune 是 int32 的别名，占 32 位，4 个字节，如果处理中文字符更适合用 rune
func defangIPaddr(address string) string {
	origin := []rune(address)
	n := len(origin)
	newN := n + 2*3
	newString := make([]rune, newN)
	k := 0
	for i := 0; i < n; i++ {
		if origin[i] != '.' {
			newString[k] = origin[i]
			k++
		} else {
			newString[k] = '['
			k++
			newString[k] = '.'
			k++
			newString[k] = ']'
			k++
		}
	}
	return string(newString)
}

```

### JavaScript
```javascript
/**
 * @param {string} address
 * @return {string}
 */
var defangIPaddr = function(address) {
    return address.replaceAll('.','[.]')
};

/**
 * @param {string} address
 * @return {string}
 */
var defangIPaddr = function(address) {
    let tempAddress = ''
    for (let i=0; i<address.length; i++) {
        if (address[i] != '.') {
            tempAddress += address[i]
        } else {
            tempAddress += '[.]'
        }
    }
    return tempAddress
}

/**
 * @param {string} address
 * @return {string}
 */
var defangIPaddr = function(address) {
    let origin = [...address]
    let n = origin.length
    let newN = n+2*3
    let newString = new Array(newN)
    let k = 0
    for (let i = 0; i < n; i++) {
        if (origin[i] != '.') {
            newString[k] = origin[i]
            k++
        } else {
            newString[k++] = '['
            newString[k++] = '.'
            newString[k++] = ']'
        }
    }
    return newString.join('')
}
```

### Python
```python
class Solution:
    def defangIPaddr(self, address: str) -> str:
        ans = []
        for i in range(len(address)):
            if address[i] != '.':
                ans.append(address[i])
            else:
                ans.append("[.]")
        return "".join(ans)
```

### C++
```c++
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
```