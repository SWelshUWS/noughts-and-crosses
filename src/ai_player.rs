/*

  Noughts & Crosses - B00379360

  module to control game A.I. 

*/
use rand::Rng;
use crate::board::Board;
//use board::{ Board };


pub struct AI {

  difficulty: u8,
  piece: char

}


impl AI {

  pub fn new(level: u8, ai_piece: char) -> Self {
    AI {
      difficulty: level,
      piece: ai_piece
    }
    
  }




  fn easy_ai(&self) -> (usize, usize){

    // picks coordinates at random
    let x: usize = rand::thread_rng().gen_range(0..3);
    let y: usize = rand::thread_rng().gen_range(0..3);


  (x,y)

  }


  fn hard_ai(&self, board: &mut Board, human_piece: &char) -> (usize,usize) {

    /*
      looks to see if the player could win on next move and
      selects that position as its next move
      if not, the first available best position is chosen based 
      on the algorithm. 
    
    */

    let (x,y): (usize, usize);
    (x,y) = board.ai_helper(human_piece);

    (x,y)

  }

  // take human piece out and change for own piece 
  pub fn take_turn(&self, board: &mut Board ){

    let mut fail = 0;
    loop{
      let (x,y): (usize, usize);
        if self.difficulty == 1 {

          // loop until valid coordinates are found 
          (x,y) = self.easy_ai();
          
        } else if self.difficulty == 2 {
          (x,y) = self.hard_ai(board, &self.piece);

          // this prevents an infinite loop forming if
          // the ai is the last player in a draw
          fail += 1;
          if fail == 1 {
            self.easy_ai();
          }

        } else {
          continue;
        }
        if board.check_pos_empty(x,y) == true {
          board.place_piece(x,y, &self.piece);
          break;
        } else {
          continue;
        }
      }    
    }

}




