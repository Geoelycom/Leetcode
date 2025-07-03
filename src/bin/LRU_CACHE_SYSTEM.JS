// -----

// **1. LRU Cache**

// Given an array of `n` request IDs as strings and an integer `k`, determine the `k` most recent unique requests after all requests have been received. The results should be ordered from the most recent to the least recent.

// **Example:**

// Suppose `n = 5`, `requests = ["item1", "item2", "item3", "item1", "item3"]`, and `k = 3`.

// Starting from the right and moving left, collecting distinct requests, there is “item3” followed by “item1”. Skip the second instance of “item3” and find “item2”. The answer is `["item3", "item1", "item2"]`.

// **Function Description**

// Complete the function `getLatestKRequests` that takes the following arguments:

// - `str requests[n]`: the request IDs
// - `int k`: the number of requests to report

// **Returns**

// - `str[k]`: the `k` most recent requests

// **Constraints**

// - `1 ≤ k ≤ n ≤ 10^5`
// - `requests[i]` consists of lowercase characters and digits only, [a-z0-9].

// **Input Format For Custom Testing**

// **Sample Case 0**

// **Sample Input For Custom Testing**

// ```
// STDIN        FUNCTION
// -----        --------
// 5            requests[] size n = 5
// item3        requests = ["item3",
// item2        "item2", "item1",
// item1        "item2", "item3"]
// item2
// item3
// 3            k = 3
// ```

// **Sample Output**

// ```
// item3
// item2
// item1
// ```


// SOLUTION
function getLatestKRequests(requests, k) {
  // Use an array to simulate the LRU cache and maintain order
  let cache = [];
  
  // Process each request
  for (let request of requests) {
      // If the request is already in the cache, remove it
      let index = cache.indexOf(request);
      if (index !== -1) {
          cache.splice(index, 1); // Remove the request from its current position
      }
      // Add the request to the end (most recent position)
      cache.push(request);
  }
  
  // Take the last k elements (most recent) and reverse them
  return cache.slice(-k).reverse();
}

// ALGORITHM EXPLANATION AND TIME COMPLEXITY
function getLatestKRequests(requests, k) {
    // Use an array to simulate the LRU cache and maintain order
    let cache = [];
    
    // Process each request
    for (let request of requests) {
        // If the request is already in the cache, remove it
        let index = cache.indexOf(request);
        if (index !== -1) {
            cache.splice(index, 1); // Remove the request from its current position
        }
        // Add the request to the end (most recent position)
        cache.push(request);
    }
    
    // Take the last k elements (most recent) and reverse them
    return cache.slice(-k).reverse();
}

// Time and Space Complexity
// •   Time Complexity:
// 	•  Iterating through requests takes O(n), where n is the length of requests.
// 	•  For each request, indexOf and splice take O(n) in the worst case.
// 	•  Total: O(n^2).
// •  Space Complexity: O(n) to store the cache array.
// While this solution works, the O(n^2) time complexity might be a concern for large inputs (up to 10^5). We can optimize it further using a Map to store the requests and their positions, combined with a Set or array to maintain order, reducing the time complexity to O(n). 