// spaghetti code -_-
// part 1 only :(
use anyhow::Result;

struct Piece {
    width: i8,
    data: &'static [u8],
}

impl Piece {
    fn data(&self, x: i8) -> impl Iterator<Item = u8> + '_ {
        self.data
            .iter()
            .rev()
            .map(move |v| v << ((8 - x) - self.width))
    }
}

struct Cycle {
    offset: usize,
    data: &'static [u8],
}

impl Iterator for Cycle {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let x = Some(self.data[self.offset]);
        self.offset = (self.offset + 1) % self.data.len();
        x
    }
}

struct Chamber {
    jets: Vec<i8>,
    rocks: Vec<u8>,
    piece_num: usize,
    jet_num: usize,
}

impl Chamber {
    const PIECES: &[Piece] = &[
        Piece {
            width: 4,
            data: &[0b1111],
        },
        Piece {
            width: 3,
            data: &[0b010, 0b111, 0b010],
        },
        Piece {
            width: 3,
            data: &[0b001, 0b001, 0b111],
        },
        Piece {
            width: 1,
            data: &[0b1, 0b1, 0b1, 0b1],
        },
        Piece {
            width: 2,
            data: &[0b11, 0b11],
        },
    ];

    fn new(jets: Vec<i8>) -> Self {
        Self {
            jets,
            rocks: vec![],
            piece_num: 0,
            jet_num: 0,
        }
    }

    fn _print_row(row: u8) {
        let mut bit = 0x80;
        while bit > 1 {
            print!("{}", if (bit & row) != 0 { '#' } else { '.' });
            bit >>= 1;
        }
    }

    fn _print_piece(piece: &[u8; 4]) {
        for row in piece {
            Self::_print_row(*row);
            println!();
        }
    }

    fn _draw(&self) {
        self.rocks.iter().rev().for_each(|row| {
            print!("|");
            Self::_print_row(*row);
            println!("|");
        });
        println!("+-------+");
    }

    fn drop_piece(&mut self) {
        let piece = &Chamber::PIECES[self.piece_num];
        let piece_h = piece.data.len();
        self.piece_num = (self.piece_num + 1) % Chamber::PIECES.len();

        self.rocks.extend((0..piece_h + 3).map(|_| 0));

        let mut jet = self.jets[self.jet_num];
        self.jet_num = (self.jet_num + 1) % self.jets.len();

        let mut x = 2 + jet;
        let mut y = self.rocks.len() - piece_h - 1;

        loop {
            let dst = &self.rocks[y..y + piece_h];
            assert!(dst.len() == piece_h);
            let fits_down = piece.data(x).zip(dst).all(|v| v.0 & v.1 == 0);
            if !fits_down {
                y = y + 1;
                break;
            }

            jet = self.jets[self.jet_num];
            self.jet_num = (self.jet_num + 1) % self.jets.len();

            let new_x = (x + jet).max(0).min(7 - piece.width);

            let fits = piece.data(new_x).zip(dst).all(|v| v.0 & v.1 == 0);
            if fits {
                x = new_x;
            }

            if y == 0 {
                break;
            }

            y -= 1;
        }

        self.rocks[y..y + piece_h]
            .iter_mut()
            .zip(piece.data(x))
            .for_each(|(a, b)| *a |= b);

        while self.rocks.last().unwrap() == &0 {
            self.rocks.pop();
        }
    }
}

fn part1(chamber: &mut Chamber) {
    for _ in 0..2022 {
        chamber.drop_piece();
    }
    chamber._draw();
}

fn main() -> Result<()> {
    let input = include_str!("./day17.txt")
        .trim()
        .chars()
        .map(|c| c as i8 - 61)
        .collect::<Vec<i8>>();

    let mut chamber = Chamber::new(input);

    part1(&mut chamber);
    println!("height: {}", chamber.rocks.len());
    Ok(())
}
