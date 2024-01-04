// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

fn get_sum(vec1: Vec<i32>, vec2: Vec<i32>) -> Vec<i32> {
    let length = if vec1.len() > vec2.len() {
        vec1.len()
    } else {
        vec2.len()
    };
    let mut vec_sum: Vec<i32> = vec![0; length + 1];

    let mut index = 0;
    let mut carry = 0;
    let mut sum;

    while index < vec1.len() && index < vec2.len() {
        sum = vec1[index] + vec2[index] + carry;
        if sum < 10 {
            vec_sum[index] = sum;
            carry = 0;
        } else {
            vec_sum[index] = sum % 10;
            carry = 1;
        }
        index += 1;
    }

    if index == vec1.len() && index <= vec2.len() {
        for i in index..vec2.len() {
            sum = vec2[i] + carry;
            if sum < 10 {
                vec_sum[i] = sum;
                carry = 0;
            } else {
                vec_sum[i] = sum % 10;
                carry = 1;
            }
        }
    } else if index == vec2.len() && index <= vec1.len() {
        for i in index..vec1.len() {
            sum = vec1[i] + carry;
            if sum < 10 {
                vec_sum[i] = sum;
                carry = 0;
            } else {
                vec_sum[i] = sum % 10;
                carry = 1;
            }
        }
    }

    if carry == 1 {
        vec_sum[length] = 1;
    } else if vec_sum[length] == 0 && vec_sum.len() > 1 {
        vec_sum.pop();
    }

    vec_sum
}

fn to_vec(mut list: Option<Box<ListNode>>) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();

    while list.is_some() {
        vec.push(list.as_ref().unwrap().val);
        list = list.unwrap().next;
    }

    vec
}

fn make_list(digits: Vec<i32>) -> Option<Box<ListNode>> {
    if digits.is_empty() {
        return Some(Box::new(ListNode::new(0)));
    } else if digits.len() == 1 {
        return Some(Box::new(ListNode::new(digits[digits.len() - 1])));
    }

    let tail = Some(Box::new(ListNode::new(digits[digits.len() - 1])));
    let mut current = Some(Box::new(ListNode {
        val: digits[digits.len() - 2],
        next: tail,
    }));

    for i in (0..digits.len() - 2).rev() {
        let next = Some(Box::new(ListNode {
            val: digits[i],
            next: current,
        }));
        current = next;
    }

    return current;
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let v1 = to_vec(l1);
        let v2 = to_vec(l2);
        let sum = get_sum(v1, v2);
        make_list(sum)
    }
}
