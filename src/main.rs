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
    for i in 0..8 {
        if i % 3 == 0 {
            println!("---------");
        }
        print!("|{}|", board[i]);
        if i % 2 == 0 && i != 0 {
            print!("\n");
        }
    }
}

fn play_loop() {
    let mut turn = "X";
    let mut board = vec!["n","n","n","n","n","n","n","n","n"];
    loop {
        print_board(&board);
        let cell = get_move() - 1;
        if !board[cell].eq("n") {
            println!("Cell is already being used, please pick a different cell.");
            continue;
        }

        board[cell] = turn;

        // TODO check if game has been won

        if turn.eq("X") {
            turn = "O";
        }
        else {
            turn = "X";
        }
    }
}

fn main() {
    loop {
        let mut input = String::new();
        print!("Type 'play' to play or 'exit' to leave: ");
        let _= io::stdout().flush().expect("Unable to flush stdout");
        io::stdin().read_line(&mut input).expect("Error reading user input.");
        input = input.trim().to_lowercase();

        if input.eq_ignore_ascii_case("play") {
            play_loop();
        }
        else if input.eq_ignore_ascii_case("exit") {
            break;
        }
        else {
            println!("Invalid input, Please type 'play' or 'exit'");
        }

    }
}
