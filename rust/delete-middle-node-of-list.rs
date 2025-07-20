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
    pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        // i did the tortoise and the hare solution first, but two passes is just faster in safe rust.
        let mut n = 0;
        let mut traveler = head.as_ref();
        while traveler.is_some(){
            n += 1;
            traveler = traveler.unwrap().next.as_ref();
        }

        if n < 2{
            return None;
        }

        let mut traveler = head.as_mut();
        for _ in 1..(n / 2){
            traveler = traveler.unwrap().next.as_mut();
        }

        let mut traveler = traveler.unwrap();
        traveler.next = traveler.next.take().unwrap().next;

        head
    }
}