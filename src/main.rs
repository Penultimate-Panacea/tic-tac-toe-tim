use text_io::read;

const SIZE: usize = 3;

#[derive(PartialEq, Debug)]
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

fn get_move() -> Vec<usize> {
    let input: String = read!();
    let moves: std::str::Split<'_, &str> = input.split(","); 
    let mut move_vec:Vec<usize> = vec![0,0,0,0];
    moves.collect::<Vec<_>>().into_iter().for_each(|gamemove_ptr| {
        let gamemove = todo!("&str into usize" );
        move_vec.push(gamemove);
    });
    move_vec
}

fn main() {
    let mut game = TicTacToe4D::new();
    while game.check_winner() == None {
        println!("{:?}", game.board);
        println!("It is {:?}'s turn.", game.current_player);
        println!("Enter x,y,z,w coordinates of move:");
        let game_move = get_move();
        game.make_move(game_move[0], game_move[1], game_move[2], game_move[3]);
    }
}
