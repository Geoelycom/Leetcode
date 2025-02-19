// Given a string s, find the length of the longest 
// substring
//  without repeating characters.

 

// Example 1:

// Input: s = "abcabcbb"
// Output: 3
// Explanation: The answer is "abc", with the length of 3.
// Example 2:

// Input: s = "bbbbb"
// Output: 1
// Explanation: The answer is "b", with the length of 1.
// Example 3:

// Input: s = "pwwkew"
// Output: 3
// Explanation: The answer is "wke", with the length of 3.
// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
 

// Constraints:

// 0 <= s.length <= 5 * 104
// s consists of English letters, digits, symbols and spaces.

impl solution {
  pub fb length_of_longest_substring(s: String) -> i32 {
    // Convert string to chars vector for easier indexing
    let chars: Vec<char> = s.chars().collect();
        let mut max_length = 0;
        
        // Go through each starting index
        for i in 0..chars.len() {
            // Keep track of characters we've seen using HashSet
            let mut seen = std::collections::HashSet::new();
            let mut current_length = 0;
            
            // Check characters starting from index i
            for j in i..chars.len() {
                let current_char = chars[j];
                
                // If we haven't seen this character before
                if !seen.contains(&current_char) {
                    seen.insert(current_char);
                    current_length += 1;
                } else {
                    // Found a repeat, stop checking from this starting position
                    break;
                }
            }
            
            // Update max_length if current_length is larger
            max_length = max_length.max(current_length);
        }
        
        max_length as i32
    // sub string is any continues sequence of English letters, digits, symbols and spaces.

  }
}

// Explanation of the Repeated substring problem.
// we start with a string s = "abcabcbb". We can keep track of the longest substring found so far and the starting index of the current substring by starting at the first index 0.
// in "abcabcbb":
// Index 0 ('a'):

// Look at: a (no repeat)
// Look at: ab (no repeat)
// Look at: abc (no repeat)
// Look at: abca (found repeat 'a'!)
// STOP and move to next index

// Index 1 ('b'):

// Look at: b (no repeat)
// Look at: bc (no repeat)
// Look at: bca (no repeat)
// Look at: bcab (found repeat 'b'!)
// STOP and move to next index.
// we would repeat this process till we get to the end of the string.

// the reason wee change the string into a vec and interator is becaause rust cannot index a string directly. hence we convert the string into a vec of chars and then iterate through the vec of chars.
// let chars: Vec<char> = s.chars().collect();

// A hashset is used to put things in it, it remembers whats inside and very easy to retrieve what is put inside.