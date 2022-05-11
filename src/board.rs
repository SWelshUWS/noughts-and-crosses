/*

  Noughts & Crosses

  board module

*/

use std:: collections::HashMap;


pub struct Board { board: [[char; 3];3] }

impl Board {

  // initialize the board with all '.'
  pub fn new() -> Self {
    Board {
      board: [['.';3];3]
    }
  }


  pub fn display_board(&self) {
  
    println!("\n    0   1   2   \n   --- --- --- ");
    let mut num2:u8 = 0;
    for (_i,row) in self.board.iter().enumerate() {
      print!("{}", num2);
      for (_j,col) in row.iter().enumerate() {
        print!(" | {}", col);
      }
  
      num2 = num2 + 1;
      println!(" |\n   --- --- --- ");
    }
  }

  pub fn check_pos_empty(&mut self, x: usize, y: usize) -> bool {

    // should return boolean
    // check a a position is empty before placement 
    
    if self.board[x][y] == '.' {     
       return true
    } else {
      return false
    }

  }
  

  pub fn is_board_full(&self) -> bool {

    // if the empty place symbol is found, return false
    
    for (_i,row) in self.board.iter().enumerate() {
      for (_j,col) in row.iter().enumerate() {
        if *col == '.' {
          return false;
        }
      }
    }
    return true;
  }
    

  pub fn place_piece(&mut self, x:usize, y:usize, piece: &char){
    self.board[x][y] = *piece;
  }

  fn check_diagonal(&self, piece: &char) -> bool {

    let pc = *piece;

    if self.board[0][0] == pc  && self.board[1][1] == pc &&  self.board[2][2] == pc {
      return true
    } else if self.board[2][0] == pc && self.board[1][1] == pc && self.board[0][2] == pc {
      return true
    } else {
      return false
    }

  }


  fn check_horizontal(&self, piece: &char) -> bool {
    
    let pc = *piece;

    if self.board[0][0] == pc && self.board[0][1] == pc && self.board[0][2]  == pc {
      return true
    } else if self.board[1][0] == pc && self.board[1][1] == pc &&  self.board[1][2]  == pc {
      return true
    } else if self.board[2][0] == pc && self.board[2][1] == pc && self.board[2][2] == pc {
      return true
    } else {
      return false
    }

  }

  fn check_vertical(&self, piece: &char) -> bool {

    let pc = *piece;
    
    if self.board[0][0] == pc && self.board[1][0] == pc && self.board[2][0] == pc {
      return true
    } else if self.board[0][1] == pc && self.board[1][1]  == pc && self.board[2][1] == pc {
      return true
    } else if self.board[0][2] == pc && self.board[1][2] == pc && self.board[2][2] == pc {
      return true
    } else {
      return false
    }

  }


  pub fn check_for_win(&self, piece: &char) -> bool {


    let mut win: bool;

    win = self.check_horizontal(piece);
    if win == true {
      return true
    }
    win = self.check_vertical(piece);
    if win == true {
      return true
    }
    win = self.check_diagonal(piece);
    if win == true {
      return true
    }
    // return win status
    win = false;

    
    win
  }


  // A.I. board functions

  fn max(&self, pos: &u8, x: &usize, y: &usize, ai_piece: &char,  board: &[[char; 3];3]) -> i8 {

    let mut max: i8 = 0; 
    let mut score: i8 = 0;

    let mut human: char = 'X';
    if *ai_piece == 'X' {
      human = 'O';
    }

    
    // check horizontal
    for mut _i in 0..3 {
      
      if board[*x][_i] == *ai_piece {
        score += 1;
        _i += 1;       
      } else if board[*x][_i] == human {
        score -= 1;
      }
    }
    if score == 2 {
      max = 10;
      return max;     
    }
    // add to min and reset
    max += score;
    score = 0;

    // check vertical
    for mut _j in 0..3 {
      if board[_j][*y] == *ai_piece  {
        score += 1;
        _j += 1;
        
      } else if board[_j][*y] == human {
        score -= 1;
      }
    } 
    if score == 2 {
      max = 10;
      return max;     
    }
    max += score;
    score = 0;

    // check diagonals

    // if on the left diagonal (\)
    if *pos == 0 || *pos == 4 || *pos == 8 {

      if board[0][0] == *ai_piece  { 
        score += 1;
      } else if board[0][0] == human {
        score -= 1;
      }
      if board[1][1] == *ai_piece  { 
        score += 1;       
      } else if board[1][1] == human {
        score -= 1;
      }
      if board[2][2] == *ai_piece  {
        score += 1;
      } else if board[2][2] == human {
        score -= 1;
      }
    } 
    if score == 2 {
      max = 10;
      return max;     
    }
    max += score;
    score = 0;

    // if on the right diagonal (/)
    if *pos == 2 || *pos == 4 || *pos == 6 {

      if board[0][2] == *ai_piece { 
        score += 1;
      } else if board[0][2] == human {
        score -= 1;
      }
      if board[1][1] == *ai_piece { 
        score += 1;       
      } else if board[1][1] == human {
        score -= 1;
      }
      if board[2][0] == *ai_piece  {
        score += 1;
      } else if board[2][0] == human {
        score -= 1;
      }
      
    }
    if score == 2 {
      max = 10;
      return max;     
    }
    max += score;
    
    return max;

  }


