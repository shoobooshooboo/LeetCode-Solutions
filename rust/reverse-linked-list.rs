// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        //simple solution. literally just reversing the direction of the "arrows" 
        //of the linked list.
        let mut prev = None;
        while let Some(mut node) = head.take(){
            //keep track of the next node
            let next = node.next.take();
            // make the current node point at the previous one
            node.next = prev.take();
            //move prev and cur forward one
            prev = Some(node);
            head = next;
        }

        prev
    }
}