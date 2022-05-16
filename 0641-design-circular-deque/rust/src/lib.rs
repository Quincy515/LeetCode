struct MyCircularDeque {
    val: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCircularDeque {
    fn new(k: i32) -> Self {
        Self {
            val: Vec::with_capacity(k as usize),
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.val.len() >= self.val.capacity() {
            return false;
        }
        self.val.insert(0, value);
        true
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.val.len() >= self.val.capacity() {
            return false;
        }
        self.val.push(value);
        true
    }

    fn delete_front(&mut self) -> bool {
        if !self.val.is_empty() {
            self.val.remove(0);
            return true;
        }
        false
    }

    fn delete_last(&mut self) -> bool {
        if !self.val.is_empty() {
            self.val.pop();
            return true;
        }
        false
    }

    fn get_front(&self) -> i32 {
        if self.val.is_empty() {
            return -1;
        }
        self.val[0]
    }

    fn get_rear(&self) -> i32 {
        if self.val.is_empty() {
            return -1;
        }
        *self.val.last().unwrap()
    }

    fn is_empty(&self) -> bool {
        self.val.is_empty()
    }

    fn is_full(&self) -> bool {
        self.val.len() == self.val.capacity()
    }
}

/**
 * Your MyCircularDeque object will be instantiated and called as such:
 * let obj = MyCircularDeque::new(k);
 * let ret_1: bool = obj.insert_front(value);
 * let ret_2: bool = obj.insert_last(value);
 * let ret_3: bool = obj.delete_front();
 * let ret_4: bool = obj.delete_last();
 * let ret_5: i32 = obj.get_front();
 * let ret_6: i32 = obj.get_rear();
 * let ret_7: bool = obj.is_empty();
 * let ret_8: bool = obj.is_full();
 */

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
