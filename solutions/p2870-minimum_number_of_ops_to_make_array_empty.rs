impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        for i in 0..nums.len() {
            *map.entry(nums[i]).or_insert(0) += 1;
        }

        let mut n_ops: i32 = 0;

        for (_, (_, value)) in map.iter().enumerate() {
            let mut temp_val = value.clone();
            while temp_val > 0 {
                dbg!(temp_val);
                if temp_val == 1 {
                    return -1;
                }
                if temp_val % 3 == 0 {
                    n_ops += temp_val / 3;
                    break;
                } else if temp_val % 3 == 1 && temp_val != 4 {
                    n_ops += temp_val / 3 - 1;
                    temp_val -= (temp_val / 3 - 1) * 3;
                } else if temp_val % 3 == 2 {
                    n_ops += temp_val / 3 + 1;
                    break;
                } else if temp_val == 4 {
                    n_ops += 2;
                    break;
                }
            }
        }

        n_ops
    }
}
