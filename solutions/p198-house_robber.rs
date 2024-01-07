impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 { return nums[0]; }
        use std::cmp;

        let mut dp: Vec<i32> = vec![0; nums.len()];

        // initialize the first two values of dp
        dp[0] = nums[0];
        dp[1] = nums[1];

        for i in 2..dp.len() { 
            dp[i] = cmp::max(dp[i],nums[i]);
            for j in 0..i-1 { 
                dp[i] = cmp::max(dp[i],nums[i] + dp[j]);
            }
        } 

        dp.iter().fold(0, |max, &v| if max < v { v } else { max })
    }
}
