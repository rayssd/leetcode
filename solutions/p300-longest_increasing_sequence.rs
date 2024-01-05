struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut res = vec![1; nums.len()];
        //dbg!(&res);

        for j in 1..nums.len() {
            for i in 0..j {
                if nums[i] < nums[j] && res[i] + 1 > res[j] {
                    res[j] = res[i] + 1;
                }
            }
        }

        let mut biggest = res[0];
        for i in 1..res.len() {
            if res[i] > biggest {
                biggest = res[i];
            }
            
        }
        biggest
        
    }
}

fn main() {
    let nums = vec![10,9,2,5,3,7,101,18];
    dbg!(Solution::length_of_lis(nums));
}
