const DATA: &str = include_str!("input.txt");

#[derive(Debug)]
struct Assignment {
    start: u32,
    end: u32,
}

impl Assignment {
    pub fn is_inside(&self, other: &Assignment) -> bool {
        self.start >= other.start && self.end <= other.end
    }

    pub fn overlaps(&self, other: &Assignment) -> bool {
        self.start >= other.start && self.start <= other.end || 
        self.end <= other.end && self.end >= other.start
    }

    pub fn parse_str(s: &str) -> Result<Assignment, String> {
        let (first, second) = match s.split_once('-') {
            Some((f, s)) => (f, s),
            _ => return Result::Err("Could not split text".to_string())
        };

        let (s, e) = match (first.parse::<u32>(), second.parse::<u32>()) {
            (Ok(s), Ok(e)) => (s, e),
            _ => return Result::Err("Could not parse ints in line".to_string()),
        };
        Ok(Assignment { start: s, end: e })
    }
}

fn parse_assignments(line: &str) -> Result<(Assignment, Assignment), String> {
    let (first, second) = match line.split_once(',') {
        Some((f, s)) => (f, s),
        _ => return Result::Err("Could not split line into two sections".to_string()),
    };

    let (a, b) = match (Assignment::parse_str(first), Assignment::parse_str(second)) {
        (Ok(a), Ok(b)) => (a, b),
        (Err(_), _) | (_, Err(_)) => return Err("Issues passing one of the assignments".to_string()) // Could just use default here
    };
    Ok((a, b))
}

fn main() {
    let part1 = DATA.lines()
    .filter_map(|line|parse_assignments(line).ok())
    .filter(|(a, b)| a.is_inside(&b) || b.is_inside(&a))
    .count();

    let part2 = DATA.lines()
    .filter_map(|line|parse_assignments(line).ok())
    .filter(|(a, b)| a.overlaps(&b) || b.overlaps(&a))
    .count();

    println!("Part 1: Number of partial overlaps: {part1:?}");
    println!("Part 2: Number of full overlaps: {part2:?}");
}

mod tests {
    use super::*;

    #[test]
    fn test_contains(){
        let a = Assignment{start: 1, end: 3};
        let b = Assignment{start: 2, end: 3};
        let a_in_b = a.is_inside(&b);
        let b_in_a = b.is_inside(&a);
        assert!(!a_in_b, "a should not be inside b {a_in_b:}");
        assert!(b_in_a, "b should be inside a. Got {b_in_a:}");
    }

    #[test]
    fn test_overlap(){
        let a = Assignment{start: 1, end: 3};
        let b = Assignment{start: 2, end: 4};
        let c = Assignment{start: 5, end: 6};

        assert!(a.overlaps(&b), "a should overlap be inside b");
        assert!(!b.overlaps(&c), "b should not overlap c");
    }
}

