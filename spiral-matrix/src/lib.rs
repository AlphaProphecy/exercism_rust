#[derive(Copy, Clone)]
enum Dir {
    Right,
    Down,
    Left,
    Up,
}
struct Turtle {
    pos: (i32, i32),
    dir: Dir,
    count: u32,
}
impl Turtle {
    pub fn inc_move(&mut self, mut map: Vec<Vec<u32>>) -> (Option<Vec<Vec<u32>>>, Vec<Vec<u32>>) {
        let mut tested_dir: Vec<Dir> = vec![];
        let mut found = false;
        let mut new = (0, 0);

        map[self.pos.0 as usize][self.pos.1 as usize] = self.count;
        self.count += 1;

        while tested_dir.len() < 4 && !found {
            new = match self.dir {
                Dir::Right => (self.pos.0, self.pos.1 + 1),
                Dir::Down => (self.pos.0 + 1, self.pos.1),
                Dir::Left => (self.pos.0, self.pos.1 - 1),
                Dir::Up => (self.pos.0 - 1, self.pos.1),
            };

            if new.0 < map.len() as i32
                && new.1 < map.len() as i32
                && new.0 >= 0
                && new.1 >= 0
                && map[new.0 as usize][new.1 as usize] == 0
            {
                found = true;
            } else {
                tested_dir.push(self.dir);

                match self.dir {
                    Dir::Right => self.dir = Dir::Down,
                    Dir::Down => self.dir = Dir::Left,
                    Dir::Left => self.dir = Dir::Up,
                    Dir::Up => self.dir = Dir::Right,
                }
            }
        }
        if !found {
            return (None, map);
        } else {
            self.pos = new;
            return (Some(map), vec![vec![]]);
        }
    }
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0 as u32; size as usize]; size as usize];
    let position = (0, 0);

    let mut turtle = Turtle {
        pos: position,
        dir: Dir::Right,
        count: 1,
    };
    if size == 0 {
        return matrix;
    }
    loop {
        let (new_map, old_map) = turtle.inc_move(matrix);
        match new_map {
            Some(v) => matrix = v,
            None => {
                matrix = old_map;
                break;
            }
        }
    }
    return matrix;
}