  fn min(&self, pos: &u8, x: &usize, y: &usize, human_piece: &char, board: &[[char; 3];3]) -> i8 {

    let mut min: i8 = 0; 
    let mut score: i8 = 0;


    let mut ai: char = 'X';
    if *human_piece == 'X' {
      ai = 'O';
    }


    // check horizontal
    for mut _i in 0..3 {
      if board[*x][_i] == *human_piece {
        score -= 1;
        _i += 1;
        if score == -2 {
          min = -10;
          return min;     
        }
      } else if board[*x][_i] == ai {
        score += 1;
      }
    }
    // add to min and reset
    min += score;
    score = 0;

    // check vertical
    for mut _j in 0..3 {
      if board[_j][*y] == *human_piece {
        score -= 1;
        _j += 1;
        if score == -2{
          min = -10;
          return min;     
        }
      } else if board[_j][*y] == ai {
        score += 1;
      }
    }
    min += score;
    score = 0;

    // check diagonals

    // left diagonal
    if *pos == 0 || *pos == 4 || *pos == 8 {

      if board[0][0] == *human_piece { 
        score += -1;
      } else if board[0][0] == ai {
        score += 1;
      }
      if board[1][1] == *human_piece { 
        score += -1;
        if score == -2 {
          min = -10;
          return min;     
        }
      } else if board[1][1] == ai {
        score += 1;
      }
      if board[2][2] == *human_piece {
        score += -1;
        if score == -2 {
          min = -10;
          return min;     
        }
      } else if board[2][2] == ai {
        score += 1;
      }
    } 
    min += score;
    score = 0;

    // right diagonal
    if *pos == 2 || *pos == 4 || *pos == 6 {

      if board[0][2] == *human_piece { 
        score += -1;
        if score == -2 {
          min = -10;
          return min;     
        }
      } else if board[0][2] == ai {
        score += 1;
      }
      if board[1][1] == *human_piece { 
        score += -1;
        if score == -2 {
          min = -10;
          return min;     
        }
      } else if board[1][1] == ai {
        score += 1;
      }
      if board[2][0] == *human_piece {
        score -= 1;
        if score == -2 {
          min = -10;
          return min;     
        }
      } else if board[2][0] == ai {
        score += 1;
      }
      
    }
    min += score;
    return min;


  }

  

  pub fn ai_helper(&self, piece: &char) -> (usize,usize) {
  
    let mut ai: char = 'X';
    let mut human: char = 'O';

    let mm_board = self.board.clone();

    // decide pieces to use for each metric
    if *piece == 'X' {
      ai = 'X';
      human = 'O';
    } else if *piece == 'O'{
      ai = 'O';
      human = 'X';
    } 


    // create hash map to store cell tuple
    // <(pos),(x,y,min,max)> 
    let mut positions: HashMap<u8, (usize, usize, i8, i8)> = HashMap::new();

   

    // populate hash map with cell information
    let mut count = 0;
    let (mut _i,mut _j): (usize, usize);
    for _i in 0..3 {
      for _j in 0..3 {
        let x = _i as usize;
        let y = _j as usize;
        if mm_board[_i][_j] == '.' {
          let min = self.min(&count,&x,&y,&human, &mm_board);
          let max = self.max(&count,&x,&y,&ai, &mm_board);
          positions.insert(count, (x,y,min,max));
          count += 1;
        } else {
          let min = 0;
          let max = 0;
          positions.insert(count, (x,y,min,max));
          count += 1;
        }
        
      }

    }

    // display gathered values for next move in hashmap
    println!("hashmap: {:?} \n", positions);

    let mut max: i8 = 0;
    let mut min: i8 = 0;
    let mut best_min: u8 = 0;
    let mut best_max: u8 = 0;
    for mut _k in 0..9 {
      let (ref _v,ref _x, ref y, ref z) = positions[&(_k)]; 
      if *y < min {
        min = *y;
        best_min = _k;
      }
      if *z > max {
        max = *z;
        best_max = _k;
      }
    }

    let (mut _x, mut _y): (usize,usize);
    if max.abs() == 10 {
      let (ref a,ref b, ref _c, ref _d) = positions[&(best_max)];
      _x = *a;
      _y = *b;
    } else if min.abs() == -10 {
      let (ref a,ref b, ref _c, ref _d) = positions[&(best_min)];
      _x = *a;
      _y = *b;
    } else if min.abs() <= max.abs() {
      let (ref a,ref b, ref _c, ref _d) = positions[&(best_max)];
      _x = *a;
      _y = *b;
    } else {
      let (ref a,ref b, ref _c, ref _d) = positions[&(best_min)];
      _x = *a;
      _y = *b;
    } 
    println!("min abs: {} max abs: {} \n", min.abs(), max.abs());
    println!("x: {}, y: {}", _x, _y);



    
    return (_x,_y)
  }
   


}



