fn solution(board: Vec<Vec<char>>) -> Vec<Vec<char>> {
  let rows = board.len();
  let cols = board[0].len();
  let mut result = board.clone();
  
  // Step 1: Simulate falling boxes column by column
  for col in 0..cols {
      let mut boxes = Vec::new();
      // Collect all boxes in the column
      for row in 0..rows {
          if result[row][col] == '#' {
              boxes.push(row);
          }
      }
      
      // Clear original box positions
      for &row in &boxes {
          result[row][col] = '-';
      }
      
      // Let boxes fall
      let mut current_row = rows as i32 - 1;
      while !boxes.is_empty() && current_row >= 0 {
          match result[current_row as usize][col] {
              '-' => {
                  // Place box here and remove from list
                  result[current_row as usize][col] = '#';
                  boxes.pop();
                  current_row -= 1;
              }
              '*' => {
                  // Box hits obstacle - trigger explosion
                  explode(&mut result, current_row as usize, col);
                  boxes.pop(); // Remove the box that triggered explosion
                  current_row -= 1;
              }
              '#' => {
                  // Hit another box, move up one row
                  current_row -= 1;
              }
              _ => unreachable!(),
          }
      }
  }
  
  result
}

// Handle explosion at given position
fn explode(board: &mut Vec<Vec<char>>, row: usize, col: usize) {
  let rows = board.len() as i32;
  let cols = board[0].len() as i32;
  
  // Check all 8 surrounding cells plus the center
  for dr in -1..=1 {
      for dc in -1..=1 {
          let r = row as i32 + dr;
          let c = col as i32 + dc;
          
          // Check bounds
          if r >= 0 && r < rows && c >= 0 && c < cols {
              let r = r as usize;
              let c = c as usize;
              // Clear boxes in explosion radius, preserve obstacles
              if board[r][c] == '#' {
                  board[r][c] = '-';
              }
          }
      }
  }
}

fn main() {
  // Test case 1
  let board1 = vec![
      vec!['#', '-', '#', '#', '*'],
      vec!['#', '-', '-', '#', '#'],
      vec!['-', '#', '-', '#', '-'],
      vec!['-', '-', '#', '-', '#'],
      vec!['#', '*', '-', '-', '-'],
      vec!['-', '-', '*', '#', '-'],
  ];
  let result1 = solution(board1);
  for row in &result1 {
      println!("{:?}", row);
  }
  
  println!();
  
  // Test case 2
  let board2 = vec![
      vec!['#', '#', '*'],
      vec!['#', '-', '*'],
      vec!['#', '-', '*'],
      vec!['-', '#', '#'],
      vec!['*', '-', '#'],
      vec!['*', '-', '-'],
      vec!['*', '-', '-'],
  ];
  let result2 = solution(board2);
  for row in &result2 {
      println!("{:?}", row);
  }
}