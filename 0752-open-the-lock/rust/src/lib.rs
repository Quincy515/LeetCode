use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let mut deadset = HashSet::new();
        for d in deadends {
            deadset.insert(d);
        }
        if deadset.contains(&"0000".to_string()) {
            return -1;
        }
        let mut queue = VecDeque::new();
        queue.push_back("0000".to_string());
        let mut visited = HashMap::new();
        visited.insert("0000".to_string(), true);
        let mut depth = 0;
        while !queue.is_empty() {
            let size = queue.len();
            let mut k = 0;
            while k < size {
                let node = queue.pop_front().unwrap();
                k += 1;
                if node == target {
                    return depth;
                }
                let node = node.chars().collect::<Vec<char>>();
                let newnodes = gen_new_node(&node);
                for newnode in newnodes {
                    if visited.contains_key(&newnode) || deadset.contains(&newnode) {
                        continue;
                    }
                    queue.push_back(newnode.to_string());
                    visited.insert(newnode, true);
                }
            }
            depth += 1;
        }
        -1
    }
}

fn gen_new_node(node: &[char]) -> Vec<String> {
    let mut newnodes = Vec::new();
    let change: Vec<i32> = vec![-1, 1];
    for i in 0..4 {
        for k in 0..2 {
            let mut new_node = vec![' '; 4];
            new_node[..i].clone_from_slice(&node[..i]);
            new_node[(i + 1)..4].clone_from_slice(&node[(i + 1)..4]);
            let new_char = format!("{}", ((node[i] as u8 - b'0') as i32 + change[k] + 10) % 10);
            let new_char: Vec<char> = new_char.chars().collect();
            new_node[i] = new_char[0];
            newnodes.push(new_node.into_iter().collect::<String>());
        }
    }
    newnodes
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::open_lock(
                vec![
                    "0201".to_owned(),
                    "0101".to_owned(),
                    "0102".to_owned(),
                    "1212".to_owned(),
                    "2002".to_owned()
                ],
                "0202".to_owned()
            ),
            6
        );
        assert_eq!(
            Solution::open_lock(vec!["8888".to_owned(),], "0009".to_owned()),
            1
        );
        assert_eq!(
            Solution::open_lock(
                vec![
                    "8887".to_owned(),
                    "8889".to_owned(),
                    "8878".to_owned(),
                    "8898".to_owned(),
                    "8788".to_owned(),
                    "8988".to_owned(),
                    "7888".to_owned(),
                    "9888".to_owned(),
                ],
                "8888".to_owned()
            ),
            -1
        );
    }
}
