impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        use std::cmp;
        // dp will be the resultant vec
        let mut dp = vec![0; cost.len()+1];
        let dp_size = dp.len();

        // initialise the base value
        dp[dp_size-1] = 0;
        dp[dp_size-2] = cost[cost.len()-1];

        for i in (0..dp.len()-2).rev() {
            dp[i] = cmp::min(cost[i] + dp[i+1], cost[i] + dp[i+2]);
        }

        return cmp::min(dp[0], dp[1]);
    }
}
