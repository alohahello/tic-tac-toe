use std::io::{self, Write}; // Importa la biblioteca estándar en Rust para entrada y salida

fn main() {
    let mut board = [[' '; 3]; 3]; // Inicializa el tablero con espacios en blanco
    let mut current_player = 'X'; // Inicializa el jugador actual como 'X'

    loop {
        print_board(&board); // Muestra el estado actual del tablero
        player_turn(&mut board, &mut current_player); // Permite al jugador actual realizar un movimiento

        // Verifica si hay un ganador o un empate
        if check_win(&board, current_player) { // Verifica si el jugador actual ha ganado
            println!("¡El jugador {} gana!", current_player); // Imprime un mensaje indicando al ganador
            break; // Sale del bucle del juego
        } else if is_draw(&board) { // Verifica si el juego ha terminado en empate
            println!("¡Es un empate!"); // Imprime un mensaje indicando un empate
            break; // Sale del bucle del juego
        }

        // Cambia de jugador para el próximo turno
        current_player = if current_player == 'X' { 'O' } else { 'X' }; // Alterna entre 'X' y 'O'
    }
}

// Función para imprimir el tablero del juego
fn print_board(board: &[[char; 3]; 3]) {
    for row in board { // Itera a través de cada fila del tablero
        for &cell in row { // Itera a través de cada celda en la fila
            print!(" {} |", cell); // Imprime el valor de la celda seguido de un separador
        }
        println!(); // Mueve a la siguiente línea después de imprimir cada fila
        println!("---+---+---"); // Imprime un separador entre las filas
    }
    println!(); // Imprime una línea en blanco después de imprimir el tablero
}

// Función para verificar si un jugador ha ganado el juego
fn check_win(board: &[[char; 3]; 3], player: char) -> bool {
    // Verifica las filas y columnas para determinar un ganador
    for i in 0..3 {
        if (board[i][0] == player && board[i][1] == player && board[i][2] == player) || // Verifica las filas
           (board[0][i] == player && board[1][i] == player && board[2][i] == player) { // Verifica las columnas
            return true; // Devuelve true si el jugador ha ganado
        }
    }
    // Verifica las diagonales para determinar un ganador
    if (board[0][0] == player && board[1][1] == player && board[2][2] == player) || // Verifica la diagonal principal
       (board[0][2] == player && board[1][1] == player && board[2][0] == player) { // Verifica la diagonal secundaria
        return true; // Devuelve true si el jugador ha ganado
    }
    false // Devuelve false si el jugador no ha ganado
}

// Función para verificar si el juego ha terminado en empate
fn is_draw(board: &[[char; 3]; 3]) -> bool {
    !board.iter().flatten().any(|&cell| cell == ' ') // Devuelve true si no hay celdas vacías en el tablero
}

// Función para permitir que un jugador realice un movimiento
fn player_turn(board: &mut [[char; 3]; 3], current_player: &mut char) {
    println!("Turno del jugador {}", current_player); // Imprime un mensaje indicando de quién es el turno

    let mut row_str = String::new(); // Crea una cadena vacía para almacenar la entrada del usuario para la fila
    let mut col_str = String::new(); // Crea una cadena vacía para almacenar la entrada del usuario para la columna
    let row: usize; // Inicializa la variable para almacenar el índice de fila
    let col: usize; // Inicializa la variable para almacenar el índice de columna

    // Bucle para solicitar al usuario una entrada de fila válida
    loop {
        println!("Ingresa la fila (1-3): ");
        io::stdout().flush().unwrap(); // Limpia el búfer de salida para asegurarse de que se muestre el mensaje
        row_str.clear(); // Borra el búfer de cadenas para la entrada de fila
        io::stdin().read_line(&mut row_str).unwrap(); // Lee la entrada del usuario para la fila

        // Analiza la entrada de fila y la valida
        match row_str.trim().parse::<usize>() {
            Ok(num) if num >= 1 && num <= 3 => {
                row = num - 1; // Convierte la entrada del usuario a indexación basada en cero
                break; // Sale del bucle si la entrada es válida
            }
            _ => println!("Entrada inválida. Por favor, ingresa un número entre 1 y 3."), // Solicita una entrada válida
        }
    }

    // Bucle para solicitar al usuario una entrada de columna válida
    loop {
        println!("Ingresa la columna (1-3): ");
        io::stdout().flush().unwrap(); // Limpia el búfer de salida para asegurarse de que se muestre el mensaje
        col_str.clear(); // Borra el búfer de cadenas para la entrada de columna
        io::stdin().read_line(&mut col_str).unwrap(); // Lee la entrada del usuario para la columna

        // Analiza la entrada de columna y la valida
        match col_str.trim().parse::<usize>() {
            Ok(num) if num >= 1 && num <= 3 => {
                col = num - 1; // Convierte la entrada del usuario a indexación basada en cero
                break; // Sale del bucle si la entrada es válida
            }
            _ => println!("Entrada inválida. Por favor, ingresa un número entre 1 y 3."), // Solicita una entrada válida
        }
    }

    // Verifica si la celda elegida está vacía, y de ser así, la marca con el símbolo del jugador actual
    if board[row][col] == ' ' {
        board[row][col] = *current_player; // Marca la celda elegida con el símbolo del jugador actual
    } else {
        println!("Esta posición ya está ocupada. Por favor, elige otra."); // Solicita una selección diferente
        player_turn(board, current_player); // Llamada recursiva para permitir al jugador elegir nuevamente
    }