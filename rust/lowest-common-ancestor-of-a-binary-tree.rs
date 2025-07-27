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
type Node = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn lowest_common_ancestor(root: Node, p: Node, q: Node) -> Node {
        Self::Lowest_common_ancestor_rec(&root, &p, &q).0.clone()
    }

    //                                                                      returns result node, p_found, q_found
    pub fn Lowest_common_ancestor_rec<'a>(branch: &'a Node, p: &'a Node, q: &'a Node) -> (Node, bool, bool){
        if branch.is_none(){
            return (None, false, false);
        }
        let p_found = branch == p;
        let q_found = branch == q;
        let binding = branch.as_ref().unwrap();
        let branch = binding.borrow();

        let left_result = Self::Lowest_common_ancestor_rec(&branch.left, p, q);
        let right_result = Self::Lowest_common_ancestor_rec(&branch.right, p, q);
        let p_found = p_found || left_result.1 || right_result.1;
        let q_found = q_found || left_result.2 || right_result.2;
        (left_result.0.or(right_result.0.or(if p_found && q_found {Some(binding.clone())} else {None})),
        p_found, q_found)
    }
}