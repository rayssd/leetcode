impl Solution {
    pub fn my_atoi(mut s: String) -> i32 {
        // sanitise input, remove all white spaces
        s = s.trim_start().to_string();

        // get sign
        let sign = match s.chars().nth(0) {
            Some(c) => match c {
                '-' => {
                    s.remove(0);
                    '-'
                }
                '+' => {
                    s.remove(0);
                    '+'
                }
                _ => '+',
            },
            None => '#',
        };

        if (sign != '+') && (sign != '-') {
            return 0;
        }

        // sanitise string further by removing non-integers
        let int = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        let mut result: String = String::new();
        let mut i: usize = 0;

        while i < s.len() {
            match s.chars().nth(i) {
                Some(c) => {
                    if int.contains(&c) {
                        result.push(c);
                    } else {
                        break;
                    }
                }
                None => println!("End of string"),
            }
            i += 1;
        }

        if result.len() == 0 {
            return 0;
        }

        // clamp
        match result.parse::<i32>() {
            Ok(res) => match sign {
                '+' => res,
                '-' => res - 2 * res,
                _ => 0,
            },
            Err(_) => match sign {
                '+' => i32::MAX,
                '-' => i32::MIN,
                _ => 0,
            },
        }
    }
}
