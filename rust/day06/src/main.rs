use std::collections::HashSet;

const DATA: &str = include_str!("input.txt");

fn start_of_msg_marker(s: &str, win_size: usize) -> Option<usize> {
    let mut set = HashSet::new();

    for (i, win) in s.as_bytes().windows(win_size).enumerate(){
        for w in win {
            set.insert(w);
        }
        if set.len() == win_size {
            return Some(i + win_size);
        }
        set.clear();
    }
    return None;
}

fn main() {
    let ans1 = start_of_msg_marker(DATA, 4);
    let ans2 = start_of_msg_marker(DATA, 14);
    println!("Part 1: {:?}", ans1);
    println!("Part 2: {:?}", ans2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_of_msg_marker(){
        let examples_part1 = vec![
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5_usize),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6_usize),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10_usize),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11_usize),
        ];

        for example in examples_part1 {
            assert_eq!(start_of_msg_marker(example.0, 4), Some(example.1));
        }
    
        assert_eq!(start_of_msg_marker("abcabc", 4), None);

        let examples_part2 = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19_usize),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23_usize),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23_usize),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29_usize),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26_usize),
        ];

        for example in examples_part2 {
            assert_eq!(start_of_msg_marker(example.0, 14), Some(example.1));
        }
    }
}
