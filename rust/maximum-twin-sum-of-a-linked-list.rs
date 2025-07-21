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
    pub fn pair_sum(head: Option<Box<ListNode>>) -> i32 {
        Self::pair_sum_rec(&mut head.as_ref(), head.as_ref())
    }

    fn pair_sum_rec(mut left: &mut Option<&Box<ListNode>>, mut right: Option<&Box<ListNode>>) -> i32{
        if right.is_none() || left.is_none(){
            return 0;
        }
        //recursive call first
        let unwrapped_right = right.take().unwrap();
        let mut max = Self::pair_sum_rec(left, unwrapped_right.next.as_ref());
        //then get the new max
        let unwrapped_left = left.take().unwrap();
        max = max.max(unwrapped_left.val + unwrapped_right.val);
        //update left and return
        *left = unwrapped_left.next.as_ref();
        max
    }
}