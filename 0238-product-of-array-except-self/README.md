# 238. 除自身以外数组的乘积
给你一个长度为 *n* 的整数数组 `nums`，其中 *n* > 1，返回输出数组 `output` ，其中 `output[i]` 等于 `nums` 中除 `nums[i]` 之外其余各元素的乘积。

## 示例:
<pre>
<b>输入:</b> [1,2,3,4]
<b>输出:</b> [24,12,8,6]
</pre>

**提示:** 题目数据保证数组之中任意元素的全部前缀元素和后缀（甚至是整个数组）的乘积都在 32 位整数范围内。

**说明:** 请**不要使用除法**，且在 O(*n*) 时间复杂度内完成此题。

## 进阶:
你可以在常数空间复杂度内完成这个题目吗？（ 出于对空间复杂度分析的目的，输出数组**不被视为**额外空间。）

## 题解：
### Rust
```rust
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let (mut left_products, mut right_products) = (vec![0; n], vec![0; n]);
        let mut product = 1;
        for i in 0..n {
            product *= nums[i];
            left_products[i] = product;
        }
        product = 1;
        for i in (0..=n - 1).rev() {
            product *= nums[i];
            right_products[i] = product;
        }
        let mut result = vec![0; n];
        for i in 0..n {
            result[i] = 1;
            if i >= 1 {
                result[i] *= left_products[i - 1];
            }
            if i + 1 < n {
                result[i] *= right_products[i + 1];
            }
        }
        result
    }
}

```
除了输⼊输出不使⽤额外的空间
```rust

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![0; n];
        let mut left_product = 1;
        for i in 0..n {
            result[i] = left_product;
            left_product *= nums[i];
        }
        let mut right_product = 1;
        for i in (0..=n - 1).rev() {
            result[i] *= right_product;
            right_product *= nums[i];
        }
        result
    }
}
```
### Go
```go

package main

func productExceptSelf(nums []int) []int {
	n := len(nums)
	leftProducts := make([]int, n)
	rightProducts := make([]int, n)
	product := 1
	for i := 0; i < n; i++ {
		product *= nums[i]
		leftProducts[i] = product
	}
	product = 1
	for i := n - 1; i >= 0; i-- {
		product *= nums[i]
		rightProducts[i] = product
	}
	result := make([]int, n)
	for i := 0; i < n; i++ {
		result[i] = 1
		if i-1 >= 0 {
			result[i] *= leftProducts[i-1]
		}
		if i+1 < n {
			result[i] *= rightProducts[i+1]
		}
	}
	return result
}

func productExceptSelf(nums []int) []int {
	n := len(nums)
	result := make([]int, n)
	leftProduct := 1
	for i := 0; i < n; i++ {
		result[i] = leftProduct
		leftProduct *= nums[i]
	}
	rightProduct := 1
	for i := n - 1; i >= 0; i-- {
		result[i] *= rightProduct
		rightProduct *= nums[i]
	}
	return result
}
```

### JavaScript
```javascript
/**
 * @param {number[]} nums
 * @return {number[]}
 */
var productExceptSelf = function(nums) {
    let n = nums.length
    let leftProducts = new Array(n)
    let rightProducts = new Array(n)
    let product = 1
    for (let i = 0; i < n; ++i) {
        product *= nums[i]
        leftProducts[i] = product
    }
    product = 1
    for (let i = n-1; i >= 0; --i) {
        product *= nums[i]
        rightProducts[i] = product
    }
    let result = new Array(n)
    for (let i = 0; i < n; ++i) {
        result[i] = 1
        if (i-1 >= 0) {
            result[i] *= leftProducts[i-1]
        }
        if (i+1 < n) {
            result[i] *= rightProducts[i+1]
        }
    }
    return result
};

// 除了输⼊输出不使⽤额外的空间

/**
 * @param {number[]} nums
 * @return {number[]}
 */
var productExceptSelf = function(nums) {
    let n = nums.length
    let result = new Array(n)
    let leftProduct = 1
    for (let i = 0; i < n; ++i) {
        result[i] = leftProduct
        leftProduct *= nums[i]
    }
    let rightProduct = 1
    for (let i = n-1; i >= 0; --i) {
        result[i] *= rightProduct
        rightProduct *= nums[i]
    }
    return result
};

```

### Python
```python
class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        n = len(nums)
        leftProducts = [0] * n
        rightProducts = [0] * n
        product = 1
        for i in range(n):
            product *= nums[i]
            leftProducts[i] = product
        product = 1
        for i in range(n-1,-1,-1):
            product *= nums[i]
            rightProducts[i] = product
        result = [None] * n
        for i in range(n):
            result[i] = 1
            if i - 1 >= 0:
                result[i] *= leftProducts[i-1]
            if i + 1 < n:
                result[i] *= rightProducts[i+1]
        return result

class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        n = len(nums)
        result = [None] * n
        leftProduct = 1
        for i in range(n):
            result[i] = leftProduct
            leftProduct *= nums[i]
        rightProduct = 1
        for i in range(n-1,-1,-1):
            result[i] *= rightProduct
            rightProduct *= nums[i]
        return result

```

### C++
```c++
class Solution {
public:
    vector<int> productExceptSelf(vector<int>& nums) {
        int n = nums.size();
        vector<int> leftProduct(n);
        vector<int> rightProduct(n);
        int product = 1;
        for (int i = 0; i < n; ++i) {
            product *= nums[i];
            leftProduct[i] = product;
        }
        product = 1;
        for (int i = n - 1; i >= 0; --i) {
            product *= nums[i];
            rightProduct[i] = product;
        }

        vector<int> result(n);
        for (int i = 0; i < n; ++i) {
            result[i] = 1;
            if (i - 1 >= 0) {
                result[i] *= leftProduct[i - 1];
            }
            if (i + 1 < n) {
                result[i] *= rightProduct[i + 1];
            }
        }
        return result;
    }
};

class Solution {
public:
    vector<int> productExceptSelf(vector<int>& nums) {
        int n = nums.size();
        vector<int> result(n);
        int leftProduct = 1;
        for (int i = 0; i < n; ++i) {
            result[i] = leftProduct;
            leftProduct *= nums[i];
        }
        int rightProduct = 1;
        for (int i = n - 1; i >= 0; --i) {
            result[i] *= rightProduct;
            rightProduct *= nums[i];
        }
        return result;
    }
};

```