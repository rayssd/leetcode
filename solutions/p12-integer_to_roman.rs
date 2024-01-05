struct Solution;

impl Solution {
    pub fn int_to_roman(mut num: i32) -> String {

        use std::collections::HashMap;

        let mut map = HashMap::new();

        map.insert(1, "I");
        map.insert(4, "IV");
        map.insert(5, "V");
        map.insert(9, "IX");
        map.insert(10, "X");
        map.insert(40, "XL");
        map.insert(50, "L");
        map.insert(90, "XC");
        map.insert(100, "C");
        map.insert(400, "CD");
        map.insert(500, "D");
        map.insert(900, "CM");
        map.insert(1000, "M");
        
        let mut s = String::new();
        while num > 0 {
            match num {
                1000..=3999 => { 
                    num -= 1000;
                    s.push_str(map.get(&1000).unwrap());
                },
                900..=999 => {
                    num -= 900;
                    s.push_str(map.get(&900).unwrap());
                },
                500..=899 => {
                    num -= 500;
                    s.push_str(map.get(&500).unwrap());
                },
                400..=499 => {
                    num -= 400;
                    s.push_str(map.get(&400).unwrap());
                },
                100..=399 => {
                    num -= 100;
                    s.push_str(map.get(&100).unwrap());
                },
                90..=99 => {
                    num -= 90;
                    s.push_str(map.get(&90).unwrap());
                }
                50..=89 => {
                    num -= 50;
                    s.push_str(map.get(&50).unwrap());
                },
                40..=49 => {
                    num -= 40;
                    s.push_str(map.get(&40).unwrap());
                },
                10..=39 => {
                    num -= 10;
                    s.push_str(map.get(&10).unwrap());
                },
                9 => {
                    num -= 9;
                    s.push_str(map.get(&9).unwrap());
                },
                5..=8 => {
                    num -= 5;
                    s.push_str(map.get(&5).unwrap());
                },
                4 => {
                    num -= 4;
                    s.push_str(map.get(&4).unwrap());
                },
                1..=3 => {
                    num -= 1;
                    s.push_str(map.get(&1).unwrap());
                }
                _ => { num = 0; }
            }
        }

        s
        
    }
}
fn main() {
    dbg!(Solution::int_to_roman(1994));
}
