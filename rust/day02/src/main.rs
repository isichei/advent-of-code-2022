const DATA: &str = include_str!("input.txt");

#[derive(Clone, Copy)]
enum Interpretation {
    Part1,
    Part2,
}

#[derive(Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors
}

struct Match {
    p1: Choice,
    p2: Choice,
}

fn char_to_choice(c: char) -> Choice {
    let choice = match c {
        'X' | 'A' => Choice::Rock,
        'Y' | 'B' => Choice::Paper,
        'Z' | 'C' => Choice::Scissors,
        _ => panic!("Unexpected Choice {}", c)
    };
    choice
}

fn char_to_expected_outcome(c: char, oppo: &Choice) -> Choice {
    let choice = match c {
        // Lose
        'X' => match *oppo {
            Choice::Rock => Choice::Scissors, 
            Choice::Paper => Choice::Rock, 
            Choice::Scissors => Choice::Paper, 
        },
        // Draw
        'Y' => oppo.clone(),
        // Win
        'Z' => match *oppo {
            Choice::Rock => Choice::Paper, 
            Choice::Paper => Choice::Scissors, 
            Choice::Scissors => Choice::Rock, 
        },
        _ => panic!("Unknown choice: {}", c)
    };
    choice
}

fn parse_line(line: &str, interpretation: Interpretation) -> Match {
    let mut char_iter = line.chars();
    let p1 = char_to_choice(char_iter.next().unwrap());
    char_iter.next();
    let p2 = match interpretation {
        Interpretation::Part1 => char_to_choice(char_iter.next().unwrap()),
        Interpretation::Part2 => char_to_expected_outcome(char_iter.next().unwrap(), &p1),
    };
    Match {p1, p2}
}

fn read_matches(data: &str, interpretation: Interpretation) -> Vec<Match> {
    let mut matches: Vec<Match> = Vec::new();
    for line in data.lines() {
        matches.push(parse_line(line, interpretation));
    }
    matches
}

fn score_outcome(m: &Match) -> u32 {
    // match me then oppo
    let score = match m.p2 {
        Choice::Rock => match m.p1 {
            Choice::Rock => 3,
            Choice::Paper => 0,
            Choice::Scissors => 6,
        },
        Choice::Paper => match m.p1 {
            Choice::Rock => 6,
            Choice::Paper => 3,
            Choice::Scissors => 0,
        },
        Choice::Scissors => match m.p1 {
            Choice::Rock => 0,
            Choice::Paper => 6,
            Choice::Scissors => 3,
        }
    };
    score
}

fn score_choice(m: &Match) -> u32 {
    // match me then oppo
    let score = match m.p2 {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    };
    score
}

fn score_matches(match_vec: &Vec<Match>) -> u32 {
    match_vec.iter().map(| m | score_choice(m) + score_outcome(m)).sum()
}

fn main() {
    let part1 = read_matches(DATA, Interpretation::Part1);
    let part2 = read_matches(DATA, Interpretation::Part2);
    println!("Part 1: {}", score_matches(&part1));
    println!("Part 2: {}", score_matches(&part2));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let test_data = include_str!("example.txt");
        let matches = read_matches(test_data, Interpretation::Part1);
        assert!(matches!((matches[0].p1, matches[0].p2), (Choice::Rock, Choice::Paper)));
        assert_eq!(matches.len(), 3);
    }

    #[test]
    fn test_example() {
        let test_data = include_str!("example.txt");
        let matches = read_matches(test_data, Interpretation::Part1);
        let total_scores: u32 = score_matches(&matches);
        assert_eq!(total_scores, 15);
    }
}
