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
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur = &mut head;
        //while there's a pair
        while cur.is_some() && cur.as_ref().unwrap().next.is_some(){
            //get the next node
            let mut n1 = cur.as_mut().unwrap().next.take(); 
            //and the one after it
            let n2 = n1.as_mut().unwrap().next.take();
            //make the current point to the one two ahead
            cur.as_mut().unwrap().next = n2;
            //make the old next node point to the curent node
            n1.as_mut().unwrap().next = cur.take();
            //replace the old current with the new current
            cur.replace(n1.unwrap());

            //advance to the next pair
            cur = &mut cur.as_mut().unwrap().next.as_mut().unwrap().next;
        }

        head
    }
}