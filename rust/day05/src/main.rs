use nom::sequence::delimited;
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::IResult;
use std::fmt;
use std::env;

const DATA: &str = include_str!("input.txt");

enum Implementation {
    Part1,
    Part2,
}

impl Implementation {
    fn try_from_string(s: &String) -> Implementation {
        let last_s = s.chars().last().unwrap();
        match last_s {
            '1' => return Implementation::Part1,
            '2' => return Implementation::Part2,
            _ => panic!("Was expecting a command ending in 1 or 2 indicating which part to run")
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Crate(char);

#[derive(Debug, PartialEq)]
struct Move {
    amount: u32,
    from: usize,
    to: usize
}

struct Stacks(Vec<Vec<Crate>>);

impl fmt::Display for Stacks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, stack) in self.0.iter().enumerate() {
            writeln!(f, "{}: {:?}", i+1, stack)?
        }
        Ok(())
    }
}

impl Stacks {
    fn apply_move(&mut self, m: &Move, implementation: &Implementation) {
        if matches!(implementation, Implementation::Part1) {
            for _ in 0..m.amount {
                let c = self.0[m.from-1].pop().unwrap();
                self.0[m.to-1].push(c);
            }
        } else {
            let mut tmp = Vec::new();
            for _ in 0..m.amount {
                let c = self.0[m.from-1].pop().unwrap();
                tmp.push(c);
            }
            tmp.reverse();
            self.0[m.to-1].append(&mut tmp);
        }
    }

    fn print_top_crates(&self) {
        let mut s = "".to_string();
        for stack in &self.0 {
            match stack.last() {
                None => (),
                Some(c) => s.push_str(&c.0.to_string()),
            }
        }
        println!("{}", s);
    }
}

// Would have never figured out how to use nom without Sir Amos
// https://fasterthanli.me/series/advent-of-code-2022/part-5
fn parse_crate(str_chunk: &str) -> IResult<&str, Option<Crate>> {
    let parser = delimited(tag("["), take(1_usize), tag("]"));
    map(parser, |s: &str| Some(Crate(s.chars().next().unwrap())))(str_chunk)
}

fn parse_empty(str_chunk: &str) -> IResult<&str, Option<Crate>> {
    map(tag("   "), |_| None)(str_chunk)
}

fn parse_chunk(str_chunk: &str) -> IResult<&str, Option<Crate>> {
    alt((parse_crate, parse_empty))(str_chunk)
}

fn parse_line(line: &str) -> Vec<Option<Crate>> {
    let (remaining_str, crates) = separated_list0(tag(" "), parse_chunk)(line).unwrap();
    if remaining_str.len() != 0 {
        panic!("Remaining string not 0. Text left: {remaining_str}")
    }
    crates
}

fn parse_move(line: &str) -> IResult<&str, Move> {
    let (i, _) = tag("move ")(line)?;
    let (i, amount) = map(digit1, |s: &str| s.parse::<u32>().unwrap())(i)?;
    let (i, _) = tag(" from ")(i)?;
    let (i, from) = map(digit1, |s: &str| s.parse::<usize>().unwrap())(i)?;
    let (i, _) = tag(" to ")(i)?;
    let (i, to) = map(digit1, |s: &str| s.parse::<usize>().unwrap())(i)?;
  
    Ok((i, Move{amount, from, to}))
}

fn transpose_crates_to_stack(crates: Vec<Vec<Option<Crate>>>) -> Stacks {
    let li = crates.len();
    let lj = crates[0].len();
    let mut trans_crates = Vec::new();
    for j in 0..lj {
        let mut stack = Vec::new();
        for i in (0..li).rev() {
            match crates[i][j] {
                Some(c) => stack.push(c),
                None => ()
            }
        }
        trans_crates.push(stack);
    }
    Stacks(trans_crates)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = Implementation::try_from_string(&args[1]);
    let mut crate_parse = true;
    let mut moves = Vec::new();
    let mut crates = Vec::new();
    for line in DATA.lines() {
        if line.starts_with(" 1") {
            crate_parse = false
        }
        match (crate_parse, line.starts_with("move")) {
            (true, _) => crates.push(parse_line(line)),
            (false, true) => moves.push(parse_move(line).unwrap().1),
            (false, false) => (),
        }
    }

    let mut stacks = transpose_crates_to_stack(crates);

    println!("STARTING:");
    println!("{}", stacks);

    for m in moves {
        stacks.apply_move(&m, &part);
    }

    println!("\nEND:");
    println!("{}", stacks);

    stacks.print_top_crates();

}


#[cfg(test)]
mod tests {
    use super::*;

    fn simple_vec(crates: Vec<Option<Crate>>) -> Vec<char> {
        let mut x = Vec::new();
        for c in crates {
            if c.is_none() {
                x.push(' ');
            } else {
                x.push(c.unwrap().0);
            }
        }
        x
    }

    #[test]
    fn test_crate_parser(){
        let line1: &str = "    [D]    ";
        assert_eq!(simple_vec(parse_line(line1)), vec![' ', 'D', ' ']);
        let line2: &str = "[N] [C]    ";
        assert_eq!(simple_vec(parse_line(line2)), vec!['N', 'C', ' ']);
        let line3: &str = "[Z] [M] [P]";
        assert_eq!(simple_vec(parse_line(line3)), vec!['Z', 'M', 'P']);
    }

    #[test]
    fn test_move_parser(){
        let line: &str = "move 1 from 2 to 3";
        let m = parse_move(line).unwrap().1;
        assert_eq!(m, Move{amount:1, from:2, to:3});
    }
}
