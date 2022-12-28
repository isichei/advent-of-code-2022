const DATA: &str = include_str!("input.txt");

fn int_or_none(s: &str) -> Option<i64> {
    match s.parse::<i64>() {
        Ok(i) => Some(i),
        Err(_) => None
    }
}

fn get_sorted_elves_calories(data: &str) -> Vec<i64> {
    let lines = data.lines().map(|l| int_or_none(l));
    let mut elves = vec![0];
    for line in lines {
        match line {
            Some(x) => {
                // hmmm...
                let y = elves.pop().unwrap();
                elves.push(x+y);
            }
            None => elves.push(0)
        }
    }
    elves.sort();
    elves.reverse();
    elves
}

fn main() {
    let elves = get_sorted_elves_calories(DATA);
    println!("Part 1: {}", elves[0]);
    println!("Part 2: {}", elves[0]+elves[1]+elves[2]);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let test_data = include_str!("example.txt");
        let out_vec = get_sorted_elves_calories(test_data);
        assert_eq!(out_vec[0], 24000);
        assert_eq!(out_vec.len(), 5);
    }
}
