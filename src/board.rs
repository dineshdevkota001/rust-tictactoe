use std::io;
use std::io::Write;

pub struct Board {
    board: [[i8; 3]; 3],
    current_turn: i8,
}

fn clear_screen() {
    print!("\x1B[2J");
    io::stdout().flush().unwrap();
}

fn get_render_string(value: i8) -> String {
    match value {
        1 => return String::from("X"),
        2 => return String::from("O"),
        _ => return String::from(" "),
    }
}

impl Board {
    fn check_board(&self) -> bool {
        // Check rows
        //Column
        for loop_num in 0..3 {
            let diag_value = self.board[loop_num][loop_num];
            if diag_value == 0 {
                continue;
            }
            for col in 0..3 {
                if diag_value != self.board[loop_num][col] {
                    break;
                }
                if col == 2 {
                    println!("col {}", loop_num);
                    return true;
                }
            }
            for row in 0..3 {
                if diag_value != self.board[row][loop_num] {
                    break;
                }
                if row == 2 {
                    println!("row {}", loop_num);
                    return true;
                }
            }
        }
        //Diagonal
        let center_element = self.board[1][1];
        if center_element == 0 {
            return false;
        }
        if self.board[0][0] == center_element && center_element == self.board[2][2] {
            println!("diagonal right");
            return true;
        }
        if self.board[0][2] == center_element && center_element == self.board[2][0] {
            println!("diagonal left");
            return true;
        }

        return false;
    }

    fn get_current_player_name(&self) -> &str {
        if self.current_turn == 1 {
            return "X";
        }
        return "O";
    }

    fn take_input(&self) -> i8 {
        let mut input_text = String::new();
        println!(
            "Please input where to put the {}",
            self.get_current_player_name()
        );
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let trimmed = input_text.trim();
        match trimmed.parse::<i8>() {
            Ok(i) => {
                if i > 9 || i < 1 {
                    println!("Out of scope");
                    return self.take_input();
                }
                if self.get_value_at_index(i - 1) != 0 {
                    println!("Cannot Input there");
                    return self.take_input();
                }
                return i;
            }
            Err(..) => {
                println!("this was not an integer: {}", trimmed);
                return self.take_input();
            }
        };
    }

    fn render_game(&self) {
        clear_screen();
        clear_screen();
        for row in 0..3 {
            if row != 0 {
                println!("- - -");
            }
            for col in 0..3 {
                if col != 0 {
                    print!("|");
                    io::stdout().flush().unwrap();
                }

                print!("{}", get_render_string(self.board[row][col]));

                io::stdout().flush().unwrap();
            }
            println!("");
        }
    }

    fn change_turn(&mut self) {
        if self.current_turn == 1 {
            self.current_turn = 2;
        } else {
            self.current_turn = 1;
        }
    }

    fn get_value_at_index(&self, input: i8) -> i8 {
        let column = input % 3;
        let row = 2 - (input - column) / 3;
        return self.board[row as usize][column as usize];
    }

    fn input_text(&mut self, input: i8) {
        let column = input % 3;
        let row = 2 - (input - column) / 3;
        self.board[row as usize][column as usize] = self.current_turn;
    }

    pub fn game_loop(&mut self) {
        loop {
            self.render_game();
            let input = self.take_input();
            self.input_text(input - 1);
            if self.check_board() {
                self.render_game();
                println!("{} wins", self.get_current_player_name());
                break;
            }
            self.change_turn();
        }
    }
    pub fn new() -> Board {
        return Board {
            board: [[0, 0, 0], [0, 0, 0], [0, 0, 0]],
            current_turn: 1,
        };
    }
}
