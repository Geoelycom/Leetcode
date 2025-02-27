// Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once. The relative order of the elements should be kept the same. Then return the number of unique elements in nums.

// Consider the number of unique elements of nums to be k, to get accepted, you need to do the following things:

// Change the array nums such that the first k elements of nums contain the unique elements in the order they were present in nums initially. The remaining elements of nums are not important as well as the size of nums.
// Return k.
// Custom Judge:

// The judge will test your solution with the following code:

// int[] nums = [...]; // Input array
// int[] expectedNums = [...]; // The expected answer with correct length

// int k = removeDuplicates(nums); // Calls your implementation

// assert k == expectedNums.length;
// for (int i = 0; i < k; i++) {
//     assert nums[i] == expectedNums[i];
// }
// If all assertions pass, then your solution will be accepted.

 

// Example 1:

// Input: nums = [1,1,2]
// Output: 2, nums = [1,2,_]
// Explanation: Your function should return k = 2, with the first two elements of nums being 1 and 2 respectively.
// It does not matter what you leave beyond the returned k (hence they are underscores).
// Example 2:

// Input: nums = [0,0,1,1,1,2,2,3,3,4]
// Output: 5, nums = [0,1,2,3,4,_,_,_,_,_]
// Explanation: Your function should return k = 5, with the first five elements of nums being 0, 1, 2, 3, and 4 respectively.
// It does not matter what you leave beyond the returned k (hence they are underscores).
 

// Constraints:

// 1 <= nums.length <= 3 * 104
// -100 <= nums[i] <= 100
// nums is sorted in non-decreasing order.



// impl Solution {

  fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    // first of all we fixed the edge cases if the array is empty
    if nums.is_empty() {
      return 0;
    }
    // we then create a variable to keep track of the unique elements in the array
    let mut i = 1;
    // we then iterate through the array to check for duplicates
    for j in 1..nums.len() {
      if nums[j] != nums[i - 1] { // we skip 
        nums[i] = nums[j];
        i += 1;
      }
    } 
    return i as i32;
  }
  

// }


fn main() {
  let mut nums1 = vec![1, 1, 2];
  let k1 = remove_duplicates(&mut nums1);
  println!("k = {}, nums = {:?}", k1, nums1); // k = 2, nums = [1, 2, _]

  let mut nums2 = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
  let k2 = remove_duplicates(&mut nums2);
  println!("k = {}, nums = {:?}", k2, nums2); // k = 5, nums = [0, 1, 2, 3, 4, ...]
}

// An array nums of integers, already sorted in non-decreasing order (meaning numbers either stay the same or get bigger, like [1, 1, 2] or [0, 0, 1, 1, 1, 2, 2, 3, 3, 4]). We want to remove the duplicates in-place, so that each element appears only once, and return the new length of the array.