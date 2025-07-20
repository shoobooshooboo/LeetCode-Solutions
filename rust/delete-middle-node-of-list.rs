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
        if head.clone().unwrap().next.is_none(){
            return None;
        }
        let mut hare = head.clone().unwrap().next;
        let mut tortoise = head.as_mut();

        loop{
            hare = hare.unwrap().next;
            hare = if hare.is_some() {hare.unwrap().next} else {None};

            if hare.is_none(){
                break;
            }
            tortoise = tortoise.unwrap().next.as_mut();
        }

        let mut tortoise = tortoise.unwrap();
        tortoise.next = tortoise.next.as_mut().unwrap().next.clone();

        head
    }
}