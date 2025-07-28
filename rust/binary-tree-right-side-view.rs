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
type RefNode<'a> = Option<&'a Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn right_side_view(root: Node) -> Vec<i32> {
        //depth-first-search solution
        //essentially, i'm just traversing the tree to the right first
        //and the first node i find at each depth is the right-most node
        //so i push it to the Vec.
        let mut right_view = Vec::new();
        Self::right_side_view_rec(root.as_ref(), 0, &mut right_view);
        right_view
    }

    fn right_side_view_rec(branch: RefNode, depth: usize, mut right_view: &mut Vec<i32>){
        if branch.is_none(){
            return;
        }
        let branch = branch.unwrap().borrow();
        //since theres one node for each level of the tree,
        //we can use the depth to determine if a node has been added for this depth.
        //if it hasn't, then this is the rightmost node.
        if depth >= right_view.len(){
            right_view.push(branch.val);
        }
        //recursive calls SPECIFICALLY STARTING WITH THE RIGHT BRANCH
        Self::right_side_view_rec(branch.right.as_ref(), depth + 1, right_view);
        Self::right_side_view_rec(branch.left.as_ref(), depth + 1, right_view);
    }
}