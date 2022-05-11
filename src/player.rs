/*

  Noughts & Crosses
  functionality for human player

*/

use crate::board::Board;

use std::io;
use std::process;

pub struct Player {

  piece: char

}


impl Player {

  pub fn new(player_piece: char) -> Self {
    Player {
      piece: player_piece
    }
    
  }

 
  pub fn get_pos(&self, board: &mut Board){
    
    loop {
      let mut pos = String::new();
      
      print!("please enter x.y coordinates (Q to quit).\n");
      // get menu option from user
      io::stdin().read_line(&mut pos)
        .expect("\nRead line error\n");
      let pos2 = pos.clone();
      let pt = pos2.trim();
      if pt != "Q" {
      let poslen = pos.chars().count() -1;
      if poslen != 3 {
        continue;
      }
      // split input into two vector elements by '.'
      let v: Vec<&str> = pos.split('.').collect();
      // check two values have been created
      let vlen = v.len();
      if vlen != 2 {
        continue;
      }
      
      // convert gridpoints to usize for use in array
      let x : usize = match v[0].trim().parse(){
        Ok(num) => (num),
        Err(_) => 1,
      };
      

      let y : usize = match v[1].trim().parse(){
        Ok(num) => (num),
        Err(_) => 1,
      };
      // check coordinates are in range
      if  x > 2 || y > 2 {
        print!("Invalid coordinates.\n");
        continue;        
      }
         
      let check = board.check_pos_empty(x,y);
      if check == true {
        board.place_piece(x, y, &self.piece);
        break;
      } else {
        print!("Position not empty. please enter x.y coordinates (Q to quit).\n");
        continue;
      }
      } else {
      print!("Exiting game...\n");
      process::exit(0);
       
      }
    } 
  }
}

 
