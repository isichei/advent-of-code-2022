use std::ops;

const TEST_DATA: &str = "R 4\nU 4\nL 3\nD 1";

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    U,
    L,
    D,
    R,
}

#[derive(Debug, PartialEq, Eq)]
struct Move {
    d: Direction,
    v: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}

impl ops::Add<Pos> for Pos {
    type Output = Pos;

    fn add(self, _rhs: Pos) -> Pos {
        Pos {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

fn get_pos_change(p1: &Pos, p2: &Pos) -> Pos {
    let diff_pos = Pos {
        x: p1.x - p2.x,
        y: p1.y - p2.y,
    };
    if diff_pos.x > 2 || diff_pos.x < -2 || diff_pos.y < -2 || diff_pos.y > 2 {
        panic!("Change too large");
    }
    match (diff_pos.x, diff_pos.y) {
        // Diag
        (-2, 1) | (-1, 2) | (-2, 2) => return Pos { x: -1, y: 1 },
        (1, 2) | (2, 1) | (2, 2) => return Pos { x: 1, y: 1 },
        (1, -2) | (2, -1) | (2, -2) => return Pos { x: 1, y: -1 },
        (-1, -2) | (-2, -1) | (-2, -2) => return Pos { x: -1, y: -1 },

        // Normal
        (0, 2) => return Pos { x: 0, y: 1 },
        (0, -2) => return Pos { x: 0, y: -1 },
        (2, 0) => return Pos { x: 1, y: 0 },
        (-2, 0) => return Pos { x: -1, y: 0 },
        _ => return Pos { x: 0, y: 0 },
    }
}

struct Rope {
    knotts: [Pos; 10],
    knott_count: usize,
}

impl Rope {
    fn new(size: usize, start_pos: Pos) -> Rope {
        if size > 10 {
            panic!("Max size is 10")
        }
        Rope {
            knotts: [start_pos; 10],
            knott_count: size,
        }
    }

    fn update_knots(&mut self, m: &Move) {
        let mut move_count = 0;
        while move_count < m.v {
            let new_pos = self.knotts[0] + direction_to_pos(&m.d);
            self.knotts[0] = new_pos;

            for i in 1..self.knott_count {
                let change = get_pos_change(&self.knotts[i - 1], &self.knotts[i]);
                self.knotts[i] = self.knotts[i] + change;
            }
            move_count += 1;
        }
    }
}

fn direction_to_pos(d: &Direction) -> Pos {
    match *d {
        Direction::U => return Pos { x: 0, y: 1 },
        Direction::D => return Pos { x: 0, y: -1 },
        Direction::L => return Pos { x: -1, y: 0 },
        Direction::R => return Pos { x: 1, y: 0 },
    }
}

fn get_instructions(data: &str) -> Vec<Move> {
    // Parse the input data
    let mut instructions = Vec::new();
    for line in data.lines() {
        let m = match line.split_once(' ') {
            Some(("R", v)) => Move {
                d: Direction::R,
                v: v.parse::<u32>().unwrap(),
            },
            Some(("L", v)) => Move {
                d: Direction::L,
                v: v.parse::<u32>().unwrap(),
            },
            Some(("U", v)) => Move {
                d: Direction::U,
                v: v.parse::<u32>().unwrap(),
            },
            Some(("D", v)) => Move {
                d: Direction::D,
                v: v.parse::<u32>().unwrap(),
            },
            _ => panic!("Could not parse line"),
        };
        instructions.push(m);
    }
    instructions
}

// cargo test --package day09 --bin day09 -- knotts::tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_instructions() {
        let instructions = get_instructions(TEST_DATA);

        let mut expected = Vec::new();
        expected.push(Move {
            d: Direction::R,
            v: 4_u32,
        });
        expected.push(Move {
            d: Direction::U,
            v: 4_u32,
        });
        expected.push(Move {
            d: Direction::L,
            v: 3_u32,
        });
        expected.push(Move {
            d: Direction::D,
            v: 1_u32,
        });
        assert_eq!(instructions, expected);
    }

    #[test]
    fn test_pos() {
        let p1 = Pos { x: 0, y: 1 };
        let p2 = Pos { x: -1, y: 2 };

        assert_eq!(p1 + p2, Pos { x: -1, y: 3 });
        assert_eq!(p2 + p1, Pos { x: -1, y: 3 });
    }

    #[test]
    fn test_rope() {
        let mut test_rope = Rope::new(2, Pos { x: 0, y: 0 });
        let instructions = get_instructions(TEST_DATA);
        for inst in instructions {
            test_rope.update_knots(&inst);
        }

        assert_eq!(test_rope.knotts[0], Pos { x: 1, y: 3 });
        assert_eq!(test_rope.knotts[1], Pos { x: 2, y: 4 });
    }
}
