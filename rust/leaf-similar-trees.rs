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
    pub fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        //literally just generating the leaf sequences and then comparing them
        //fine for the small scale we're given, but scales up poorly
        let (mut seq1, mut seq2) = (Vec::new(), Vec::new());
        Self::leaf_seq(root1, &mut seq1);
        Self::leaf_seq(root2, &mut seq2);
        println!("{:?}", seq1);
        println!("{:?}", seq2);
        seq1 == seq2
    }

    fn leaf_seq(root: Option<Rc<RefCell<TreeNode>>>, mut seq: &mut Vec<i32>){
        if root.is_none(){ return; }
        let root = root.unwrap().replace(TreeNode::new(0));
        if root.left.is_none() && root.right.is_none(){
            seq.push(root.val);
        }else{
            Self::leaf_seq(root.left, seq);
            Self::leaf_seq(root.right, seq);
        }
    }
}