use std::io::{self, Write}; // Import standard library in Rust. io input and output.

fn main() {
    let mut board = [[' '; 3]; 3]; // Initialize board with empty spaces
    let mut current_player = 'X'; // Initialize the current player as 'X'

    loop {
        print_board(&board); // Print the current state of the game board
        player_turn(&mut board, &mut current_player); // Allow the current player to make a move

        // Check for win or draw
        if check_win(&board, current_player) { // Check if the current player has won
            println!("Player {} wins!", current_player); // Print a message indicating the winner
            break; // Exit the game loop
        } else if is_draw(&board) { // Check if the game is a draw
            println!("It's a draw!"); // Print a message indicating a draw
            break; // Exit the game loop
        }

        // Switch player for the next turn
        current_player = if current_player == 'X' { 'O' } else { 'X' }; // Alternate between 'X' and 'O'
    }
}

// Function to print the game board
fn print_board(board: &[[char; 3]; 3]) {
    for row in board { // Iterate through each row of the board
        for &cell in row { // Iterate through each cell in the row
            print!(" {} |", cell); // Print the value of the cell followed by a separator
        }
        println!(); // Move to the next line after printing each row
        println!("---+---+---"); // Print a separator between rows
    }
    println!(); // Print an empty line after printing the board
}

// Function to check if a player has won the game
fn check_win(board: &[[char; 3]; 3], player: char) -> bool {
    // Check rows and columns for a win
    for i in 0..3 {
        if (board[i][0] == player && board[i][1] == player && board[i][2] == player) || // Check rows
           (board[0][i] == player && board[1][i] == player && board[2][i] == player) { // Check columns
            return true; // Return true if the player has won
        }
    }
    // Check diagonals for a win
    if (board[0][0] == player && board[1][1] == player && board[2][2] == player) || // Check main diagonal
       (board[0][2] == player && board[1][1] == player && board[2][0] == player) { // Check secondary diagonal
        return true; // Return true if the player has won
    }
    false // Return false if the player has not won
}

// Function to check if the game is a draw
fn is_draw(board: &[[char; 3]; 3]) -> bool {
    !board.iter().flatten().any(|&cell| cell == ' ') // Return true if there are no empty cells on the board
}

// Function to allow a player to make a move
fn player_turn(board: &mut [[char; 3]; 3], current_player: &mut char) {
    println!("Player {}'s turn", current_player); // Print a message indicating whose turn it is

    let mut row_str = String::new(); // Create a new empty string to store user input for row
    let mut col_str = String::new(); // Create a new empty string to store user input for column
    let row: usize; // Initialize variable to store the row index
    let col: usize; // Initialize variable to store the column index

    // Loop to prompt the user for a valid row input
    loop {
        println!("Enter the row (1-3): ");
        io::stdout().flush().unwrap(); // Flush the output buffer to ensure prompt is displayed
        row_str.clear(); // Clear the string buffer for row input
        io::stdin().read_line(&mut row_str).unwrap(); // Read user input for row

        // Parse the row input and validate it
        match row_str.trim().parse::<usize>() {
            Ok(num) if num >= 1 && num <= 3 => {
                row = num - 1; // Convert user input to zero-based indexing
                break; // Exit the loop if input is valid
            }
            _ => println!("Invalid input. Please enter a number between 1 and 3."), // Prompt for valid input
        }
    }

    // Loop to prompt the user for a valid column input
    loop {
        println!("Enter the column (1-3): ");
        io::stdout().flush().unwrap(); // Flush the output buffer to ensure prompt is displayed
        col_str.clear(); // Clear the string buffer for column input
        io::stdin().read_line(&mut col_str).unwrap(); // Read user input for column

        // Parse the column input and validate it
        match col_str.trim().parse::<usize>() {
            Ok(num) if num >= 1 && num <= 3 => {
                col = num - 1; // Convert user input to zero-based indexing
                break; // Exit the loop if input is valid
            }
            _ => println!("Invalid input. Please enter a number between 1 and 3."), // Prompt for valid input
        }
    }

    // Check if the chosen cell is empty, and if so, mark it with the current player's symbol
    if board[row][col] == ' ' {
        board[row][col] = *current_player; // Mark the chosen cell with the current player's symbol
    } else {
        println!("This position is already taken. Please choose another one."); // Prompt for a different choice
        player_turn(board, current_player); // Recursive call to allow the player to choose again
    }
}
