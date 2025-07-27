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
use std::collections::HashMap;
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> i32 {
        //solution kinda like two-sum but with prefix sums and going down a tree.
        let mut sums = HashMap::new();
        //no nodes is one way to reach 1
        sums.insert(0, 1);
        Self::get_sums(root.as_ref(), 0, target, &mut sums)
    }

    fn get_sums(root: Option<&Rc<RefCell<TreeNode>>>, mut prefix: i64, target: i32, mut sums: &mut HashMap<i64, i32>) -> i32{
        if root.is_none(){
            return 0;
        }

        //update prefix
        let root = root.unwrap().borrow();
        prefix += root.val as i64;

        //update count
        let mut count = *sums.get(&(prefix - target as i64)).unwrap_or(&0);
        //increment the current prefix entry in sums
        *sums.entry(prefix).or_insert(0i32) += 1;

        //recursive calls to tally up counts
        count += Self::get_sums(root.left.as_ref(), prefix, target, sums)
               + Self::get_sums(root.right.as_ref(), prefix, target, sums);
        //decrement current prefix entry in sums (cause it's no long available)
        *sums.entry(prefix).or_insert(0i32) -= 1;
        count
    }
}