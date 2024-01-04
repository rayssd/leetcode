impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if (s.len() <= 1) | (num_rows <= 1) {
            return s;
        }

        enum Direction {
            Up,
            Down,
        }

        let num_rows: usize = num_rows as usize;

        let num_rows = num_rows as usize;

        let v: Vec<char> = s.chars().collect();
        let mut row = 0;
        let mut column = 0;

        let mut zigzag = vec![vec![' '; v.len()]; num_rows];

        let mut direction = Direction::Down;

        for i in 0..v.len() {
            match direction {
                Direction::Down => {
                    zigzag[row][column] = v[i];
                    row += 1;
                    if row >= num_rows {
                        direction = Direction::Up;
                        row = num_rows - 1; // when reached the bottom, reset row to last row
                    }
                }
                Direction::Up => {
                    row -= 1;
                    column += 1;
                    zigzag[row][column] = v[i];
                    if row == 0 {
                        direction = Direction::Down;
                        row = 1; // when reached the top, next write at row 2
                    }
                }
            }
        }

        let mut new_vec = Vec::new();
        for r in zigzag.iter_mut() {
            r.retain(|&c| c != ' ');
            new_vec.append(r);
        }
        let new_string: String = new_vec.iter().collect();
        new_string
    }
}
