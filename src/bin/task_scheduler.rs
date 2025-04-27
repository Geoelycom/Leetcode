// Problem Breakdown
// •  Input: A list of tasks (represented as uppercase letters A-Z) and a cooling period n.
// •  Constraints:
// 	•  Each task takes 1 time unit to execute.
// 	•  If the same task (e.g., ‘A’) is executed, we must wait n time units before executing another ‘A’.
// 	•  We can insert idle periods if needed to satisfy the cooling constraint.
// •  Output: The minimum number of time units required to execute all tasks.
// •  Examples:
// 	•  Example 1: tasks = ['A', 'A', 'A', 'B', 'B', 'B'], n = 2
// 		•  Output: 8
// 		•  Sequence: [A, B, idle, A, B, idle, A, B]
// 	•  Example 2: tasks = ['A', 'C', 'A', 'B', 'D', 'B'], n = 1
// 		•  Output: 6
// 		•  Sequence: [A, C, A, B, D, B]
// Approach to Solve
// 1.  Understand the Problem:
// 	•  The cooling period means that after executing a task (e.g., ‘A’), we must wait n time units before executing another ‘A’.
// 	•  To minimize the total time, we should schedule tasks as efficiently as possible, filling gaps with other tasks if available, or idle periods if not.
// 	•  The total time depends on the frequency of the most frequent task, as it will dictate the minimum number of idle periods needed.
// 2.  Key Insight:
// 	•  The task with the highest frequency (e.g., ‘A’ appearing 3 times) will require the most idle periods to satisfy the cooling constraint.
// 	•  We can calculate the minimum time by considering the most frequent task and arranging other tasks around it.
// 	•  Formula for the minimum time (without considering other tasks filling idle slots):
// 		•  Let max_freq be the frequency of the most frequent task.
// 		•  Let max_freq_count be the number of tasks with the maximum frequency.
// 		•  The minimum time due to the most frequent task is: (max_freq - 1) * (n + 1) + max_freq_count.
// 		•  This represents scheduling the most frequent tasks with cooling periods between them, plus the final execution of each task with the maximum frequency.
// 	•  However, if other tasks can fill the idle periods, the total time might simply be the number of tasks (if the cooling period is small enough).
// 3.  Algorithm:
// 	•  Count the frequency of each task (A-Z).
// 	•  Find the maximum frequency (max_freq) and the number of tasks with that frequency (max_freq_count).
// 	•  Compute the minimum time using the formula: (max_freq - 1) * (n + 1) + max_freq_count.
// 	•  Compare this with the total number of tasks (tasks.len()), as other tasks might fill idle periods, reducing the need for idle time.
// 	•  Return the maximum of these two values, as the total time cannot be less than the number of tasks.
// 4.  Edge Cases:
// 	•  Empty task list: Return 0.
// 	•  Cooling period n = 0: No cooling needed, so the total time is just the number of tasks.
// 	•  Large cooling period: The formula will account for the necessary idle periods.


// Solution in Rust
// This Rust code implements the task scheduling algorithm described above.
// It calculates the minimum time required to execute all tasks while respecting the cooling period.
fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
  // Handle edge case: empty task list
  if tasks.is_empty() {
      return 0;
  }

  // Step 1: Count the frequency of each task (A-Z)
  let mut freq = vec![0; 26]; // Since tasks are A-Z
  for &task in &tasks {
      let idx = (task as u8 - b'A') as usize; // Convert char to index (0-25)
      freq[idx] += 1;
  }

  // Step 2: Find the maximum frequency and the count of tasks with that frequency
  let max_freq = *freq.iter().max().unwrap(); // Maximum frequency
  let max_freq_count = freq.iter().filter(|&&f| f == max_freq).count() as i32; // Number of tasks with max frequency

  // Step 3: Calculate the minimum time based on the most frequent task
  // Formula: (max_freq - 1) * (n + 1) + max_freq_count
  // This represents the time needed if we schedule the most frequent tasks with cooling periods
  let time_with_cooling = (max_freq - 1) * (n + 1) + max_freq_count;

  // Step 4: Compare with the total number of tasks
  // If there are enough other tasks to fill idle periods, the total time might just be tasks.len()
  let total_tasks = tasks.len() as i32;
  time_with_cooling.max(total_tasks)
}

fn main() {
  // Test cases
  let tasks1 = vec!['A', 'A', 'A', 'B', 'B', 'B'];
  let n1 = 2;
  println!("Example 1: {}", least_interval(tasks1, n1)); // Should print 8

  let tasks2 = vec!['A', 'C', 'A', 'B', 'D', 'B'];
  let n2 = 1;
  println!("Example 2: {}", least_interval(tasks2, n2)); // Should print 6.
}