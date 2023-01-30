use clearscreen;
use rand::Rng;
#[derive(Clone, Copy, Debug)]
enum Tile {
    Empty,
    Cross,
    Nought,
}
#[derive(Debug, Clone, Copy)]
struct Board {
    tiles: [[Tile; 3]; 3],
}
impl Board {
    fn new() -> Board {
        Board {
            tiles: [[Tile::Empty; 3]; 3],
        }
    }
    fn can_place(self, x: usize, y: usize) -> bool {
        if x > 2 || y > 2 {
            return false;
        }
        match self.tiles[2 - x][y] {
            Tile::Empty => return true,
            _ => return false,
        }
    }
    fn print(self) {
        for n in 0..3 {
            for i in 0..3 {
                match self.tiles[n][i] {
                    Tile::Empty => eprint!(" - "),
                    Tile::Cross => eprint!(" * "),
                    Tile::Nought => eprint!(" 0 "),
                }
            }
            println!("")
        }
    }
    fn won(self) -> bool {
        true
    }
}
pub fn noughts() {
    let mut board = Board::new();
    let mut turn: u8 = 0;
    loop {
        clearscreen::clear().unwrap();
        board.print();
        println!("Put in coords like x,y");
        loop {
            let mut input: String = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let coords: Vec<usize> = input
                .trim()
                .split(",")
                .map(|f| f.replace(" ", "").parse().unwrap())
                .collect();
            if board.can_place(coords[0], coords[1]) {
                if turn % 2 == 0 {
                    board.tiles[2 - coords[1]][coords[0]] = Tile::Cross;
                } else {
                    board.tiles[2 - coords[1]][coords[0]] = Tile::Nought;
                }
                break;
            } else {
                println!("Invalid coords do something else")
            }
        }
        turn += 1;
    }
}

fn ai_input() -> Vec<usize> {
    let x: usize = rand::thread_rng().gen_range(0..3);
    let y: usize = rand::thread_rng().gen_range(0..3);
    vec![x, y]
}
