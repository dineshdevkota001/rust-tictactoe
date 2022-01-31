mod board;

fn main() {
    let mut board = board::Board::new();
    // board.take_input();
    board.game_loop()
    // println!("{}", board.check_board())
}
