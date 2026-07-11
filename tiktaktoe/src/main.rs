mod board;
use std::io;

fn main() {
    let mut game = board::Places::new();

    game.add(board::Position::P6, board::User::Ux);
    
    board::showboard(&game);

    game.add(board::Position::P6, board::User::Ux);

}

fn playroundx(user: board::User, game: board::Places) {
	let mut input = String::new();
	
	io::stdin().read_line(&mut input).expect("Couldn't readline");

	match input {
		"1" => {
			if !(game.p1 == " ") {
				println("invalid");
			} else {
				game.add(board::Position::P1, user)
			}
		}
	}
}
