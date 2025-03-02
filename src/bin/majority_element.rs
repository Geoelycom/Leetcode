// Given an array nums of size n, return the majority element.

// The majority element is the element that appears more than ⌊n / 2⌋ times. You may assume that the majority element always exists in the array.

 

// Example 1:

// Input: nums = [3,2,3]
// Output: 3
// Example 2:

// Input: nums = [2,2,1,1,1,2,2]
// Output: 2
 

// Constraints:

// n == nums.length
// 1 <= n <= 5 * 104
// -109 <= nums[i] <= 109
 

// Follow-up: Could you solve the problem in linear time and in O(1) space?

// We’ve got options—let’s narrow it down:

// Counting: Reliable but needs space.
// Sorting: Works but might not be the fastest (O(n log n)).
// Canceling: Clever, possibly O(n) time, O(1) space (this is the famous Boyer-Moore Voting Algorithm, but let’s discover it naturally!).


// Canceling Idea (Boyer-Moore Hint)
// Concept: Imagine every non-majority element cancels one majority element. Since the majority is > n/2, it’s always left standing.
// Plan:
// Pick a “candidate” for majority.
// Count it up when you see it, count it down when you see something else.
// If the count hits 0, pick a new candidate.
// Why it works: The majority element appears so often it can’t be fully canceled.
// Try [3, 2, 3]:
// Start: Candidate = 3, Count = 1
// 2: Different, Count = 0 (cancel)
// 3: Count = 0, new Candidate = 3, Count = 1
// End: 3 remains.

// impl Solution {
  fn majority_element(nums: Vec<i32>) -> i32 {
    // handle egde cases of an empty array
   if nums.is_empty() {
    return 0;
   }
   let mut candidate = nums[0];
   let mut count = 1;
   for i in 1..nums.len() {
    if nums[i] == candidate {
      count += 1;
    } else if count == 0 { 
      candidate = nums[i];
     count = 1;
    } else {
      count -= 1;
    }
    }
    return candidate;
   }


   fn main () {
      let nums = vec![2, 2, 1, 1, 1, 2, 2];
      let result = majority_element(nums);
      println!("The majority element is: {}", result);
   }
// }