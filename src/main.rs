use std::io;

fn main() {
    //handles replay logic
    println!("TIC... TAC... TOE... rust edition");
    println!("You are X. AI is O.");

    loop {
        play_game();

        println!("Play again? (Y/N): ");
        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap();

        match response.trim().to_lowercase().as_str() {
            "y" => continue,
            "n" => {
                println!("okie... byeeeeee");
                break;
            }
            _ => {
                println!("idk what u inputted... im heading out...");
                break;
            }
        }
    }
}

fn play_game() {
    //runs one round of de game :)
    let mut board = [' '; 9];
    let mut player = 'X';

    loop {
        draw_board(&board);

        let position = if player == 'X' {
            get_player_input(&board)
        } else {
            let ai_choice = ai_move(&board);
            println!("AI chooses {}", ai_choice);
            ai_choice
        };

        board[position - 1] = player;

        if check_win(&board, player) {
            draw_board(&board);
            if player == 'X' {
                println!("YOU WIN?? HOW.");
            } else {
                println!("AI wins. As expected.");
            }
            break;
        }

        if is_draw(&board) {
            draw_board(&board);
            println!("TIE GAME.");
            break;
        }

        player = if player == 'X' { 'O' } else { 'X' };
    }
}

fn draw_board(board: &[char; 9]) {
    println!();
    println!(" {} | {} | {} ", board[0], board[1], board[2]);
    println!("-----------");
    println!(" {} | {} | {} ", board[3], board[4], board[5]);
    println!("-----------");
    println!(" {} | {} | {} ", board[6], board[7], board[8]);
    println!();
}

fn get_player_input(board: &[char; 9]) -> usize {
    loop {
        println!("Pick spot 1-9:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(num) = input.trim().parse::<usize>() {
            if num >= 1 && num <= 9 && board[num - 1] == ' ' {
                return num;
            }
        }

        println!("Invalid move. Try again.");
    }
}

fn check_win(b: &[char; 9], p: char) -> bool {
    //horizontal wins
    (b[0] == p && b[1] == p && b[2] == p) ||
    (b[3] == p && b[4] == p && b[5] == p) ||
    (b[6] == p && b[7] == p && b[8] == p) ||
    //vertical wins
    (b[0] == p && b[3] == p && b[6] == p) ||
    (b[1] == p && b[4] == p && b[7] == p) ||
    (b[2] == p && b[5] == p && b[8] == p) ||
    //diagonal wins
    (b[0] == p && b[4] == p && b[8] == p) ||
    (b[2] == p && b[4] == p && b[6] == p)
}

fn is_draw(board: &[char; 9]) -> bool {
    !board.contains(&' ')
    //refers to array that drew board. also is our "check_draw" functiom if you will...
}

// [char; 9] array of 9 characters actually stored in computers memory (direct ownership of array?) when you call this you outright move array from one spot to another?
// &[char; 9] reference (borrow) to array on 9 characters. when this is called, passing in a pointer to the array instead of setting one up to hard store data in memory-mainly just prooviding a way to look at array.

//to allow ai to pick best possible move
fn ai_move(board: &[char; 9]) -> usize {
    let mut best_score = i32::MIN;
    let mut best_move = 0;
    let mut board_copy = *board;

    for i in 0..9 {
        if board_copy[i] == ' ' {
            board_copy[i] = 'O';
            let score = minimax(&mut board_copy, false, 0);
            board_copy[i] = ' ';

            if score > best_score {
                best_score = score;
                best_move = i;
            }
        }
    }

    best_move + 1
}

fn minimax(board: &mut [char; 9], is_maximizing: bool, depth: i32) -> i32 {
    if check_win(board, 'O') {
        return 10 - depth;
    }
    if check_win(board, 'X') {
        return depth - 10;
    }
    if is_draw(board) {
        return 0;
    }

    if is_maximizing {
        let mut best_score = i32::MIN;

        for i in 0..9 {
            if board[i] == ' ' {
                board[i] = 'O';
                let score = minimax(board, false, depth + 1);
                board[i] = ' ';
                best_score = best_score.max(score);
            }
        }

        best_score
    } else {
        let mut best_score = i32::MAX;

        for i in 0..9 {
            if board[i] == ' ' {
                board[i] = 'X';
                let score = minimax(board, true, depth + 1);
                board[i] = ' ';
                best_score = best_score.min(score);
            }
        }

        best_score
    }
}
