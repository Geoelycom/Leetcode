// Given a string s, return the longest palindromic
 
// substring
//  in s.

 

// Example 1:

// Input: s = "babad"
// Output: "bab"
// Explanation: "aba" is also a valid answer.
// Example 2:

// Input: s = "cbbd"
// Output: "bb"
 

// Constraints:

// 1 <= s.length <= 1000
// s consist of only digits and English letters.

// two possible solution to the above problem is to use the brute force method or the dynamic programming method.(0(n^3)) || (0(n^2)).

// The brute force method is to check all possible substrings from the longest to the shortest and check if it is a palindrome.

// while the dynamic programming method is to check if the substring is a palindrome.

// The dynamic programming method is the best approach to solve the problem.

  impl Solution  {
    pub fn longest_palindrome(s: String) -> String {
      // first of all, we check if the string is empty or not to handle the edge case.
      if s.is_empty() {
        return "".to_string();
      }

      let mut start = 0;
      let mut end = 0;
      let s_chars: Vec<char> = s.chars().collect(); // convert the string into a an index collectable vector.

      for i in 0..s.len() {
        let len1 = Self::expand_from_center(&s_chars, i as isize, i); // odd length palindrome
        let len2 = Self::expand_from_center(&s_chars, i as isize, i + 1); // even length palindrome
        let len = len1.max(len2);

        if len > end - start {
          start = i.saturating_sub((len - 1)  / 2);
          end = (i + len / 2).min(s.len() - 1);
        }
      }

      s_chars[start..=end].iter().collect() // extract the substring from start to end and return longest palindrome

    } 

    fn expand_from_center(s: &[char], mut left: isize, mut right: usize) -> usize {
      while left >= 0 && right < s.len() && s[left as usize] == s[right] {
        left -= 1;
        right += 1;
      }
      (right -  1 ) - (left as usize ) 
    }
  }