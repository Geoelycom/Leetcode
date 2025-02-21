// You are running a gravity simulation involving falling boxes and exploding obstacles. The scenario is represented by a rectangular matrix of characters board.
// Each cell of the matrix board contains one of three characters:

// * '-', which means that the cell is empty;
// * '*', which means that the cell contains an obstacle;
// * '#', which means that the cell contains a box.

// The boxes all fall down simultaneously as far as possible, until they 
// 1. hit an obstacle, 
// 2. another grounded box, 
// 3. or the bottom of the board. 

// If a box hits an obstacle, the box explodes, destroying itself and any other boxes within eight cells surrounding the obstacle. All the explosions happen simultaneously as well.

// Given board, your task is to return the state of the board after all boxes have fallen.
// *Note: You are not expected to provide the most optimal solution, but a solution with time complexity not worse than *O(board[0].length · board.length2) will fit within the execution time limit.
// Example
// * For


// Copy
// board = [['#', '-', '#', '#', '*'],
//          ['#', '-', '-', '#', '#'],
//          ['-', '#', '-', '#', '-'],
//          ['-', '-', '#', '-', '#'],
//          ['#', '*', '-', '-', '-'],
//          ['-', '-', '*', '#', '-']]
// the output should be


// Copy
// solution(board) = [['-', '-', '-', '-', '*'],
//                    ['-', '-', '-', '-', '-'],
//                    ['-', '-', '-', '-', '-'],
//                    ['-', '-', '-', '-', '-'],
//                    ['-', '*', '-', '-', '#'],
//                    ['#', '-', '*', '-', '#']]
// Explanation:
// Expand to see the example video.

// Note: If you are not able to see the video, use this link to access it.
// * For


// Copy
// board = [['#', '#', '*'],
//          ['#', '-', '*'],
//          ['#', '-', '*'],
//          ['-', '#', '#'],
//          ['*', '-', '#'],
//          ['*', '-', '-'],
//          ['*', '-', '-']]
// the output should be


// Copy
// solution(board) = [['-', '-', '*'],
//                    ['-', '-', '*'],
//                    ['-', '-', '*'],
//                    ['-', '-', '-'],
//                    ['*', '-', '-'],
//                    ['*', '-', '#'],
//                    ['*', '-', '#']]
// Explanation:
// Expand to see the example video.

// Note: If you are not able to see the video, use this link to access it.
// Input/Output
// * [execution time limit] 2 seconds (rs)
// * [memory limit] 1 GB
// * [input] array.array.char board

// A matrix that shows where the boxes and obstacles are located. The matrix consists only of characters '-', '*', and '#'.
// Guaranteed constraints: 3 ≤ board.length ≤ 100, 3 ≤ board[i].length ≤ 100.
// * [output] array.array.char
// Return a matrix representing the state of the board after all of the boxes have fallen.

// The boxes are falling down under the influence of gravity. Each box falls down until it hits an obstacle, another grounded box, or the bottom of the board. A box that hits an obstacle or another box explodes, destroying itself and any other boxes within eight cells surrounding the obstacle or the other box. The explosion happens simultaneously with the box falling down.




// FIRRST TEST CASE AFTER BOXES FALL

Original:                  After Fall:
['#','-','#','#','*']     ['-','-','-','-','*']  
['#','-','-','#','#']     ['-','-','-','-','-']
['-','#','-','#','-']     ['-','-','-','-','-']
['-','-','#','-','#']     ['#','-','#','-','-']
['#','*','-','-','-']     ['#','*','#','#','#'] 
['-','-','*','#','-']     ['#','-','*','#','#']


// In column 1: A box falls next to the '*' at row 4
// In column 2: A box falls and hits the '*' at row 5
// In column 3: A box falls next to the '*' at row 5
// In column 0 and column 4, there are no '*' to stop the boxes from falling to the bottom of the board so they fall gracefully to the bottom of the board hence of no effect to us.



// MAPPING OUT EXPLOSIONS THAT HAPPENS IN COLUMN 1,2,3
// After the falls, looking at where boxes hit obstacles:

