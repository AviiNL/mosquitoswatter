use rand::Rng;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

pub enum Tile {
    Mine,
    Empty,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl From<usize> for Tile {
    fn from(val: usize) -> Self {
        match val {
            0 => Tile::Empty,
            1 => Tile::One,
            2 => Tile::Two,
            3 => Tile::Three,
            4 => Tile::Four,
            5 => Tile::Five,
            6 => Tile::Six,
            7 => Tile::Seven,
            8 => Tile::Eight,
            _ => Tile::Empty,
        }
    }
}

pub struct Row {
    tiles: Vec<Tile>,
}

impl Row {
    pub fn get_tiles(&self) -> &[Tile] {
        &self.tiles
    }
}

pub struct Board {
    rows: Vec<Row>,
}

impl Board {
    pub fn get_rows(&self) -> &[Row] {
        &self.rows
    }
}

pub struct Minesweeper {
    mines: Vec<usize>,
    width: usize,
    height: usize,
}

impl Minesweeper {
    pub fn empty(width: usize, height: usize) -> Self {
        Self {
            mines: vec![],
            width,
            height,
        }
    }

    pub fn random(width: usize, height: usize, mines: usize) -> Self {
        let mut s = Self::empty(width, height);
        let total = width * height;
        let mines = mines.max(1).min(total - 1);

        while s.mines.len() < mines {
            let num = rand::thread_rng().gen_range(0..total);

            if s.mines.contains(&num) {
                continue; // generate a new one
            }

            s.mines.push(num);
        }

        s
    }

    pub fn random_seed<H: core::hash::Hash>(
        width: usize,
        height: usize,
        mines: usize,
        seed: H,
    ) -> Self {
        let mut s = Self::empty(width, height);
        let total = width * height;
        let mines = mines.max(1).min(total - 1);

        let mut rng: Pcg64 = Seeder::from(seed).make_rng();

        while s.mines.len() < mines {
            let num = rng.gen_range(0..total);

            if s.mines.contains(&num) {
                continue; // generate a new one
            }

            s.mines.push(num);
        }

        s
    }

    pub fn get_board(&self) -> Board {
        let mut b = Board { rows: vec![] };

        for y in 0..self.height {
            let mut row = Row { tiles: vec![] };

            for x in 0..self.width {
                if self.mines.contains(&((y * self.width) + x)) {
                    row.tiles.push(Tile::Mine);
                    continue;
                }

                let icon = self.check_neigbors(x, y);
                row.tiles.push(icon.into());
            }
            b.rows.push(row);
        }

        b
    }

    fn check_neigbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for y in y.max(1) - 1..=y + 1 {
            for x in x.max(1) - 1..=(x + 1).min(self.width - 1) {
                let index = &((y * self.width) + x);
                if self.mines.contains(index) {
                    count += 1;
                }
            }
        }

        count
    }
}

// impl Display for Minesweeper {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let mut output = String::new();
//         for y in 0..self.height {
//             for x in 0..self.width {
//                 if self.mines.contains(&((y * self.width) + x)) {
//                     output += "||:mosquito:||";
//                     continue;
//                 }

//                 let icon = self.check_neigbors(x, y);

//                 output += &format!("||{}||", icon).to_string();
//             }
//             output += "\n";
//         }

//         write!(f, "{}", output)
//     }
// }

// fn generate_board(cols: usize, rows: usize, mosquitos: usize) -> String {
//     let mut output = String::new();

//     let rows = rows.min(9).max(3);
//     let cols = cols.min(11).max(3);
//     let total = rows * cols;

//     let num_mosquitos = mosquitos.max(1).min(total - 1);

//     let mut mosquitos = vec![];

//     while mosquitos.len() < num_mosquitos {
//         let num = rand::thread_rng().gen_range(0..total);

//         if mosquitos.contains(&num) {
//             continue; // generate a new one
//         }

//         mosquitos.push(num);
//     }

// for y in 0..rows {
//     for x in 0..cols {
//         if mosquitos.contains(&((y * cols) + x)) {
//             output += "||:mosquito:||";
//             continue;
//         }

//         let icon = check_neigbors(&mosquitos, cols, (x, y));

//         output += &format!("||{}||", icon).to_string();
//     }
//     output += "\n";
// }

//     output
// }

// fn check_neigbors(mosquitos: &[usize], cols: usize, pos: (usize, usize)) -> usize {
//     let mut count = 0;
//     for y in pos.1.max(1) - 1..=pos.1 + 1 {
//         for x in pos.0.max(1) - 1..=(pos.0 + 1).min(cols - 1) {
//             let index = &((y * cols) + x);
//             if mosquitos.contains(index) {
//                 count += 1;
//             }
//         }
//     }

//     count
// }
