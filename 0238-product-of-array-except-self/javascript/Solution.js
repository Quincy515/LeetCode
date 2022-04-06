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
