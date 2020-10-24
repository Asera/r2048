use rand::random;

pub struct GameState {
    pub score_current: u32,
    pub score_best: u32,
    pub field: Vec<Vec<Box<Tile>>>,
    pub changed_this_turn: bool,
}

#[derive(Copy, Clone)]
pub struct Tile {
    pub power: u32,
    pub merged_this_turn: bool,
}

impl Tile {
    pub fn init(power: u32) -> Tile {
        Tile { power, merged_this_turn: false }
    }
}

impl GameState {
    pub fn init() -> GameState {
        let x_pos_1: u32 = random::<u32>() % 4;
        let y_pos_1: u32 = random::<u32>() % 4;
        let element_pow_1 = get_initial_pow();

        let mut x_pos_2 = random::<u32>() % 4;
        let mut y_pos_2 = random::<u32>() % 4;
        let element_pow_2 = get_initial_pow();

        loop {
            if x_pos_1 != x_pos_2 && y_pos_1 != y_pos_2 {break}
            x_pos_2 = random::<u32>() % 4;
            y_pos_2 = random::<u32>() % 4;
        }

        let mut initial_field = vec![];

        for x in 0..4 {
            let mut initial_row = vec![];
            for y in 0..4 {
                if x == x_pos_1 && y == y_pos_1 {
                    initial_row.push(Box::new(Tile::init(element_pow_1)));
                } else if x == x_pos_2 && y == y_pos_2 {
                    initial_row.push(Box::new(Tile::init(element_pow_2)));
                } else {
                    initial_row.push(Box::new(Tile::init(0)));
                }
            }
            initial_field.push(initial_row);
        }

        return GameState {
            score_current: 0,
            score_best: 0,
            field: initial_field,
            changed_this_turn: false
        };
    }

    pub fn shift_left(&mut self) {
        // from far left column to far right column:
        //   try to move each element on one position to the left
        //   if left cell is empty - move block to the left
        //   if left cell is occupied by the same power - merge blocks and increase score by sum power
        for x in 1..4 {
            for y in 0..4 {
                // Skipping all zero tiles
                if self.field[x][y].power == 0 {continue;}

                let mut i = 1;
                while x - i > 0 && self.field[x - i][y].power == 0 {
                    i += 1;
                }
                if self.field[x - i][y].power == 0 {
                    self.field[x - i][y].power = self.field[x][y].power;
                    self.field[x][y].power = 0;
                    self.changed_this_turn = true;
                } else if self.field[x - i][y].power == self.field[x][y].power {
                    if !self.field[x - i][y].merged_this_turn {
                        self.field[x - i][y].power += self.field[x][y].power;
                        self.field[x][y].power = 0;
                        self.score_current += self.field[x - i][y].power;
                        self.field[x - i][y].merged_this_turn = true;
                        self.changed_this_turn = true;
                    }
                } else {
                    if i != 1 {
                        self.field[x - i + 1][y].power = self.field[x][y].power;
                        self.field[x][y].power = 0;
                        self.changed_this_turn = true;
                    }
                }
            }
        }

        self.clear_merge_flags();
    }

    pub fn shift_right(&mut self) {
        // from far right column to far left column:
        //   try to move each element on one position to the right
        //   if right cell is empty - move block to the right
        //   if right cell is occupied by the same power - merge blocks and increase score by sum power
        for x in (0..3).rev() {
            for y in 0..4 {
                // Skipping all zero tiles
                if self.field[x][y].power == 0 {continue;}

                let mut i = 1;
                while x + i < 3 && self.field[x + i][y].power == 0 {
                    i += 1;
                }
                if self.field[x + i][y].power == 0 {
                    self.field[x + i][y].power = self.field[x][y].power;
                    self.field[x][y].power = 0;
                    self.changed_this_turn = true;
                } else if self.field[x + i][y].power == self.field[x][y].power {
                    if !self.field[x + i][y].merged_this_turn {
                        self.field[x + i][y].power += self.field[x][y].power;
                        self.field[x][y].power = 0;
                        self.score_current += self.field[x + i][y].power;
                        self.field[x + i][y].merged_this_turn = true;
                        self.changed_this_turn = true;
                    }
                } else {
                    if i != 1 {
                        self.field[x + i - 1][y].power = self.field[x][y].power;
                        self.field[x][y].power = 0;
                        self.changed_this_turn = true;
                    }
                }
            }
        }

        self.clear_merge_flags();
    }

