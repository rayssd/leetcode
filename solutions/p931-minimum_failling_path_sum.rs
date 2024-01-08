struct Solution;

impl Solution {
    pub fn min_falling_path_sum(mut matrix: Vec<Vec<i32>>) -> i32 {
        use std::cmp;

        let num_of_rows = matrix.len();
        let num_of_columns = matrix[0].len();

        if num_of_rows == 1 || num_of_columns == 1 {
            return matrix.iter().flatten().fold(i32::MAX, |min, &v| if min > v { v } else { min });
        }
        
        for i in 1..num_of_rows {
            for j in 0..num_of_columns {
                if j == 0 {
                    matrix[i][j] += cmp::min(matrix[i-1][j], matrix[i-1][j+1]);
                } else if j == num_of_columns - 1 {
                    matrix[i][j] += cmp::min(matrix[i-1][j-1], matrix[i-1][j]);
                } else { 
                    matrix[i][j] += cmp::min(cmp::min(matrix[i-1][j-1], matrix[i-1][j]), matrix[i-1][j+1]);
                }
            }
        }
        matrix[num_of_rows-1].iter().fold(i32::MAX, |min, &v| if min > v { v } else { min })
    }
}

fn main() {
    let matrix = vec![vec![2,1,3],vec![6,5,4],vec![7,8,9]];
    dbg!(Solution::min_falling_path_sum(matrix));

}
