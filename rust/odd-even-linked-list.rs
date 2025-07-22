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
    pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        //keep track of an odd tail and an even tail so you can basically unzip the linked list
        //and then append the even list to the odd list
        if head.is_none() || head.as_ref().unwrap().next.is_none(){
            return head;
        }

        //head == odd_head
        let mut odd_tail = head.as_mut().unwrap();
        let mut even_head = odd_tail.next.take().unwrap();
        let mut even_tail = &mut even_head;
        while let Some(odd_node) = even_tail.next.take(){
            odd_tail.next = Some(odd_node);
            odd_tail = odd_tail.next.as_mut().unwrap();

            if let Some(even_node) = odd_tail.next.take(){
                even_tail.next = Some(even_node);
                even_tail = even_tail.next.as_mut().unwrap();
            }
        }

        odd_tail.next = Some(even_head);

        head
    }
}