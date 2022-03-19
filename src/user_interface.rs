use std::io;

pub fn get_user_move() -> usize {
	let result: usize;
	loop {
		let mut input_text = String::new();
		io::stdin()
			.read_line(&mut input_text)
			.expect("Failed to read from stdin");

		let trimmed = input_text.trim();
		//Using 9 to express the user want to take back the last move		
		//backend::push_move() will handle this situation
		if trimmed == String::from("-mvback") {
			result = 9;
			return result;
		}


		match trimmed.parse::<usize>() {
			Ok(i) => {
				result = i;
				break;
			}
			Err(_) => continue,
		};
	}
	result
}
