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
    pub fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let binding = root.unwrap();
        let root = binding.borrow();
        //start zigzags to the left and right
        Self::longest_zig_zag_rec(&root.left, 1, false).max(Self::longest_zig_zag_rec(&root.right, 1, true))
    }
    fn longest_zig_zag_rec(root: &Option<Rc<RefCell<TreeNode>>>, steps: i32, go_left: bool) -> i32 {
        if root.is_none(){
            //last branch wasn't real, so remove one
            return steps - 1;
        }
        let binding = root.as_ref().unwrap();
        let root = binding.borrow();
        //handle recursive calls based on if you're going left or right
        if go_left{
            Self::longest_zig_zag_rec(&root.left, steps + 1, false)
            .max(Self::longest_zig_zag_rec(&root.right, 1, true))
        }else{
            Self::longest_zig_zag_rec(&root.right, steps + 1, true)
            .max(Self::longest_zig_zag_rec(&root.left, 1, false))
        }
    }
}