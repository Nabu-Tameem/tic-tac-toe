use std::io;
use std::io::Write;

fn get_move() -> usize {
    loop {
        let mut cell = String::new();
        let stdin = io::stdin();
        print!("Enter a cell number: ");
        let _=io::stdout().flush().expect("Unable to flush stdout");
        stdin.read_line(&mut cell).expect("Error reading cell input.");
        cell = cell.trim().to_string();

        // if inputted value is an 8bit int
        if cell.parse::<i8>().is_ok() {
            // convert string to int
            let cell_index = cell.parse::<usize>().unwrap();
            if cell_index >= 1 &&  cell_index <= 9 {
                return cell_index;
            }

            println!("Invalid number. Please enter an integer from 1 to 9");
        }
        else {
            println!("Invalid input. Input must be an integer from 1 to 9");
        }
    }

}

fn print_board(board: &Vec<&str>) {
    for i in 0..9 {
        if i % 3 == 0 {
            println!("\n---------");
        }
        print!("|{}|", board[i]);
    }
    // two new lines
    println!("\n");
}

fn check_with_offset(board: &Vec<&'static str>, offset: usize, multiplier: usize) -> &'static str {
    for i in 0..3 {
        let index: usize = i * multiplier;
        if board[index] == board[index + offset] && board[index] == board[index + offset * 2] {
            return board[index];
        }
    }

    return "N";
}

fn check_winner(board: &Vec<&'static str>) -> &'static str {
    // check horizontal
    let horizontal_result = check_with_offset(board, 1 ,3);
    if !horizontal_result.eq("N") {
        return horizontal_result;
    }

    // check vertical
    let vertical_result = check_with_offset(board, 3, 1);
    if !vertical_result.eq("N") {
        return vertical_result;
    }

    // check diagonal
    if (board[0] == board[4] && board[0] == board[8]) ||
       board[2] == board[4] && board[2] == board[6] {
        return board[4];
    }

    // check tie
    for i in 0..9 {
        if !board[i].eq("X") && !board[i].eq("O") {
            // board is not full, return no winner
            return "N";
        }
    }
    // board is full, indicate a tie
    return "T";
}

fn play_loop() -> &'static str{
    let mut turn = "X";
    let mut board = vec!["1","2","3","4","5","6","7","8","9"];
    print_board(&board);
    loop {

        let cell = get_move() - 1;
        if board[cell].eq("X") || board[cell].eq("O") {
            println!("Cell is already being used, please pick a different cell.");
            continue;
        }

        board[cell] = turn;

        print_board(&board);

        let winner = check_winner(&board);
        if !winner.eq("N") {
            if winner.eq("T") {
                println!("It is a tie!");
                return winner;
            }
            println!("{} has won!", winner);
            return winner;
        }

        if turn.eq("X") {
            turn = "O";
        }
        else {
            turn = "X";
        }
    }
}

fn main() {
    let mut x_wins = 0;
    let mut o_wins = 0;

    loop {

        let mut input = String::new();
        print!("Type 'play' to play or 'exit' to leave: ");
        let _= io::stdout().flush().expect("Unable to flush stdout");
        io::stdin().read_line(&mut input).expect("Error reading user input.");
        input = input.trim().to_string();

        if input.eq_ignore_ascii_case("play") {

            let winner = play_loop();
            if winner.eq("X") {
                x_wins += 1;
            }
            else if winner.eq("O") {
                o_wins += 1;
            }
            println!("X: {}, O: {}", x_wins, o_wins)
        }
        else if input.eq_ignore_ascii_case("exit") {
            break;
        }
        else {
            println!("Invalid input, Please type 'play' or 'exit'");
        }

    }
}
