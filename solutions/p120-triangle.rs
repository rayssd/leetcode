impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        use std::cmp;
        let mut m = triangle.len();

        if m == 1 { return triangle[0][0]; }

        for i in 1..m {
            for j in 0..triangle[i].len() {
                if j == 0 {
                    triangle[i][j] += triangle[i-1][j];
                } else if j == triangle[i].len() - 1 {
                    triangle[i][j] += triangle[i-1][j-1];
                } else {
                    triangle[i][j] += cmp::min(triangle[i-1][j], triangle[i-1][j-1]);
                }
            }
        }

        triangle[m-1].iter().fold(i32::MAX, | min, &v | if min > v { v } else { min })
        
    }
}
