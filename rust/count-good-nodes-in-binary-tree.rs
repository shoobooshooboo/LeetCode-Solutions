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
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        //this one feels pretty straightforward
        Self::good_nodes_rec(root, i32::MIN)
    }

    fn good_nodes_rec(branch: Option<Rc<RefCell<TreeNode>>>, mut max: i32) -> i32{
        if branch.is_none(){
            return 0;
        }
        let branch = branch.unwrap().replace(TreeNode::new(0));
        //get new max
        max = max.max(branch.val);
        //if this node is == to new max, it's a good node
        (if branch.val == max {1} else {0} )
        + Self::good_nodes_rec(branch.left, max)
        + Self::good_nodes_rec(branch.right, max)
    }
}