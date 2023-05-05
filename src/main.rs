use std::io::{stdin,stdout,Write};
// This is a constant that has the board be 3 by 3
const SIZE: usize = 3;

fn print_board(board: [[char; SIZE]; SIZE]) {
// A forloop that adds the rows to the board.
    for row in board.iter() {
        println!(" {} | {} | {}", row[0], row[1], row[2]);
        if row != &board[SIZE-1] {
            println!("-----------");
        }
    }
}

fn check_if_win(board: [[char; SIZE]; SIZE], player: char) -> bool {
    // Checks the rows and columns.
    for i in 0..SIZE {
        if board[i][0] == player && board[i][1] == player && board[i][2] == player {
            return true;
        }
        if board[0][i] == player && board[1][i] == player && board[2][i] == player {
            return true;
        }
    }
    // Checks the diagonals.
    if board[0][0] == player && board[1][1] == player && board[2][2] == player {
        return true;
    }
    if board[0][2] == player && board[1][1] == player && board[2][0] == player {
        return true;
    }
    // No one wins.
    false
}

fn main() {
    let mut board = [[' '; SIZE]; SIZE];
    let mut turn = 'X';

// This asks the player which row and column he wants to enter.
    loop {
        print_board(board);
        print!("Player {}, enter row and column (e.g. 1 2): ", turn);
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let tokens: Vec<&str> = input.trim().split(" ").collect();
        println!("{:?}", tokens);
        let row: usize = tokens[0].parse().unwrap();
        let col: usize = tokens[1].parse().unwrap();

// This tells the player that if the spot is already taken that he should pick another spot. 
        if board[row-1][col-1] != ' ' {
            println!("That spot is already taken bro pick somewhere else!");
            continue;
        }
// This checks if the player won the game, if the player won the game than 
        board[row-1][col-1] = turn;
        if check_if_win(board, turn) {
            print_board(board);
            println!("Congrats on the win Player {}, you deserve a pie but unfortunatly since I'm a computer, the only pie I can give you is raspberry!", turn);
            break;
        }
        if board.iter().all(|row| row.iter().all(|&x| x != ' ')) {
            print_board(board);
            println!("It's a draw!");
            break;
        }
        turn = if turn == 'X' { 'O' } else { 'X' };
    }
}
