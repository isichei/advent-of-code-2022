const TEST_DATA: &str = "R 4\nU 4\nL 3\nD 1";

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    U, L, D, R
}

#[derive(Debug, PartialEq, Eq)]
struct Move {
    d: Direction,
    v: u32,
}

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

struct Rope {
    knotts: [Option<Pos>; 10],
    knott_count: usize,
}

impl Rope {
    fn new(size: usize, start_pos: Pos) -> Rope {
        if size > 10 {
            panic!("Max size is 10")
        }
        let mut rope = Rope{knotts: [None; 10], knott_count: size};

        for i in 0..rope.knott_count {
            if i == size {
                break
            }
            rope.knotts[i] = Some(start_pos);
        }
        rope
    }

    fn update_knots(&mut self, d: Direction) {
        todo!();
    }
}

fn get_instructions(data: &str) -> Vec<Move> {
    // Parse the input data
    let mut instructions = Vec::new();
    for line in data.lines() {
        let m = match line.split_once(' ') {
            Some(("R", v)) =>  Move{d:Direction::R, v:v.parse::<u32>().unwrap()},
            Some(("L", v)) =>  Move{d:Direction::L, v:v.parse::<u32>().unwrap()},
            Some(("U", v)) =>  Move{d:Direction::U, v:v.parse::<u32>().unwrap()},
            Some(("D", v)) =>  Move{d:Direction::D, v:v.parse::<u32>().unwrap()},
            _ => panic!("Could not parse line"),
            };
        instructions.push(m);
        }
    instructions
    }

    #[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_instructions() {
        let instructions = get_instructions(TEST_DATA);

        let mut expected = Vec::new();
        expected.push(Move{d:Direction::R, v:4_u32});
        expected.push(Move{d:Direction::U, v:4_u32});
        expected.push(Move{d:Direction::L, v:3_u32});
        expected.push(Move{d:Direction::D, v:1_u32});
        assert_eq!(instructions, expected);
    }

    #[test]
    fn test_rope() {
        !todo!();
    }
}