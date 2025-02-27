// Reverse an array in place
fn reverse_array(nums: &mut Vec<i32>) {
let mut left = 0; // we start at the beginning of the array
let mut right = nums.len() - 1; // we start at the end of the array
while left < right {
  let temp = nums[left]; // store the value at the left index
  nums[left] = nums[right]; // swap the value at the left index with the value at the right index
  nums[right] = temp; // swap the value at the right index with the value stored in temp
  left += 1; // move to the next index from the left
  right -= 1; // move to the next index from the right
}

}

fn main() {
  let mut nums = vec![1, 2, 3, 4, 5];
  reverse_array(&mut nums);
  println!("{:?}", nums);
}