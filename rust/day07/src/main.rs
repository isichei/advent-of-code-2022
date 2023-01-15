use std::collections::HashMap;

const DATA: &str = include_str!("input.txt");

struct File {
    name: String,
    size: u32,
    dir_id: Option<usize>,
}

impl File {
    fn is_dir(&self) -> bool {
        self.dir_id.is_some()
    }
}

#[derive(Debug)]
struct DataPos {
    start: usize,
    end: usize,
}

struct DataStore {
    data: Vec<File>,
    dir_stack: Vec<usize>,
    dir_lookup: HashMap<usize, DataPos>,
}

impl DataStore {
    fn new() -> DataStore {
        DataStore { data: Vec::<File>::new(), dir_stack: Vec::<usize>::new(), dir_lookup: HashMap::<usize, DataPos>::new() }
    }

    fn get_last_value_off_stack(&self) -> Option<usize> {
        match self.dir_stack.last() {
            None => return None,
            Some(x) => return Some(*x),
        }
    }

    fn get_dir_id(&self, dir_name: &String) -> usize {
        let current_dir_id = self.get_last_value_off_stack();
        if current_dir_id.is_none() {
            return 0;
        }
        
        let pos = self.dir_lookup.get(&current_dir_id.unwrap()).unwrap();
        let mut dir_id = None;
        for (i, f) in self.data[pos.start..pos.end].iter().enumerate() {
            println!("{} {}", f.name, f.is_dir());
            if f.is_dir() && f.name == dir_name.to_string() {
                dir_id = Some(pos.start + i);
                break;
            }
        }
        let d = dir_id.unwrap();
        d
    }

    fn enter_dir(&mut self, dir_name: &str) {
        // Need to note end of current dir
        let current_dir_id = self.get_last_value_off_stack();
        if current_dir_id.is_some() {
            self.update_lookup(&current_dir_id.unwrap());
        }

        // Need to add new dir to top of stack
        let new_dir_id = self.get_dir_id(&dir_name.to_string());
        self.dir_stack.push(new_dir_id);
        self.dir_lookup.insert(new_dir_id, DataPos { start: self.data.len(), end: 0_usize });
    }

    fn exit_dir(&mut self) {
        let current_dir_id = self.dir_stack.pop().unwrap();
        self.update_lookup(&current_dir_id);
    }

    fn add_file(&mut self, name: &str, size: u32){
        self.data.push(File{name: name.to_string(), size: size, dir_id: None});
    }

    fn add_dir(&mut self, name: &str){
        let dir_id = self.data.len();
        self.data.push(File{name: name.to_string(), size: 0, dir_id: Some(dir_id)});
    }

    fn update_lookup(&mut self, dir_id: &usize){
        if let Some(x) = self.dir_lookup.get_mut(dir_id) {
            // Only set if it hasn't already been updated.
            // 0 is init value could use None but value will never be zero for this challenge.
            if x.end == 0_usize {
                x.end = self.data.len();
            }
        }
    }

    fn complete(&mut self){
        while self.dir_stack.len() != 0 {
            self.exit_dir();
        }
    }

    fn calculate_all_sizes(&mut self){
        let mut dir_ids: Vec<usize> = self.data.iter().filter_map(|f| f.dir_id).collect();

        while dir_ids.len() > 0 {
            let dir_id = dir_ids.pop().unwrap();
            let pos = self.dir_lookup.get(&dir_id).unwrap();
            let total_size: u32 = self.data[pos.start..pos.end].iter().map(|f| f.size).sum();
            self.data[dir_id].size = total_size;
        }
    }

    fn dbg_print(&self) {
        let mut data_str = Vec::new();
        for f in self.data.iter() {
            data_str.push(f.name.to_string());
        }

        println!("DATA");
        println!(" data: {:?}", data_str);
        println!(" lu: {:?}", self.dir_lookup);
        println!(" stack: {:?}", self.dir_stack);
        println!("");
    }
}


fn main() {
    let mut data_store = DataStore::new();
    data_store.add_dir("/");

    for line in DATA.lines() {
        if line == "$ cd .." {
            data_store.exit_dir();
        }
        else if line.starts_with("$ cd") {
            let (_, dir_name) = line.split_once("cd ").unwrap();
            data_store.enter_dir(dir_name);
        }
        else if line.starts_with("$ ls") {
            // pass
        }
        else if line.starts_with("dir") {
            let (_, file_name) = line.split_once(' ').unwrap();
            data_store.add_dir(file_name);
        }
        else {
            // Assume file in fmt <size> <filename>
            let (file_size, file_name) = line.split_once(' ').unwrap();
            data_store.add_file(file_name, file_size.parse::<u32>().unwrap());
        }
        // data_store.dbg_print();
    }
    
    data_store.complete();
    data_store.calculate_all_sizes();

    // Calculate lengths
    for (i, v) in data_store.data.iter().enumerate() {
        let t = if v.is_dir() { "dir" } else {"file"};
        println!("{}  {}: {}, size: {}", i, t, v.name, v.size);
    }
    let total_size = data_store.data[0].size.clone();
    println!("\n Total disk used: {}", total_size);
    
    
    let mut part1: u32 = 0;
    let mut part2: u32 = total_size.clone();

    let required_space = 30000000 - (70000000 - total_size);
    println!("required space: {}\n", required_space);

    for f in data_store.data {
        if f.is_dir() && f.size < 100000 {
            part1 += f.size;
        }
        if f.is_dir() && f.size >= required_space {
            if f.size < part2 {
                part2 = f.size
            }
        }
    }

    println!("part1 {}", part1);
    println!("part2 {}", part2);
}
