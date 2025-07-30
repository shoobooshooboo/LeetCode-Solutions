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
    pub fn delete_node(root: Node, key: i32) -> Node {
        if root.is_none(){
            return None;
        }
        let mut binding = root.unwrap();
        //this part is in a different scope so that node will drop before returnging binding
        {
            let mut node = binding.borrow_mut();
            //search for the node to delete
            if node.val > key{
                node.left = Self::delete_node(node.left.clone(), key);
            }else if node.val < key{
                node.right = Self::delete_node(node.right.clone(), key);
            }else{
                //if the node is found, get the successor and replace it with it's successor.
                if node.right.is_none(){
                    return node.left.clone();
                }
                let successor = Self::find_successor(node.right.clone());
                if let Some(s) = successor{
                    node.val = s.borrow().val;
                    node.right = Self::delete_node(node.right.clone(), node.val);
                }
            }
        }
        
        Some(binding)
    }

    fn find_successor(branch: Node) -> Node{
        if branch.is_none(){
            return None;
        }

        let node = branch.unwrap();
        if node.borrow().left.is_some(){
            Self::find_successor(node.borrow().left.clone())
        }else{
            Some(node)
        }
    }
}