impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        use std::cmp;

        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();

        // solve edge cases
        if (m == 1 && obstacle_grid[0].contains(&1)) || obstacle_grid[m-1][n-1] == 1 {
            return 0;
        }

        if n == 1 {
            for i in 0..m {
                if obstacle_grid[i][0] == 1 {
                    return 0;
                }
            }
        }

        let mut dp = vec![vec![0;n]; m];

        // initialize the bottom to 1, 0 to mark obstacles
        for j in (0..n).rev() {
            if obstacle_grid[m-1][j] != 1 {
                dp[m-1][j] = 1;
            } else {
                dp[m-1][j] = 0;
            }
        }
        // initialize the right edge to 1, 0 to mark obstacles
        for i in (0..m).rev() {
            if obstacle_grid[i][n-1] != 1 {
                dp[i][n-1] = 1;
            } else {
                dp[i][n-1] = 0;
            }
        }

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if i == m - 1 && j == n - 1 {
                    dp[i][j] = 1
                } else if obstacle_grid[i][j] == 1 {
                    dp[i][j] = 0;
                } else {
                    if i + 1 >= m {
                        dp[i][j] = dp[i][j+1];
                    } else if j + 1 >= n {
                        dp[i][j] = dp[i+1][j];
                    } else {
                        dp[i][j] = dp[i+1][j] + dp[i][j+1]; 
                    }
                     
                }       
            }
        } 

        dp[0][0]
    }
}
