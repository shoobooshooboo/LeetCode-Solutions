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
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut traveler = &head;
        
        loop{
            if traveler.is_none(){
                break;
            }
            traveler = &traveler.as_ref().unwrap().next;
            len += 1;
        }

        let mut traveler = &mut head;
        for _ in 0..(len - n){
            traveler = &mut traveler.as_mut().unwrap().next;
        }

        *traveler = traveler.as_mut().unwrap().next.clone(); 

        head
    }
}