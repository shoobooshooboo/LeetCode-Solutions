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
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        //intuitive, slow solution
        let mut dummy = ListNode::new(0);
        let mut cur = &mut dummy;

        loop{
            let mut next: Option<usize> = None;
            for i in 0..lists.len(){
                if lists[i].is_some(){
                    if next.is_none() || lists[i].clone().unwrap().val < lists[next.clone().unwrap()].clone().unwrap().val{
                        next = Some(i);
                    }
                }
            }

            if next.is_none(){
                break;
            }

            let next = next.unwrap();
            cur.next = lists[next].take();
            cur = cur.next.as_mut().unwrap();
            lists[next] = cur.next.take();
        }

        dummy.next
    }
}