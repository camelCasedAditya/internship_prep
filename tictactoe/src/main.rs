use tictactoe:: {check_win, check_tie, print_board, get_move};

const X_MOVE: char = 'X';
const O_MOVE: char = 'O';


fn main() {
    let mut turn: char = 'X';
    let mut board: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
    loop {
        print_board(&board);
        board = get_move(turn, board);

        if check_tie(&board) == true ||  check_win(turn, &board) {
            print_board(&board);
            println!("Thanks for playing!");
            break;
        }



        if turn == X_MOVE {
            turn = O_MOVE;
        }
        else if turn == O_MOVE {
            turn = X_MOVE;
        }
    }
}

