use std::{io, i8};

pub fn print_board(board: &[char]) -> () {
    println!("-------------");
    for i in 0..3 {
        let i = i*3;
        println!("| {} | {} | {} |", &board[i], &board[i+1], &board[i+2]);
        println!("-------------")
    }
}

pub fn get_move(player: char, mut board:[char; 9]) -> [char; 9] {
    loop {
        let index: i8;
        let mut input = String::new();
        println!("Player {} make your move", player);
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        

        if let Ok(number) = input.trim().parse::<usize>() {
            
            if (number) < 1 || (number) > 9 {
                println!("Please box number must be between 1 an 9");
                continue;
            }

            index = (number - 1) as i8;
            if board[(index) as usize] == 'X' || board[(index) as usize] == 'O' {
                println!("This square is already taken");
                continue;
            }
            if board[(index) as usize] != 'X' ||  board[(index) as usize] != 'O' {
                board[(index) as usize] = player;
                return board
            }
        }
    }
}

pub fn check_win(player: char, board: &[char; 9]) -> bool {
    // Check rows
    for mut i in 0..3 {

        if board[i] == board[i + 3] && board[i] == board[i + 6] {
            println!("Player {} won the game", player);
            return true;
        }

        i = i*3;

        if board[i] == player && board[i] == board[i+1] && board[i] == board[i+2] {
            println!("Player {} won the game", player);
            return true
        }

        if (board[0] == board[4] && board[0] == board[8]) || (board[2] == board[4] && board[2] == board[6]) {
        println!("Player {} won the game", player);
        return true
        }
    }

    return false
}

pub fn check_tie(board: &[char; 9]) -> bool {
    let mut counter = 0;

    for i in 0..board.len() {
        if board[i] == 'X' || board[i] == 'O' {
            counter+=1;
        }
    }

    if counter == 9 && check_win('X', &board) == false && check_win('O', &board) == false{
        println!("The game is a tie");
        return true
    }
    

    return false
}