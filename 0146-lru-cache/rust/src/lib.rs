#![allow(dead_code, unused)]
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

/// 具体实现 参考 https://github.com/jeromefroe/lru-rs/blob/master/src/lib.result
/// Least Recently Used, 最近最少使用, 关键在于追踪每一个 entry 的 age, 每次淘汰最小的那一个 key
/// 假如淘汰逻辑要做到 O(1) 复杂度, 我们可以引入一个链表, 每次 touch 一个值时, 就删掉它重新 push_back, 而当达到容量要驱逐时, 则 pop_front
/// Rust 的链表不支持根据引用删除任意元素，也没有 LinkedHashMap，需要自己实现一个

// LRU 上的元素项
struct Entry<K, V> {
    key: K,
    val: Option<V>,
    next: Option<usize>,
    prev: Option<usize>,
}

struct LruCache<K, V> {
    cap: usize,
    head: Option<usize>,
    tail: Option<usize>,
    map: HashMap<K, usize>,
    entries: Vec<Entry<K, V>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl<K: Clone + Hash + Eq + Display, V> LruCache<K, V> {
    fn new(cap: usize) -> Self {
        Self::with_capacity(cap)
    }

    fn with_capacity(cap: usize) -> Self {
        LruCache {
            cap,
            head: None,
            tail: None,
            map: HashMap::with_capacity(cap),
            entries: Vec::with_capacity(cap),
        }
    }

    fn insert(&mut self, key: K, val: V) -> Option<V> {
        if self.map.contains_key(&key) {
            self.access(&key);
            let entry = &mut self.entries[self.head.unwrap()];
            let old_val = entry.val.take();
            entry.val = Some(val);
            old_val
        } else {
            println!("key: {}", key);
            self.ensure_room();

            // 更新原始头指针
            let index = self.entries.len();
            self.head.map(|e| {
                self.entries[e].prev = Some(index);
            });

            // 新的头结点
            self.entries.push(Entry {
                key: key.clone(),
                val: Some(val),
                prev: None,
                next: self.head,
            });
            self.head = Some(index);
            self.tail = self.tail.or(self.head);
            self.map.insert(key, index);

            None
        }
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.map.remove(&key).map(|index| {
            self.remove_from_list(index);
            self.entries[index].val.take().unwrap()
        })
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        if self.contains(key) {
            self.access(key);
        }

        let entries = &self.entries;
        self.map
            .get(key)
            .and_then(move |&i| entries[i].val.as_ref())
    }

    fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        if self.contains(key) {
            self.access(key);
        }

        let entries = &mut self.entries;
        self.map
            .get(key)
            .and_then(move |&i| entries[i].val.as_mut())
    }

    fn contains(&mut self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    fn is_full(&self) -> bool {
        self.map.len() == self.cap
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    // 获取某个 key 的值，移除原来位置的值并在头部加入
    fn access(&mut self, key: &K) {
        let i = *self.map.get(key).unwrap();
        self.remove_from_list(i);
        self.head = Some(i);
    }

    fn remove_from_list(&mut self, i: usize) {
        let (prev, next) = {
            let entry = self.entries.get_mut(i).unwrap();
            (entry.prev, entry.next)
        };

        match (prev, next) {
            // 数据项在缓存中间
            (Some(j), Some(k)) => {
                let head = &mut self.entries[j];
                head.next = next;
                let next = &mut self.entries[k];
                next.prev = prev;
            }
            // 数据项在缓存末尾
            (Some(j), None) => {
                let head = &mut self.entries[j];
                head.next = None;
                self.tail = prev;
            }
            // 数据项在缓存头部
            _ => {
                if self.len() > 1 {
                    let head = &mut self.entries[0];
                    head.next = None;
                    let next = &mut self.entries[1];
                    next.prev = None;
                }
            }
        }
    }

    // 确保容量足够，满了就移除末尾的元素
    fn ensure_room(&mut self) {
        println!("ensure_room cap: {}, len: {}", self.cap, self.len());
        if self.cap == self.len() {
            self.remove_tail();
        }
    }

    fn remove_tail(&mut self) {
        if let Some(index) = self.tail {
            self.remove_from_list(index);
            let key = &self.entries[index].key;
            println!("remove_tail index: {} key: {}", index, key);
            self.map.remove(key);
        }

        if self.tail.is_none() {
            self.head = None;
        }
    }
}

struct LRUCache {
    cache: LruCache<i32, i32>,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        Self {
            cache: LruCache::new(capacity as usize),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(v) = self.cache.get(&key) {
            *v
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        self.cache.insert(key, value);
    }

    fn length(&self) -> usize {
        self.cache.len()
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut obj = LRUCache::new(2);
        assert_eq!(obj.get(2), -1);
        obj.put(2, 6);
        assert_eq!(obj.get(1), -1);
        obj.put(1, 5);
        obj.put(1, 2);
        assert_eq!(obj.get(1), 2);
        assert_eq!(obj.get(2), 6);

        let mut obj = LRUCache::new(3);
        obj.put(1, 1);
        obj.put(2, 2);
        obj.put(3, 3);
        obj.put(4, 4);
        assert_eq!(obj.get(4), 4);
        assert_eq!(obj.get(3), 3);
        assert_eq!(obj.get(2), 2);
        assert_eq!(obj.get(1), -1);
        obj.put(5, 5);
        assert_eq!(obj.get(1), -1);
        assert_eq!(obj.get(2), 2);
        assert_eq!(obj.get(3), 3);
        assert_eq!(obj.get(4), -1);
        assert_eq!(obj.get(5), 5);
    }

    #[test]
    fn it_works2() {
        let mut obj = LRUCache::new(2);
        obj.put(1, 1);
        obj.put(2, 2);
        assert_eq!(obj.get(1), 1);
        obj.put(3, 3);
        assert_eq!(obj.get(2), -1);
        obj.put(4, 4);
        println!("len: {}", obj.length());
        assert_eq!(obj.get(1), -1);
        assert_eq!(obj.get(3), 3);
        assert_eq!(obj.get(4), 4);
    }
}
