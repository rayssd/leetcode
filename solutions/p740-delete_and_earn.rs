struct Solution;

impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        
        use std::cmp;
        use std::collections::HashMap;

        if nums.len() == 1 {
            return nums[0];
        }
        let mut map = HashMap::new();

        // initialise the map
        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }

        // make a vector, index = nums[i], value = count
        let mut vec = vec![0; *map.keys().max().unwrap() as usize + 1];
        for (key, value) in map.iter() {
            vec[*key as usize] = *value;
        }

        let mut dp = vec![0; vec.len()];

        dp[0] = 0;
        dp[1] = vec[1];

        for i in 2..dp.len() {
            dp[i] = cmp::max(dp[i], vec[i]*i);
             for j in 0..i-1 { 
                dp[i] = cmp::max(dp[i],vec[i]*i + dp[j]);
            }
            
        }
        dp.iter().fold(0, |max, &v| if max < v as i32 { v as i32 } else { max })

    }
}

fn main() {
    dbg!(Solution::delete_and_earn(vec![3,1]));
}
