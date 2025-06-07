impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut carry = 0; //carryover
        let mut head = Some(Box::new(ListNode::new(0))); //dummyhead
        let mut tail = head.as_mut(); //current tail of new list

        let (mut l1, mut l2) = (l1.as_ref(), l2.as_ref());
        //while there are more numbers to add
        while l1.is_some() || l2.is_some(){
            //calculate sum
            let sum = if l1.is_some() {l1.unwrap().val} else {0} 
                + if l2.is_some() {l2.unwrap().val} else {0} 
                + carry;
            //update places
            (l1, l2) = (
                if l1.is_some() {l1.unwrap().next.as_ref()} else {None},
                if l2.is_some() {l2.unwrap().next.as_ref()} else {None});

            //store info and move on
            carry = sum / 10;
            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum % 10)));
            tail = tail.unwrap().next.as_mut();
        }

        //deal with extra carry
        if carry > 0{
            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(carry)));
            tail = tail.unwrap().next.as_mut();
        }

        //ignore the dummy head and give them what they actually want
        head.unwrap().next
    }
}