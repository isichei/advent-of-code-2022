use std::collections::HashMap;

const DATA: &str = include_str!("input.txt");

struct File {
    name: String,
    size: u32,
    dir: bool,
}

#[derive(Debug)]
struct DataPos {
    start: usize,
    end: usize,
}

struct DataStore {
    data: Vec<File>,
    dir_stack: Vec<String>,
    dir_lookup: HashMap<String, DataPos>,
}

impl DataStore {
    fn new() -> DataStore {
        DataStore { data: Vec::<File>::new(), dir_stack: Vec::<String>::new(), dir_lookup: HashMap::<String, DataPos>::new() }
    }

    fn get_last_value_off_stack(&self) -> Option<String> {
        match self.dir_stack.last() {
            None => return None,
            Some(x) => return Some(x.to_string()),
        }
    }


    fn enter_dir(&mut self, dir_name: &str) {
        // Need to note end of current dir
        // let current_dir_name = self.dir_stack.last().unwrap();
        let current_dir_name = self.get_last_value_off_stack();
        if current_dir_name.is_some() {
            self.update_lookup(&current_dir_name.unwrap());
        }
        

        // Need to add new dir to top of stack
        self.dir_stack.push(dir_name.to_string());
        self.dir_lookup.insert(dir_name.to_string(), DataPos { start: self.data.len(), end: 0_usize });
    }

    fn exit_dir(&mut self) {
        let current_dir_name = self.dir_stack.pop().unwrap();
        self.update_lookup(&current_dir_name);
    }

    fn add_file(&mut self, file: File){
        self.data.push(file);
    }

    fn update_lookup(&mut self, dir_name: &String){
        if let Some(x) = self.dir_lookup.get_mut(dir_name) {
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
        let file_sizes: Vec<u32> = self.data.iter().map(|f| f.size).collect();

        for f in self.data.iter_mut().rev() {
            if f.dir {
                let p = self.dir_lookup.get(&f.name).unwrap();
                let total_size: u32 = file_sizes[p.start..p.end].iter().sum();
                f.size = total_size;
            }
        }
    }

    fn disk_size(&self) -> u32 {
        let pos = self.dir_lookup.get(&"/".to_string()).unwrap();
        self.data[pos.start..pos.end].iter().map(|f| f.size).sum()
    }

}


fn main() {
    let mut data_store = DataStore::new();

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
            data_store.add_file(File{name: file_name.to_string(), size: 0_u32, dir: true});
        }
        else {
            // Assume file in fmt <size> <filename>
            let (file_size, file_name) = line.split_once(' ').unwrap();
            data_store.add_file(File{name: file_name.to_string(), size: file_size.parse::<u32>().unwrap(), dir: false});
        }
    }    
    data_store.complete();
    data_store.calculate_all_sizes();

    // Calculate lengths
    for (i, v) in data_store.data.iter().enumerate() {
        let t = if v.dir { "dir" } else {"file"};
        println!("{}\t{}: {}, size: {}", i, t, v.name, v.size);
    }
    
    // Part 1 answer
    println!("Total disk used: {}", data_store.disk_size());

}