['-','-','-','-','*']     
['-','-','-','-','-']
['-','-','-','-','-']
['#','-','#','-','-']     
['#','*','#','#','#']  // Key row for explosion in column 1   
['#','-','*','#','#']  // Key row for explosions in columns 2 & 3


// Column 1 (middle explosion):


// Has '*' at row 4
// Has boxes in surrounding 8 cells that will be destroyed:

// Left: the '#' in row 4, column 0
// Right: the '#' in row 4, column 2
// Below: the '#' in row 5, column 0
// Above: the '#' in row 3, column 0
// Diagonally: any '#' in those positions


// Columns 2 & 3 (bottom explosions):

// Has '*' at row 5, column 2
// Will destroy boxes in:

// Left: any '#' in row 5, column 1
// Right: the '#' in row 5, column 3
// Above: the '#' in row 4, columns 1, 2, and 3
// Diagonally: any '#' in those positions


// FINAL STATE OF EXPLOSIONS IN COLUMN 1,2,3

['-','-','-','-','*']     
['-','-','-','-','-']
['-','-','-','-','-']
['#','-','#','-','-']     
['#','*','#','#','#']  // Row 4: explosions from middle '*'
['#','-','*','#','#']  // Row 5: explosions from '*' here


// AFTER EXPLOSION HAPPENS SIMUTANEOUSLY 

['-','-','-','-','*']  // Obstacle remains   
['-','-','-','-','-']
['-','-','-','-','-']
['-','-','-','-','-']  // Box destroyed by explosion from row 4
['-','*','-','-','#']  // Obstacle remains, right'#'survives (too far from expl)
['#','-','*','-','#']  // Left and right '#' survive (too far from explosion)


// EXLANATION OF REMAININNG PIECES AND EXPLOSION
// The '*' in row 0, column 4: It's an obstacle, never moves or explodes
// The '*' in row 4, column 1: It's an obstacle, never moves or explodes
// The '*' in row 5, column 2: It's an obstacle, never moves or explodes
// The '#' in row 4, column 4: Survives because it's not within 8 cells of any exploding box
// The '#' in row 5, column 0: Survives because it's not within 8 cells of any exploding box
// The '#' in row 5, column 4: Survives because it's not within 8 cells of any exploding box



// When these boxes hit obstacles, they explode and destroy:

// The box that hit the obstacle
// Any boxes in the 8 surrounding cells (up, down, left, right, and diagonals)

// The explosions happen simultaneously, so any box in range of multiple explosions only needs to be destroyed once.




// SOLUTION

fn solution(board: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = board.len();
    let cols = board[0].len();
    let mut new_board = board.clone();

    // Step 1: Simulate Gravity first
    for c in 0..cols {
        let mut write_pos = rows; // Start from bottom
        for r in (0..rows).rev() { // Go from bottom up
            match new_board[r][c] {
                '#' => {
                    write_pos = write_pos.saturating_sub(1); // Ensure we don't underflow
                    if r != write_pos { // Only move if needed
                        new_board[r][c] = '-';     // Clear original position
                        new_board[write_pos][c] = '#'; // Place box in new position
                    }
                }
                '*' => {
                    write_pos = r; // Reset write position to above obstacle
                }
                _ => {}
            }
        }
    }

    // Step 2: Check for boxes that hit obstacles and mark for explosion
    let mut to_destroy = vec![vec![false; cols]; rows];
    for r in 0..rows {
        for c in 0..cols {
            if new_board[r][c] == '*' {
                // Check adjacent positions for boxes
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        let nr = r as isize + dr;
                        let nc = c as isize + dc;
                        if nr >= 0 && nr < rows as isize && 
                           nc >= 0 && nc < cols as isize {
                            if new_board[nr as usize][nc as usize] == '#' {
                                to_destroy[nr as usize][nc as usize] = true;
                            }
                        }
                    }
                }
            }
        }
    }

    // Step 3: Apply explosions simultaneously
    for r in 0..rows {
        for c in 0..cols {
            if to_destroy[r][c] {
                new_board[r][c] = '-';
            }
        }
    }

    new_board
}