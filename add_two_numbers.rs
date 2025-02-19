// You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.

// You may assume the two numbers do not contain any leading zero, except the number 0 itself.

 

// Example 1:


// Input: l1 = [2,4,3], l2 = [5,6,4]
// Output: [7,0,8]
// Explanation: 342 + 465 = 807.
// Example 2:

// Input: l1 = [0], l2 = [0]
// Output: [0]
// Example 3:

// Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
// Output: [8,9,9,9,0,0,0,1]
 

// Constraints:

// The number of nodes in each linked list is in the range [1, 100].
// 0 <= Node.val <= 9
// It is guaranteed that the list represents a number that does not have leading zeros.


// Traversing a linked list with rust. we have to always handle the Option nature of the next node.

// two ways of travessing a linked list in rust

// 1. Using while let Some():
let mut current = &list;  // list is your Option<Box<ListNode>>
while let Some(node) = current {
    // Do something with node.val
    println!("Current value: {}", node.val);
    current = &node.next;
}

// 2. Using while with explicit matching:
let mut current = list;
while current.is_some() {
    if let Some(node) = &current {
        // Do something with node.val
        println!("Current value: {}", node.val);
        current = node.next;
    }
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Solution {
  pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // 1. Create a dummy node to hold the result
    let mut dummy = ListNode::new(0);
    let mut current = &mut dummy;

    // setup mutable pointers for traversal
    let mut l1_current = l1;
    let mut l1_current = l2;
    let mut carry = 0;

    while l1_current.is_some() || l2_current.is_some() || carry > 0 {
      // Get values from current nodes if they exist
      let val1 = l1_current.as_ref().map_or(0, |node| node.val);
      let val2 = l2_current.as_ref().map_or(0, |node| node.val);
      // 2. Here you'll do your addition logic
      // - Add the values and carry
      // - Calculate new digit and new carry
      let sum =  val1 + val2 + carry;
      let digit = sum % 10;
      carry = sum / 10;
        // EXPLANATION OF THE ADDITION CODE = [2,4,3] + [5,6,4]:

        // ONE: The modulo operator (%) gives us the remainder after division by 10, which is always going to be a single digit (0-9). This is exactly what we need for each position in our number. if a number is less than 10, that number becomes the remainder as its in this case here

        // TWO: When you divide two integers, the decimal part is truncated - you only get the whole number part. This is different from regular math division!

        // Note: If sum < 10, then sum / 10 = 0
        //       If sum >= 10, then sum / 10 = 1
        // First iteration:
          sum = 2 + 5 + 0 = 7
          digit = 7 % 10 = 7
          carry = 7 / 10 = 0

        // Second iteration:
          sum = 4 + 6 + 0 = 10
          digit = 10 % 10 = 0
          carry = 10 / 10 = 1

        // Third iteration:
          sum = 3 + 4 + 1(1 carried from 2nd interation) = 8
          digit = 8 % 10 = 8
          carry = 8 / 10 = 0

        // 3. Here you'll create your new node
        // - Create node with calculated digit
        // - Add it to your result list
        // let new_node = ListNode {
        //   val: digit,
        //   next: None
        // }

        current.next = Some(Box::new(ListNode::new(digit)));
        current = current.next.as_mut().unwrap();

        // 4. Move to next nodes
        l1_current = l1_current.and_then(|node| node.next);
        l2_current = l2_current.and_then(|node| node.next);


    }
      // 5. Return your result
      dummy.next
  }
}