mod board;
use std::io;

fn main() {
    let mut game = board::Places::new();

	let mut round = 1;
   	board::showboard(&game);

    loop {
    	if round % 2 == 0{
    		println!("X's turn");
       		if playround(board::User::Ux, &mut game) == Some(0) {
       			continue;
       		}
    		round += 1;
    	} else {
    		println!("Y's turn");
    		if playround(board::User::Uy, &mut game) == Some(0){
    			continue;
    		}
    		round += 1;
    	}
    	board::showboard(&game);
		if let (true, Some(b)) = is_win(&game) {
			if b == "X" {
				println!("X wins");
			} else {
				println!("Y wins");
			} 
			board::showboard(&game);
			break;
		}
		if is_done(&game) {
			println!("Board done, new game");
			game = board::Places::new();
			board::showboard(&game);
		}
    	
    }
    

}

fn playround(user: board::User, game: &mut board::Places) -> Option<i32>{
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Couldn't readline");
	let neue = input.trim();
	let input = neue.to_string();
	match &*input {
		"1" => {
			if game.p1 != " " {
				println!("invalid");
				Some(0)
			} else {
				game.add(board::Position::P1, user);
				None
			}
		},
		"2" => {
			if game.p2 != " " {
				println!("invalid");
				Some(0)
			} else {
				game.add(board::Position::P2, user);
				None
			}
		},
		"3" => {
			if game.p3 != " " {
				println!("invalid");
				Some(0)
			} else {
				game.add(board::Position::P3, user);
				None
			}
		},
		"4" => {
			if game.p4 != " " {
				println!("invalid");
				Some(0)
			} else {
				game.add(board::Position::P4, user);
				None
			}
		},
		"5" => {
			if game.p5 != " " {
				println!("invalid");
				Some(0)
			} else {
				game.add(board::Position::P5, user);
				None
			}
		},
		"6" => {
			if game.p6 != " " {
				println!("invalid");
				Some(0)
			} else {
				game.add(board::Position::P6, user);
				None
			}
		},
		"7" => {
			if game.p7 != " " {
				println!("invalid");
				Some(0)
			} else {
				game.add(board::Position::P7, user);
				None
			}
		},
		"8" => {
			if game.p8 != " " {
				println!("invalid");
				Some(0)
			} else {
				game.add(board::Position::P8, user);
				None
			}
		},
		"9" => {
			if game.p9 != " " {
				println!("invalid");
				Some(0)
			} else {
				game.add(board::Position::P9, user);
				None
			}
		},
		&_ => {
			println!("Invalid");
			Some(0)
		}

	}
}

fn is_win(game: &board::Places) -> (bool, Option<String>) {
    let board = (
        &game.p1, &game.p2, &game.p3,
        &game.p4, &game.p5, &game.p6,
        &game.p7, &game.p8, &game.p9,
    );

    match board {
        (p1, p2, p3, _, _, _, _, _, _) if p1 == p2 && p2 == p3 && p1 != " " => (true, Some(p1.clone())),
        (_, _, _, p4, p5, p6, _, _, _) if p4 == p5 && p5 == p6 && p4 != " " => (true, Some(p4.clone())),
        (_, _, _, _, _, _, p7, p8, p9) if p7 == p8 && p8 == p9 && p7 != " " => (true, Some(p7.clone())),

        (p1, _, _, p4, _, _, p7, _, _) if p1 == p4 && p4 == p7 && p1 != " " => (true, Some(p1.clone())),
        (_, p2, _, _, p5, _, _, p8, _) if p2 == p5 && p5 == p8 && p2 != " " => (true, Some(p2.clone())),
        (_, _, p3, _, _, p6, _, _, p9) if p3 == p6 && p6 == p9 && p3 != " " => (true, Some(p3.clone())),

        (p1, _, _, _, p5, _, _, _, p9) if p1 == p5 && p5 == p9 && p1 != " " => (true, Some(p1.clone())),
        (_, _, p3, _, p5, _, p7, _, _) if p3 == p5 && p5 == p7 && p3 != " " => (true, Some(p3.clone())),

        _ => (false, None),
    }
}

fn is_done(game: &board::Places) -> bool {
	let board: [&String; 9] = [
		&game.p1, &game.p2, &game.p3,
		&game.p4, &game.p5, &game.p6,
		&game.p7, &game.p8, &game.p9,
	];

let mut grat = -1;
	for uno in board {
		if uno == &" ".to_string() {
			grat = 1;
			break;
		} else {
			grat = 0;
		}
	}
	grat != 1
}
