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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        //really easy recursive solution. The tree is built stupid, though.
        //Really doesnt need ot be Rc<RefCell> and all that. could just be
        //Option<Box<TreeNode>>
        if root.is_none(){
            return 0;
        }
        //because it's stupid, i need to generate a new TreeNode to replace the root node.
        let root = root.unwrap().replace(TreeNode::new(0));
        Self::max_depth(root.left).max(Self::max_depth(root.right)) + 1
    }
}