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
use std::collections::VecDeque;
impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        //just a simple bfs.
        let mut queue = VecDeque::new();
        queue.push_front(root);
        let mut max_sum = i64::MIN;
        let mut max_level = 0i32;
        let mut level = 0i32;
        while !queue.is_empty(){
            level += 1;
            let level_size = queue.len();
            let mut sum = 0i64;
            let mut has_nodes = false;
            for _ in 0..level_size{
                let Some(Some(node_wrap)) = queue.pop_back() else {continue};
                has_nodes = true;
                let node = node_wrap.replace(TreeNode::new(0));
                sum += node.val as i64;

                queue.push_front(node.left);
                queue.push_front(node.right);
            }
            if has_nodes && sum > max_sum{
                max_sum = sum;
                max_level = level;
            }
        }

        max_level as i32
    }
}