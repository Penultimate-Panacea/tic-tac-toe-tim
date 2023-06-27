const SIZE: usize = 4;

enum Player {
    X,
    O,
}

struct TicTacToe4D {
    board: [[[[char; SIZE]; SIZE]; SIZE]; SIZE],
    current_player: Player,
}

impl TicTacToe4D {
    fn new() -> Self {
        TicTacToe4D {
            board: [[[[' '; SIZE]; SIZE]; SIZE]; SIZE],
            current_player: Player::X,
        }
    }

    fn make_move(&mut self, x: usize, y: usize, z: usize, w: usize) -> bool {
        if self.board[x][y][z][w] == ' ' {
            self.board[x][y][z][w] = match self.current_player {
                Player::X => 'X',
                Player::O => 'O',
            };
            self.current_player = match self.current_player {
                Player::X => Player::O,
                Player::O => Player::X,
            };
            true
        } else {
            false
        }
    }

    fn check_winner(&self) -> Option<Player> {
        let lines_to_check: Vec<Vec<(usize, usize, usize, usize)>> = vec![
            // Horizontal lines
            (0..SIZE)
                .flat_map(|x| {
                    (0..SIZE).flat_map(move |y| {
                        (0..SIZE).map(move |z| (x, y, z, 0)).collect::<Vec<_>>()
                    })
                })
                .collect(),
            // Vertical lines
            (0..SIZE)
                .flat_map(|x| {
                    (0..SIZE).flat_map(move |z| {
                        (0..SIZE).map(move |y| (x, y, z, 0)).collect::<Vec<_>>()
                    })
                })
                .collect(),
            // Depth lines
            (0..SIZE)
                .flat_map(|y| {
                    (0..SIZE).flat_map(move |z| {
                        (0..SIZE).map(move |x| (x, y, z, 0)).collect::<Vec<_>>()
                    })
                })
                .collect(),
            // Diagonal lines
            (0..SIZE)
                .flat_map(|x| {
                    (0..SIZE).map(move |y| (x, y, y, 0)).collect::<Vec<_>>()
                })
                .collect(),
            (0..SIZE)
                .flat_map(|x| {
                    (0..SIZE).map(move |y| (x, y, SIZE - y - 1, 0)).collect::<Vec<_>>()
                })
                .collect(),
        ];

        for line in lines_to_check {
            let symbols: Vec<char> = line
                .iter()
                .map(|&(x, y, z, w)| self.board[x][y][z][w])
                .collect();

            if symbols.iter().all(|&s| s == 'X') {
                return Some(Player::X);
            } else if symbols.iter().all(|&s| s == 'O') {
                return Some(Player::O);
            }
        }

        None
    }
}


fn main() {
    let mut game = TicTacToe4D::new();

}
