const DATA: &str = include_str!("input.txt");

fn is_visible(trees: &Vec<Vec<u8>>, pi: &usize, pj: &usize) -> bool {
    let len_i = trees.len();
    let len_j = trees[0].len();

    if *pi == 0 || *pj == 0 || *pi == len_i-1 || *pj == len_j-1 {
        return true;
    }

    let check_tree = trees[*pi][*pj];
    let mut visible = true;

    // search up
    let mut i = *pi;
    while i < len_i-1 && visible {
        i += 1;
        if check_tree <= trees[i][*pj] {
            visible = false;
        }
    }
    if visible {
        return true;
    }

    // search down
    visible = true;
    i = *pi;
    while i > 0 && visible {
        i -= 1;
        if check_tree <= trees[i][*pj] {
            visible = false;
        }
    }
    if visible {
        return true;
    }

    // search right
    let mut j = *pj;
    visible = true;
    while j < len_j-1 && visible  {
        j += 1;
        if check_tree <= trees[*pi][j] {
            visible = false;
        }
    }
    if visible {
        return true;
    }

    // search left
    j = *pj;
    visible = true;
    while j > 0 && visible {
        j -= 1;
        if check_tree <= trees[*pi][j] {
            visible = false;
        }
    }

    visible
}


fn scenic_score(trees: &Vec<Vec<u8>>, pi: &usize, pj: &usize) -> u32 {
    let len_i = trees.len();
    let len_j = trees[0].len();

    let check_tree = trees[*pi][*pj];
    let mut up_score: u32 = 0;

    // search up
    let mut i = *pi;
    while i < len_i-1 {
        i += 1;
        up_score += 1;
        if check_tree <= trees[i][*pj] {
            break;
        }
    }

    // search down
    let mut down_score: u32 = 0;
    i = *pi;
    while i > 0 {
        i -= 1;
        down_score += 1;
        if check_tree <= trees[i][*pj] {
            break;
        }
    }
    
    // search right
    let mut right_score: u32 = 0;
    let mut j = *pj;
    while j < len_j-1 {
        j += 1;
        right_score += 1;
        if check_tree <= trees[*pi][j] {
            break;
        }
    }
    
    // search left
    let mut left_score: u32 = 0;
    j = *pj;
    while j > 0 {
        j -= 1;
        left_score += 1;
        if check_tree <= trees[*pi][j] {
            break;
        }
    }
    up_score * down_score * left_score * right_score
}

fn main() {
    let mut trees: Vec<Vec<u8>> = Vec::new();
    for row in DATA.lines() {
        trees.push(row.chars().filter_map(|c| c.to_digit(10)).map(|i| i as u8).collect::<Vec<u8>>());
    }

    let i_len = trees.len();
    let j_len = trees[0].len();

    let mut visable_trees: Vec<(usize, usize)> = Vec::new();
    let mut scenic_trees: Vec<u32> = Vec::new();

    for i in 0..i_len {
        for j in 0..j_len {
            scenic_trees.push(scenic_score(&trees, &i, &j));
            if is_visible(&trees, &i, &j) {
                visable_trees.push((i, j));
                print!("x")
            } else {
                print!(".")
            }
        }
        println!("")
    }
    
    println!("Total visible: {}", visable_trees.len());
    println!("Best Scenic Score: {}", scenic_trees.iter().max().unwrap());
}
