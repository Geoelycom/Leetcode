// Median of Two Sorted Arrays

// Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
// The overall run time complexity should be O(log (m+n)).

 

// Example 1:

// Input: nums1 = [1,3], nums2 = [2]
// Output: 2.00000
// Explanation: merged array = [1,2,3] and median is 2.
// Example 2:

// Input: nums1 = [1,2], nums2 = [3,4]
// Output: 2.50000
// Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
 

// Constraints:

// nums1.length == m
// nums2.length == n
// 0 <= m <= 1000
// 0 <= n <= 1000
// 1 <= m + n <= 2000
// -106 <= nums1[i], nums2[i] <= 106



  pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    // create a new vector to store merged array. 
      let mut merged = Vec::new();
      // innitialise new variables to enable merging of the two arrays from the index
      let mut i = 0;
      let mut j = 0;

      while i < nums1.len() &&  j < nums2.len() {
        if nums1[i] < nums2[j] {
          merged.push(nums1[i]); 
          i += 1;
        } else {
          merged.push(nums2[j]);
          j += 1;
        }
      }

        while i < nums1.len() {
          merged.push(nums1[i]);
          i += 1;
        }

        while j < nums2.len() {
          merged.push(nums2[j]);
          j += 1;
        }

      let n = merged.len();
      if n % 2 == 0 { // even
       return  ((merged[n/2] + merged[n/2 -1]) as f64) / 2.0;
      } else {
        // odd
        return merged[n/2] as f64;
      }
  }


// HOW TO MERGE TWO SORTED ARRAYS IN RUST(Using a two pointer technique)
// Initialize pointers i and j at start of arr1 and arr2
// Create a result vector
// Compare elements at i and j, push smaller one
// Move that pointer forward
// Continue until both arrays are processed
// calculating the middle index of an array

// The middle index of an array can be calculated using the formula `n/2` where `n` is the length of the array. This formula works for both arrays with odd and even lengths.

// `n/2` calculates the middle index of the array. If the array has an odd length, this will give you the index of the middle element. If the array has an even length, this will give you the index of the second element of the two middle elements.

// `n/2 - 1` calculates the index of the first middle element when the array has an even length. This is because the second middle element is at index `n/2` and the first middle element is at index `n/2 - 1`. Subtracting 1 from `n/2` gives you the index of the first middle element.
