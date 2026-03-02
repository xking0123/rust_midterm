use std::io;

//rust midterm- adding ai to tic tac toe
fn main(){
    let mut board = [' '; 9];
    let mut player = 'X';
    let mut moves = 0;

    clearscreen::clear().expect("failureeeee");
    println!("TIC... TAC... TOE... rust edition");
    println!("Player 1: X and Player 2: O");

    loop {
        //drawing board
        println!(" {} | {} | {} ", board[0], board[1], board[2]);
        println!("-----------");
        println!(" {} | {} | {} ", board[3], board[4], board[5]);
        println!("-----------");
        println!(" {} | {} | {} ", board[6], board[7], board[8]);

        //getting user input and validating
        println!("{}, Pick spot 1-9 to make ya move ya goober: ", player);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let player_choice: usize = input.trim().parse().unwrap_or(0); //.unwrap like .expect?

        if player_choice >= 1 && player_choice <= 9 && board[player_choice - 1] == ' ' {
            board[player_choice - 1] = player;
            moves += 1;
        } else if player_choice > 9 || player_choice < 1 {
            println!("u stupid... try again");
            continue;
        } else if board[player_choice - 1] == 'X' || board[player_choice - 1] == 'O' {
            println!("what u doing... that spot has already been taken... try again");
            continue;
        }

        if check_win(&board, player) {
            println!("game... over....");
            reset_game();
            break;
        }else if moves == 9{
            println!("TIE GAME... BAAAANG");
            reset_game();
            break;
        }

        player = if player == 'X' { 'O' } else { 'X' };
    }
}

fn check_win(b: &[char; 9], p: char) -> bool {
    //all horizontal wins
    (b[0] == p && b[1] == p && b[2] == p) || 
    (b[3] == p && b[4] == p && b[5] == p) || 
    (b[6] == p && b[7] == p && b[8] == p) ||
    //all vertical wins
    (b[0] == p && b[3] == p && b[6] == p) || 
    (b[1] == p && b[4] == p && b[7] == p) || 
    (b[2] == p && b[5] == p && b[8] == p) || 
    //all diagonal wins
    (b[0] == p && b[4] == p && b[8] == p) || 
    (b[2] == p && b[4] == p && b[6] == p)
}
//tried to do a check_draw, couldn't wrap my head around it fully :(

fn reset_game() {
    println!("ggs. wanna run it back? (Y/N): ");
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("failureeeee");
    let trimmed_response = response.trim(); //trimming necessary so input can be read????

    if trimmed_response == "Y" || trimmed_response == "y" {
        println!("good luck we're running it again");
        main();
    } else if trimmed_response == "N" || trimmed_response == "n" {
        println!("okie... byeeeeee")
    } else {
        println!("bye bye");
    }
}