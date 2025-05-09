// Given an integer array nums sorted in non-decreasing order, remove some duplicates in-place such that each unique element appears at most twice. The relative order of the elements should be kept the same.

// Since it is impossible to change the length of the array in some languages, you must instead have the result be placed in the first part of the array nums. More formally, if there are k elements after removing the duplicates, then the first k elements of nums should hold the final result. It does not matter what you leave beyond the first k elements.

// Return k after placing the final result in the first k slots of nums.

// Do not allocate extra space for another array. You must do this by modifying the input array in-place with O(1) extra memory.

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

// Input: nums = [1,1,1,2,2,3]
// Output: 5, nums = [1,1,2,2,3,_]
// Explanation: Your function should return k = 5, with the first five elements of nums being 1, 1, 2, 2 and 3 respectively.
// It does not matter what you leave beyond the returned k (hence they are underscores).
// Example 2:

// Input: nums = [0,0,1,1,1,1,2,3,3]
// Output: 7, nums = [0,0,1,1,2,3,3,_,_]
// Explanation: Your function should return k = 7, with the first seven elements of nums being 0, 0, 1, 1, 2, 3 and 3 respectively.
// It does not matter what you leave beyond the returned k (hence they are underscores).
 

// Constraints:

// 1 <= nums.length <= 3 * 104
// -104 <= nums[i] <= 104
// nums is sorted in non-decreasing order.
 // impl Solution {

  fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    // handle edge cases for empty arrays and when array is <= 2
    if nums.is_empty() {
      return 0;
    }
    if nums.len() <= 2 {
      return nums.len() as i32;
    }

    let mut i = 2;
    for j in 2..nums.len() {
      if nums[j] != nums[i - 2] {
        nums[i] = nums[j];
        i += 1;
      }
    }
    return i as i32;
  }
 // }

 fn main() {
  let mut num0 = vec![1];
  let k0 = remove_duplicates(&mut num0);
  println!("k = {}, nums = {:?}", k0, num0); // k = 1, nums = [1, _]
  
  let mut nums1 = vec![1,1,1,2,2,3];
  let k1 = remove_duplicates(&mut nums1);
  println!("k = {}, nums = {:?}", k1, nums1); // k = 5, nums = [1, 1, 2, 2, 3, 3, _]

  let mut nums2 = vec![0,0,1,1,1,1,2,3,3];
  let k2 = remove_duplicates(&mut nums2);
  println!("k = {}, nums = {:?}", k2, nums2); // k = 7, nums = [0, 0, 1, 1, 2, 3, 3, 3, 3, ...]
}