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
    pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        //i shouldnt have to explain this one.
        if root.is_none(){
            return None;
        }
        let root = root.unwrap();
        if root.borrow().val < val{
            Self::search_bst(root.borrow().right.clone(), val)
        }else if root.borrow().val > val{
            Self::search_bst(root.borrow().left.clone(), val)
        }else{
            Some(root)
        }
    }
}