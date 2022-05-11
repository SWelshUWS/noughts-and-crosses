/*

  Noughts & Crosses - B00379360

  Main rs 

*/

use std::io;
use std::process;


mod board;
mod ai_player;
mod player;
mod game;



fn main() {
  println!("\n*~*~*~* Noughts & Crosses *~*~*~*\n\n");
 
 loop{

    /*
      easy mode: simple rand used
      hard mode: uses minimax algorithm 
      quit: usually used after a few runs of option 2
    */

    println!("Please select one of the following options.\n");
    println!("\t1: Easy mode\n\t2: Hard mode\n\t3: Quit the game\n");

    // mutable var to hold menu choice
    let mut sel = String::new();

    // get menu option from user
    io::stdin().read_line(&mut sel)
      .expect("\nRead line error\n");

    // extract integer from stdin
    let sel : u8 = match sel.trim().parse(){
      Ok(num) => (num),
      Err(_) => continue,
    };
    
    if sel == 1 {
      println!("Playing in easy mode\n");
      let mut game = game::Game::new(sel);
      game.start_game();
      break;
    } else if sel == 2 {
      println!("Playing in hard mode lol\n");
      let mut game = game::Game::new(sel);
      game.start_game();
      break;
    } else if sel == 3 {
      println!("Exiting the game...\n");
      process::exit(0);
    } else {
      println!("Please select a valid option. \n");
      continue;
    }

 }
}
