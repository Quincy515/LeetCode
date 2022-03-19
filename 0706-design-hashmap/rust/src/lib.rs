struct MyHashMap {
    value: Vec<i32>,
}

const LIMIT: usize = 1_000_000;

impl MyHashMap {
    fn new() -> Self {
        MyHashMap {
            value: vec![-1; LIMIT + 1],
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        self.value[key as usize] = value;
    }

    fn get(&self, key: i32) -> i32 {
        self.value[key as usize]
    }

    fn remove(&mut self, key: i32) {
        self.value[key as usize] = -1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
