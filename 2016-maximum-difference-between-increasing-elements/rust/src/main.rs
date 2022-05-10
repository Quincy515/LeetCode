struct Solution;

impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut max_diff = 0;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if i < j && nums[j] - nums[i] > max_diff {
                    max_diff = nums[j] - nums[i];
                }
            }
        }
        if max_diff == 0 {
            return -1;
        }    
        max_diff
    }
}

fn main() {
    println!("{}", Solution::maximum_difference(vec![7, 1, 5, 4]));
    println!(
        "{}",
        Solution::maximum_difference(vec![
            999, 997, 980, 976, 948, 940, 938, 928, 924, 917, 907, 907, 881, 878, 864, 862, 859,
            857, 848, 840, 824, 824, 824, 805, 802, 798, 788, 777, 775, 766, 755, 748, 735, 732,
            727, 705, 700, 697, 693, 679, 676, 644, 634, 624, 599, 596, 588, 583, 562, 558, 553,
            539, 537, 536, 509, 491, 485, 483, 454, 449, 438, 425, 403, 368, 345, 327, 287, 285,
            270, 263, 255, 248, 235, 234, 224, 221, 201, 189, 187, 183, 179, 168, 155, 153, 150,
            144, 107, 102, 102, 87, 80, 57, 55, 49, 48, 45, 26, 26, 23, 15
        ])
    );
}
