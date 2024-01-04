impl Solution {
    pub fn find_matrix(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut new_vec: Vec<Vec<i32>> = Vec::new();

        if nums.len() > 0 {
            new_vec.push(vec![nums[0]]);
        }

        for i in 1..nums.len() {
            for row in 0..new_vec.len() + 1 {
                match new_vec.get(row) {
                    Some(_) => {
                        if !new_vec[row].contains(&nums[i]) {
                            new_vec[row].push(nums[i]);
                            break;
                        }
                    }
                    None => {
                        new_vec.push(vec![nums[i]]);
                        break;
                    }
                }
            }
        }

        new_vec
    }
}
