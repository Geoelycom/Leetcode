impl Solution {
  pub fn find_solution(customfunction: &CustomFunction, z: i32) -> Vec<Vec<i32>> {
let mut result = Vec::new();
  let mut x = 1; // we start with X = 1 (positive intgers only)
      
      // while f(x, 1) <= there might be valid solutions
  while x <= 1000 && customfunction.f(x, 1) <= z { // we add a reasonable bound
      let mut y = 1; // we start with y = 1 
       
      // while f(x,y) <= z, we check for solutions
      while y <= 1000 && customfunction.f(x, y) <= z {
          if customfunction.f(x, y) == z {
              result.push(vec![x, y]);
          }
          y = y + 1;
      }
      x = x + 1;
  }
  result
}
}