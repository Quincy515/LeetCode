struct Solution;

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        let max_n = customers.len().max(grumpy.len()) + 1;
        let (mut cust_sum, mut cust_gru) = (vec![0; max_n], vec![0; max_n]);
        let n = customers.len();
        for i in 1..=n {
            let index = i - 1;
            cust_sum[i] = cust_sum[i - 1] + customers[index];
            cust_gru[i] = cust_gru[i - 1] + (1 - grumpy[index]) * customers[index]
        }
        println!("cust_sum: {:?}, cust_gru: {:?}", cust_sum, cust_gru);
        let (mut i, mut j, mut result) = (1, 0, 0);
        while j < n {
            j += 1;
            while j - i + 1 > minutes as usize {
                i += 1;
            }
            if j - i + 1 == minutes as usize {
                let val =
                    cust_gru[n] - (cust_gru[j] - cust_gru[i - 1]) + cust_sum[j] - cust_sum[i - 1];
                result = result.max(val);
            }
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(
            Solution::max_satisfied(
                vec![1, 0, 1, 2, 1, 1, 7, 5],
                vec![0, 1, 0, 1, 0, 1, 0, 1],
                3
            ),
            16
        );
        assert_eq!(Solution::max_satisfied(vec![1], vec![0], 1), 1);
    }
}
