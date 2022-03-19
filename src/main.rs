//use std::io;
mod backend;
mod user_interface;

fn main() {

	let mut current_game = backend::new_game();
	current_game.print_board();
	let (mut game_finished, mut winner) = current_game.check_endgame();

	while !game_finished {
		if current_game.x_turn {
			current_game.push_move(user_interface::get_user_move());
		} else {
			backend::minimax_algo(&mut current_game, 9, -2, 2);
		}
		current_game.print_board();
		(game_finished, winner) = current_game.check_endgame();
	}

	println!("");

	match winner {
	     1 => println!("X won!"),
		-1 => println!("O won!"),
		 0 => println!("It's a draw"),
		std::i8::MIN..=-2 | 2..=std::i8::MAX => println!("Something weird happened"),
	}

}
