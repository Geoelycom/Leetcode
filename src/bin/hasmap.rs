// Design HashSet

// Solution
// Design a HashSet without using any built-in hash table libraries.

// Implement MyHashSet class:

// void add(key) Inserts the value key into the HashSet.
// bool contains(key) Returns whether the value key exists in the HashSet or not.
// void remove(key) Removes the value key in the HashSet. If key does not exist in the HashSet, do nothing.
 

// Example 1:

// Input
// ["MyHashSet", "add", "add", "contains", "contains", "add", "contains", "remove", "contains"]
// [[], [1], [2], [1], [3], [2], [2], [2], [2]]
// Output
// [null, null, null, true, false, null, true, null, false]

// Explanation
// MyHashSet myHashSet = new MyHashSet();
// myHashSet.add(1);      // set = [1]
// myHashSet.add(2);      // set = [1, 2]
// myHashSet.contains(1); // return True
// myHashSet.contains(3); // return False, (not found)
// myHashSet.add(2);      // set = [1, 2]
// myHashSet.contains(2); // return True
// myHashSet.remove(2);   // set = [1]
// myHashSet.contains(2); // return False, (already removed)
 

// Constraints:

// 0 <= key <= 106
// At most 104 calls will be made to add, remove, and contains.

struct MyHashSet {
  buckets: Vec<Vec<i32>>,
  size: i32,
  }
  
  
  /** 
   * `&self` means the method takes an immutable reference.
   * If you need a mutable reference, change it to `&mut self` instead.
   */
  impl MyHashSet {
  
      fn new() -> Self {
          let size = 769;
          let mut buckets = Vec::with_capacity(size as usize);
          for _ in 0..size {
              buckets.push(Vec::new());
          }
          MyHashSet { buckets, size }
      }
      
      fn add(&mut self, key: i32) {
         let index = (key % self.size) as usize;
          let bucket = &mut self.buckets[index];
          if !bucket.contains(&key) {
              bucket.push(key);
          }
      }
      
      fn remove(&mut self, key: i32) {
          let index = (key % self.size) as usize;
          let bucket = &mut self.buckets[index];
          if let Some(pos) = bucket.iter().position(|&x| x == key) {
              bucket.remove(pos);
          }
      }
      
      fn contains(&self, key: i32) -> bool {
         let index = (key % self.size) as usize;
          self.buckets[index].contains(&key)
      }
  }
  
  /**
   * Your MyHashSet object will be instantiated and called as such:
   * let obj = MyHashSet::new();
   * obj.add(key);
   * obj.remove(key);
   * let ret_3: bool = obj.contains(key);
   */