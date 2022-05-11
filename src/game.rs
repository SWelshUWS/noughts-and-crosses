/*
  
    Noughts & Crosses 
    main game instance 

*/

use crate::board;
use crate:: ai_player;
use crate::player;

use rand::Rng;
use std::io;
use std:: process;


pub struct Game {

  player_piece: char,
  ai_piece: char,
  game_status: u8,
  difficulty: u8
}

impl Game {

  pub fn new(level: u8) -> Self {
    Game {
      player_piece: '#',
      ai_piece: '#',
      game_status: 0,
      difficulty: level
    }
    
  }

  pub fn start_game(&mut self){

    // start main game loop
    loop{
      // set game status to off
      self.game_status = 0;

      // randomly select who is player 1 (0: human, -1: ai)
      let mut player: i32 = rand::thread_rng().gen_range(-1..1);
        
        // assign X to player 1, O to player 2
        if player == 0 {

          self.player_piece = 'X';
          self.ai_piece = 'O';
        } else {
          self.player_piece = 'O';
          self.ai_piece = 'X';
        }
        let ai = ai_player::AI::new(self.difficulty, self.ai_piece);
        let human = player::Player::new(self.player_piece);

        // set game status to active
        self.game_status = 1;
        

        // board destoyed when game loop ended
        let mut board = board::Board::new();

        // 
        loop{
          if player == 0 {
            board.display_board();
            human.get_pos(&mut board);  
            let win = board.check_for_win(&self.player_piece);      
            if win == true {
              break;
            }  
          } else if player == -1 {
            print!("A.I. turn...\n");
                ai.take_turn(&mut board); 
                board.display_board();
                let win: bool = board.check_for_win(&self.ai_piece);
                if win == true {
                  break;
                }              
          }


          // check game still active
          if self.game_status == 0 { 
            break; 
          } else {
            if board.is_board_full() == true {
              player = 1;
              break;
            }
            // swap to next player 
            player = !player;
            print!("Player =  {}\n", player);
          }


        }
      match player {
        // display end of game message
        0 => println!("Human wins! No singularity for you today.\n"),
        -1 => println!("A.I. wins! Bring on the singularity!\n"),
        1 => println!("Game draw...\n\n"),
        _ => println!("Something's not right.\n")
      }
      // match statement to display human or ai as the winner
      
      
      loop{
        println!("Game over! Play again? Yy/Nn \n");
        let mut again = String::new();
        io::stdin().read_line(&mut again)
          .expect("\nRead line error\n");
        let yn = again.to_lowercase();
        let again = yn.trim();
        if again == "y" {
          println!("Restarting the game... \n");
          break;
        } else if again == "n"{
          println!("Exiting the game... \n");
          process::exit(0);
        }else {
          println!("Please make a valid selection (Yy/Nn) \n");
          continue;
        }
      }
    }  
  }
}
