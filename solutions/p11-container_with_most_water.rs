struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        use std::cmp;
        let mut largest = 0;

        let mut i = 0;
        let mut j = height.len() - 1;

        while i < j {
            largest = cmp::max(
                largest,
                (j as i32 - i as i32) * cmp::min(height[i], height[j]),
            );
            if height[i] > height[j] {
                j -= 1;
            } else {
                i += 1;
            }
        }
        largest
    }
}

fn main() {
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    dbg!(Solution::max_area(height));
}
