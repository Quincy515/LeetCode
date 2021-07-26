use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() { return false; }

        let mut map = HashMap::new();

        // 计算s中每个字母的数量
        for c in s.chars() {
            // entry 方法会以当前字符作为键检查是否有对应值，没有对应值则使用or_insert方法插入键值对。
            // 返回值是&mut i32类型，将这个返回值与变脸count绑定
            // 通过解引用操作符"*"对count进行赋值
            let count = map.entry(c).or_insert(0);
            *count += 1;
        }

        // 用t减去每个字母的数量
        for c in t.chars() {
            let count = map.entry(c).or_insert(0);
            *count -= 1;
            // 如果计数器小于零，说明t包含一个不在s的额外字母，立即返回False
            if *count < 0 {
                return false;
            }
        }
        return true;
    }
}

fn main() {
    println!("Hello, world!");
}