    pub fn shift_up(&mut self) {
        for x in 0..4 {
            for y in 1..4 {
                // Skipping all zero tiles
                if self.field[x][y].power == 0 {continue;}

                let mut i = 1;
                while y - i > 0 && self.field[x][y - i].power == 0 {
                    i += 1;
                }

                if self.field[x][y - i].power == 0 {
                    self.field[x][y - i].power = self.field[x][y].power;
                    self.field[x][y].power = 0;
                    self.changed_this_turn = true;
                } else if self.field[x][y - i].power == self.field[x][y].power {
                    if !self.field[x][y - i].merged_this_turn {
                        self.field[x][y - i].power += self.field[x][y].power;
                        self.field[x][y].power = 0;
                        self.score_current += self.field[x][y - i].power;
                        self.field[x][y - i].merged_this_turn = true;
                        self.changed_this_turn = true;
                    }
                } else {
                    if i != 1 {
                        self.field[x][y - i + 1].power = self.field[x][y].power;
                        self.field[x][y].power = 0;
                        self.changed_this_turn = true;
                    }
                }
            }
        }

        self.clear_merge_flags();
    }

    pub fn shift_down(&mut self) {
        for x in 0..4 {
            for y in (0..3).rev() {
                // Skipping all zero tiles
                if self.field[x][y].power == 0 {continue;}

                let mut i = 1;
                while y + i < 3 && self.field[x][y + i].power == 0 {
                    i += 1;
                }
                if self.field[x][y + i].power == 0 {
                    self.field[x][y + i].power = self.field[x][y].power;
                    self.field[x][y].power = 0;
                    self.changed_this_turn = true;
                } else if self.field[x][y + i].power == self.field[x][y].power {
                    if !self.field[x][y + i].merged_this_turn {
                        self.field[x][y + i].power += self.field[x][y].power;
                        self.field[x][y].power = 0;
                        self.score_current += self.field[x][y + i].power;
                        self.field[x][y + i].merged_this_turn = true;
                        self.changed_this_turn = true;
                    }
                } else {
                    if i != 1 {
                        self.field[x][y + i - 1].power = self.field[x][y].power;
                        self.field[x][y].power = 0;
                        self.changed_this_turn = true;
                    }
                }
            }
        }

        self.clear_merge_flags();
    }

    fn clear_merge_flags(&mut self) {
        for line in self.field.iter_mut() {
            for tile in line.iter_mut() {
                tile.merged_this_turn = false;
            }
        }
    }

    pub fn add_tile(&mut self) {
        let mut tile_x_pos = random::<usize>() % 4;
        let mut tile_y_pos = random::<usize>() % 4;
        let tile_pow = get_initial_pow();

        loop {
            if self.field[tile_x_pos][tile_y_pos].power == 0 {break}
            tile_x_pos = random::<usize>() % 4;
            tile_y_pos = random::<usize>() % 4;
        }

        self.field[tile_x_pos][tile_y_pos] = Box::new(Tile::init(tile_pow));
    }

    pub fn clear_changed_flag(&mut self) {
        self.changed_this_turn = false;
    }

    pub fn no_more_turns(&self) -> bool {
        // check on empty tiles
        for line in self.field.iter() {
            for tile in line.iter() {
                if tile.power == 0 {
                    return false;
                }
            }
        }

        // check on adjacent tiles with similar power
        for (x, line) in self.field.iter().enumerate() {
            for (y, tile) in line.iter().enumerate() {
                if (x != 0) && (tile.power == self.field[x - 1][y].power) {
                    return false;
                }
                if (x != 3) && (tile.power == self.field[x + 1][y].power) {
                    return false;
                }
                if (y != 3) && (tile.power == self.field[x][y + 1].power) {
                    return false;
                }
                if (y != 0) && (tile.power == self.field[x][y - 1].power) {
                    return false;
                }
            }
        }

        true
    }

    pub fn restart(&self) -> GameState {
        let mut empty_state = GameState::init();
        if self.score_best < self.score_current {
            empty_state.score_best = self.score_current;
        } else {
            empty_state.score_best = self.score_best;
        }
        empty_state
    }
}

fn get_initial_pow() -> u32 {
    let rand_seed: u32 = random::<u32>() % 10;
    if rand_seed == 0 {
        return 4;
    }
    return 2;
}