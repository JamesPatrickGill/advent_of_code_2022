use std::collections::HashMap;

#[derive(Debug, Clone)]
enum FileRef {
    File(usize),
    Dir(String),
}

fn recursive_sum(files: Vec<FileRef>, original: HashMap<String, Vec<FileRef>>) -> usize {
    let mut total = 0;
    for file in files.into_iter() {
        match file {
            FileRef::File(size) => total = total + size,
            FileRef::Dir(name) => {
                let next_files = original.clone().get(&name[..]).unwrap().to_owned();
                total = total + recursive_sum(next_files, original.clone());
            }
        }
    }
    total
}

fn main() {
    let input = include_str!("../input.txt");
    let lines: Vec<&str> = input.split_terminator("\n").collect();

    let mut dir_structure: HashMap<String, Vec<FileRef>> = HashMap::new();
    let mut dir_pointer = vec![];

    for (_idx, line) in lines.clone().into_iter().enumerate() {
        if line.get(0..1).unwrap() == "$" {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let cmd = parts.get(1).unwrap().to_owned();

            if cmd == "cd" {
                let arg = parts.get(2).unwrap().to_owned();
                if arg == ".." {
                    dir_pointer.pop();
                } else {
                    dir_pointer.push(arg);
                    if dir_structure.get(arg).is_none() {
                        dir_structure.insert(dir_pointer.join("@"), vec![]);
                    } else {
                    }
                }
            } else if cmd == "ls" {
            }
        } else {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let part_1 = parts.get(0).unwrap().to_owned();

            if part_1 == "dir" {
                let dir_name = parts.get(1).unwrap().to_owned();
                let mut pointer_clone = dir_pointer.clone();
                pointer_clone.push(dir_name);

                let dir_parent_vec = &mut dir_structure.get_mut(&dir_pointer.join("@")).unwrap();
                dir_parent_vec.push(FileRef::Dir(pointer_clone.join("@")));
            } else {
                let size = part_1.parse::<usize>().unwrap();
                let dir_parent_vec = &mut dir_structure.get_mut(&dir_pointer.join("@")).unwrap();

                dir_parent_vec.push(FileRef::File(size));
            }
        }
    }

    let mut dir_sizes: HashMap<String, usize> = HashMap::new();
    for (key, val) in dir_structure.clone().into_iter() {
        dir_sizes.insert(key, recursive_sum(val, dir_structure.clone()));
    }

    let mut sum = 0;
    for (_dir, size) in dir_sizes.into_iter() {
        if size <= 100000 {
            sum = sum + size
        }
    }

    println!("{sum:?}")
}
