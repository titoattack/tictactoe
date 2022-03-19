pub struct Game {
	pub board: [i8; 9],
	pub x_turn: bool,
	pub move_count: usize,
	pub move_history: [usize; 9],
	pub available_moves: [bool; 9],
}

impl Game {
	pub fn print_board(&self) {
		//Convert board to printable format
		let mut printable_board = [' '; 9];
		for (i, x) in self.board.iter().enumerate() {
			printable_board[i] = match x {
				0 => ' ',
				1 => 'X',
				-1 => 'O',
				2..=std::i8::MAX => ' ',
				std::i8::MIN..=-2 => ' ',
			};
		}

		//Clear terminal
		print!("\x1B[2J\x1B[1;1H");

		for i in 0..3 {
			for j in 0..3 {
				print!("{} ", printable_board[3*i + j]);
			}
			print!("\n");
		}
	}

	pub fn push_move(&mut self, input_move: usize) -> bool {
		match input_move {
			10..=std::usize::MAX => {
				println!("Invalid move");
				return false;
			}

			9 => {
				self.takeback_move()
			}


			_ => {
				if self.board[input_move] != 0 {
					println!("Invalid move");
					return false;
				} else {
					if self.x_turn {
						self.board[input_move] = 1;
					} else {
						self.board[input_move] = -1;
					}
					self.x_turn = !self.x_turn;
					self.move_history[self.move_count] = input_move;
					self.move_count += 1;
					self.available_moves[input_move] = false;
				}

				println!("Good move!");
				return true;
			}
		}
	}

	pub fn takeback_move(&mut self) -> bool {
		if self.move_count == 0 {
			false
			} else {
			self.move_count -= 1;
			let last_move = self.move_history[self.move_count];
			self.board[last_move] = 0;
			self.x_turn = !self.x_turn;
			self.available_moves[last_move] = true;
			true
		}
	}


	pub fn check_endgame(&self) -> (bool, i8) {
		let mut finished_game = false;
		let mut winner = 0;
		if self.move_count > 8 {
			finished_game = true;
		}	

		let mut aux: usize;
		for pattern in WINNING_PATTERNS.iter() {
			aux = pattern[0];
			if self.board[aux] == 0 {
				continue;
			} else {
				if self.board[pattern[1]] == self.board[aux] && self.board[pattern[2]] == self.board[aux] {
					finished_game = true;
					winner = self.board[aux];
					break;
				}
			}

		}

		(finished_game, winner)
	}

	pub fn evaluation(&self) -> i8 {
		let (_, winner) = self.check_endgame();
		winner
	}
}	


pub fn new_game() -> Game {
	let result = Game {
		board: [0; 9],
		x_turn: true,
		move_count: 0,
		move_history: [0; 9],
		available_moves: [true; 9],
	};
	result
}

pub const WINNING_PATTERNS: [[usize; 3]; 8] = [[0, 3, 6], [1, 4, 7], [2, 5, 8], 
					   							[0, 1, 2], [3, 4, 5], [6, 7, 8],
												[0, 4, 8], [2, 4, 6]];

