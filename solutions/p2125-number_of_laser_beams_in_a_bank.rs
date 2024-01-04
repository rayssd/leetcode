impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        if bank.len() <= 1 {
            return 0;
        }
        // convert bank: Vec<String> to 2d vectors

        let mut lasers: Vec<Vec<char>> = Vec::new();
        for i in 0..bank.len() {
            lasers.push(bank[i].chars().collect());
        }

        // find number of 1's in the row
        let mut count: Vec<usize> = Vec::new();
        for i in 0..lasers.len() {
            let temp = lasers[i].iter().filter(|&x| *x == '1').count();
            if temp != 0 {
                count.push(temp);
            }
        }
        let mut product: usize = 0;
        let mut i = 0;

        if count.len() == 0 {
            0
        } else {
            while i < count.len() - 1 {
                product += count[i] * count[i + 1];
                i += 1;
            }

            return product as i32;
        }
    }
}
