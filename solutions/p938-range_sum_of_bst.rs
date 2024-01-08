// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        
        let mut sum = 0;
        
        let mut stack = Vec::new();
        let mut current = root;

        while current.is_some() || !stack.is_empty() {
            while let Some(node) = current {
                stack.push(node.clone());
                current = node.borrow().left.clone();
            }

            current = stack.pop();
            if let Some(node) = current {
                if low <= node.borrow().val && high >=node.borrow().val {
                    sum += node.borrow().val;
                }
                current = node.borrow().right.clone();
            }
        }
      
        sum
    }

}
