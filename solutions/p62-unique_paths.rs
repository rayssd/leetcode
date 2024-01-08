impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m == 1 || n == 1 { return 1; }

        let m = m as usize;
        let n = n as usize;

        let mut dp = vec![vec![0;n]; m];

        // initialize the bottom to 1
        for j in (0..n).rev() {
            dp[m-1][j] = 1;
        }
        // initialize the right edge to 1
        for i in (0..m).rev() {
            dp[i][n-1] = 1;
        }

        for i in (0..m-1).rev() {
            for j in (0..n-1).rev() {
                dp[i][j] = dp[i+1][j] + dp[i][j+1];        
            }
        } 

        dp[0][0]
    }
}
