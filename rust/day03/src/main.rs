use std::collections::{HashSet, HashMap};
use itertools::Itertools;

const DATA: &str = include_str!("input.txt");

enum Interpretation {
    Part1,
    Part2,
}

fn create_priorities() -> HashMap<char, u32> {
    let text = "ABCDEFGHIJKLMNOPQRTSUVWXYZ";
    let mut lookup = HashMap::new();

    for char in text.chars().into_iter(){
        lookup.insert(char, (char as u32) - 38u32); // Uppercase score
        lookup.insert(char.to_lowercase().next().unwrap(), (char as u32) - 64u32); // equivalent lowercase score
    }
    lookup
}


fn hash_from_string(s: &str) -> HashSet<char> {
    let mut set = HashSet::new();

    for c in s.chars() {
        set.insert(c);
    }
    set
}

fn split_data(data: &str, interpretation: Interpretation) -> Vec<HashSet<char>> {
    let mut groups = Vec::new();
    for line in data.lines(){
        match interpretation {
            Interpretation::Part1 => {
                let (a, b) = line.split_at(line.len()/2);
                groups.push(hash_from_string(a));
                groups.push(hash_from_string(b));
            },
            Interpretation::Part2 => {
                groups.push(hash_from_string(line));
            }
        }
    }
    groups
}

fn union_set(hash_a: &HashSet<char>, hash_b: &HashSet<char>) -> HashSet<char> {
    let mut d = HashSet::new();
    for c in hash_a.intersection(hash_b) {
        d.insert(c.clone());
    }
    d
}

fn part1(data: &str, plu: &HashMap<char, u32>) {
    let group_hashes = split_data(data, Interpretation::Part1);
    let mut score: u32 = 0;
    for (a, b) in group_hashes.iter().tuples(){
        let i = union_set(a, b);
        if i.len() != 1 {
            panic!("Got more than I should: {:?}", i)
        }
        score += plu.get(i.iter().next().unwrap()).unwrap();
    }
    println!("Part 1: {}", score);
}


fn part2(data: &str, plu: &HashMap<char, u32>) {
    let group_hashes = split_data(data, Interpretation::Part2);
    let mut score: u32 = 0;
    for (a, b, c) in group_hashes.iter().tuples(){
        let first_hash = union_set(a, b);
        let i = union_set(&first_hash, c);

        if i.len() != 1 {
            panic!("Got more than I should: {:?}", i)
        }
        score += plu.get(i.iter().next().unwrap()).unwrap();
    }
    println!("Part 2: {}", score);
}

fn main() {
    let plu = create_priorities();
    part1(DATA, &plu);
    part2(DATA, &plu);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priorities_lookup(){
        let plu = create_priorities();
        assert_eq!(plu.get(&'a'), Some(&1u32), "Was testing value of a");
        assert_eq!(plu.get(&'z'), Some(&26u32), "Was testing value of z");
        assert_eq!(plu.get(&'A'), Some(&27u32), "Was testing value of A");
        assert_eq!(plu.get(&'Z'), Some(&52u32), "Was testing value of Z");
    }

    #[test]
    fn test_hash_hash_from_string(){
        let h = hash_from_string("abCC");
        assert!(h.contains(&'a'));
        assert!(h.contains(&'b'));
        assert!(h.contains(&'C'));
        assert_eq!(h.len(), 3);
    }
}

