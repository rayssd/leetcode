impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        
        use std::cmp;

        let m = grid.len();
        let n = grid[0].len();

        // deal with special cases

        if m == 1 && n == 1 {
            return grid[0][0];
        }

        let mut sum = 0;

        if m == 1 {
            for i in 0..n {
                sum += grid[0][i];
            }
            return sum;
        }

        if n == 1 {
            for i in 0..m {
                sum += grid[i][0];
            } 
            return sum; 
        }
        
        // when m > 1 && n > 1

        let mut dp = vec![vec![0;n]; m];

        dp[0][0] = grid[0][0];


        // initialize the top row
        for j in 1..n {
            dp[0][j] = dp[0][j-1] + grid[0][j];
        }
        // initialize the left edge
        for i in 1..m {
            dp[i][0] = dp[i-1][0] + grid[i][0];
        }

        for i in 1..m {
            for j in 1..n {
                dp[i][j] = cmp::min(dp[i-1][j], dp[i][j-1]) + grid[i][j];
            }
        }

        dp[m-1][n-1]


    }
}
