use std::io::{self, Write};
use rand::Rng;

fn main() {
    let mut board = [[' '; 3]; 3];
    let mut current_player = 'X';

    loop {
        print_board(&board);

        if current_player == 'X' {
            player_turn(&mut board, &mut current_player);
        } else {
            make_computer_move(&mut board, 'O');
        }

        if check_win(&board, current_player) {
            println!("Player {} wins!", current_player);
            print_board(&board); // Print the final board when a player wins
            break;
        } else if is_draw(&board) {
            println!("It's a draw!");
            break;
        }

        current_player = if current_player == 'X' { 'O' } else { 'X' };
    }
}

// Function to print the game board in a formatted way
fn print_board(board: &[[char; 3]; 3]) {
    for row in board {
        for &cell in row {
            print!(" {} |", cell);
        }
        println!();
        println!("---+---+---");
    }
    println!();
}

// Function to check if a player has won the game (considering rows, columns, and diagonals)
fn check_win(board: &[[char; 3]; 3], player: char) -> bool {
    // Check rows and columns for a win
    for i in 0..3 {
        if (board[i][0] == player && board[i][1] == player && board[i][2] == player) ||
           (board[0][i] == player && board[1][i] == player && board[2][i] == player) {
            return true;
        }
    }

    // Check diagonals for a win
    if (board[0][0] == player && board[1][1] == player && board[2][2] == player) ||
       (board[0][2] == player && board[1][1] == player && board[2][0] == player) {
        return true;
    }

    false // Return false if the player has not won
}

// Function to check if the game is a draw (no empty cells on the board)
fn is_draw(board: &[[char; 3]; 3]) -> bool {
    !board.iter().flatten().any(|&cell| cell == ' ')
}

// Function for the human player (X) to make a move (prompts for row and column input)
fn player_turn(board: &mut [[char; 3]; 3], current_player: &mut char) {
    println!("Player {}'s turn", current_player);

    let mut row_str = String::new(); // String to store user input for row
    let mut col_str = String::new(); // String to store user input for column
    let mut row: usize; // Variable to store the row index
    let mut col: usize; // Variable to store the column index

    // Loop to prompt for valid row input (1-3)
    loop {
        print!("Enter the row (1-3): ");
        io::stdout().flush().unwrap(); // Flush the output buffer to ensure prompt is displayed
        row_str.clear(); // Clear the string buffer for row input
        io::stdin().read_line(&mut row_str).unwrap(); // Read user input for row

        match row_str.trim().parse::<usize>() {
            Ok(num) if num >= 1 && num <= 3 => {
                row = num - 1; // Convert user input to row index (0-based)
                break; // Exit the loop if valid input is provided
            }
            _ => {
                println!("Invalid input. Please enter a number between 1 and 3.");
                continue; // Prompt for input again if invalid input is provided
            }
        }
    }

    // Loop to prompt for valid column input (1-3)
    loop {
        print!("Enter the column (1-3): ");
        io::stdout().flush().unwrap(); // Flush the output buffer to ensure prompt is displayed
        col_str.clear(); // Clear the string buffer for column input
        io::stdin().read_line(&mut col_str).unwrap(); // Read user input for column

        match col_str.trim().parse::<usize>() {
            Ok(num) if num >= 1 && num <= 3 => {
                col = num - 1; // Convert user input to column index (0-based)
                break; // Exit the loop if valid input is provided
            }
            _ => {
                println!("Invalid input. Please enter a number between 1 and 3.");
                continue; // Prompt for input again if invalid input is provided
            }
        }
    }

    // Check if the chosen position is empty
    if board[row][col] == ' ' {
        board[row][col] = *current_player; // Place the current player's symbol on the chosen position
    } else {
        println!("This position is already taken. Please choose another one.");
        player_turn(board, current_player); // Retry turn if the chosen position is not empty
    }
}

// Function for the computer player (O) to make a move (randomly chooses a valid empty cell)
fn make_computer_move(board: &mut [[char; 3]; 3], computer_player: char) {
    let mut rng = rand::thread_rng();
    let mut row: usize;
    let mut col: usize;

    loop {
        row = rng.gen_range(0..3); // Generate a random row index
        col = rng.gen_range(0..3); // Generate a random column index

        if board[row][col] == ' ' {
            board[row][col] = computer_player; // Place the computer player's symbol on the chosen position
            break; // Exit the loop after placing the symbol in a valid empty cell
        }
    }
}
