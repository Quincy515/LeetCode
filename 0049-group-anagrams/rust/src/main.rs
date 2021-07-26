use std::collections::HashMap;

struct Solution;
// 对于每个字符串，比较每个字符出现的次数是否相等，若相等就将其放入一个数组中
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut vecs: Vec<Vec<String>> = Vec::new();
        let mut used: Vec<bool> = vec![false; strs.len()];

        for i in 0..strs.len() {
            let mut temp: Vec<String> = Vec::new();
            if !used[i] {
                temp.push(strs[i].clone());
                for j in i + 1..strs.len() {
                    let mut is_anagram: bool = true;
                    if strs[i].len() != strs[j].len() {
                        continue;
                    }

                    let mut map = HashMap::new();

                    // 计算strs[i]中每个字母的数量
                    for c in strs[i].chars() {
                        let count = map.entry(c).or_insert(0);
                        *count += 1;
                    }

                    // 用strs[j]减少每个字母的数量
                    for c in strs[j].chars() {
                        let count = map.entry(c).or_insert(0);
                        *count -= 1;
                        // 如果计数器小于零，说明strs[j]包含一个不在strs[i]的字母，立即结束剩余字母的比较
                        if *count < 0 {
                            is_anagram = false;
                            break;
                        }
                    }
                    // 如果是异位词，将该字符串标记为易用，同时加入动态数组
                    if is_anagram {
                        used[j] = true;
                        temp.push(strs[j].clone());
                    }
                }
            }
            if !temp.is_empty() {
                vecs.push(temp);
            }
        }
        return vecs;
    }
}

fn main() {
    let eat = String::from("eat");
    let tea = String::from("tea");
    let tan = String::from("tan");
    let ate = String::from("ate");
    let nat = String::from("nat");
    let bat = String::from("bat");
    let strs: Vec<String> = vec![eat, tea, tan, ate, nat, bat];

    println!("{:?}", Solution::group_anagrams(strs));
}

// 引入哈希表来优化程序。将每个字符串转换为由字符出现次数组成的字符串，其共由26个非负整数组成，分别表示26个字母的统计数量。
// 字符数之间用"#"分隔，比如"abbccc"表示为"1#2#3#0#0#..#0"。再对这些由字符数组成的字符串进行比较，相同的就放入一个动态数组中。

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut vecs: Vec<Vec<String>> = Vec::new();
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for i in 0..strs.len() {
            // 将字符串转换为字符计数
            let mut count = [0; 26];
            for c in strs[i].chars() {
                let index = (c as u32 - 'a' as u32) as usize;
                count[index] += 1;
            }


            // 字符用“#” 分隔组成字符串
            let mut chars = vec![];
            for i in 0..count.len() {
                chars.push(count[i].to_string() + "#");
            }
            let key: String = chars.into_iter().collect();

            // 以26个字母字符数与"#"组成的字符串为键在 HashMap 中进行查找
            let value = map.get(&key);
            if value != None {
                // 找到对应值（字符串动态数组），将当前字符串压入并更新HashMap的键值对
                let mut v = value.unwrap().to_vec();
                v.push(strs[i].clone());
                map.insert(key, v);
            } else {
                // 未找到对应值，创建以当前字符串初始化的动态数组，并组成键值对插入HashMap
                let v = vec![strs[i].clone()];
                map.insert(key, v);
            }
        }
        // 迭代HashMap的所有值，每个值对应一个异位词
        for val in map.values() {
            vecs.push((*val).clone());
        }
        return vecs;
    }
}

// 再次优化程序，将每个字符串按字母顺序排序后，以字符串分类的方式维护HashMap，其中键是一个经过排序的字符串，
// 值是原始字符串的动态数组。动态数组中每个字符串经过排序后都与键相等
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut vecs: Vec<Vec<String>> = Vec::new();
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for i in 0..strs.len() {
            // 将字符串转换为字符数组并对其按字母顺序排序
            let mut chars = vec![];
            for c in strs[i].chars() {
                chars.push(c);
            }
            chars.sort();

            // 将已排序的字符数组转换为字符串
            let key: String = chars.into_iter().collect();
            // 以字母有序的字符串为键在 HashMap 中进行查找
            let value = map.get(&key);
            if value != None {
                // 找到对应值（字符串动态数组），将原始字符串压入并更新HashMap的键值对
                let mut v = value.unwrap().to_vec();
                v.push(strs[i].clone());
                map.insert(key, v);
            } else {
                // 未找到对应值，创建以原始字符串初始化的动态数组，并组成键值对插入HashMap
                let v = vec![strs[i].clone()];
                map.insert(key, v);
            }
        }

        for val in map.values() {
            vecs.push(val.to_vec());
        }

        return vecs;
    }
}
