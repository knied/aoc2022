use std::fs;

enum Line {
    CD(String),
    LS,
    Dir(String),
    File(String, usize),
}

impl std::str::FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let token = s.split_whitespace().collect::<Vec<_>>();
        match token[0] {
            "$" => match token[1] {
                "cd" => Ok(Line::CD(token[2].to_owned())),
                "ls" => Ok(Line::LS),
                _ => Err("invalid command".to_owned()),
            },
            "dir" => Ok(Line::Dir(token[1].to_owned())),
            _ => match token[0].parse::<usize>().ok() {
                Some(size) => Ok(Line::File(token[1].to_owned(), size)),
                _ => Err("invalid input line".to_owned()),
            },
        }
    }
}

struct Dir {
    name: String,
    self_size: usize,
    subdirs: std::collections::HashMap<String, usize>,
}

impl Dir {
    pub fn new(name: &str) -> Self {
        Dir {
            name: name.to_string(),
            self_size: 0,
            subdirs: std::collections::HashMap::<String, usize>::new(),
        }
    }
    pub fn contains(&self, name: &str) -> bool {
        self.subdirs.contains_key(name)
    }
}

struct DirTree {
    dirs: Vec<Dir>,
    stack: Vec<usize>,
}

impl DirTree {
    pub fn new() -> Self {
        let mut res = DirTree {
            dirs: Vec::<Dir>::new(),
            stack: Vec::<usize>::new(),
        };
        res.dirs.push(Dir::new(""));
        res
    }
    pub fn root(&self) -> &Dir {
        &self.dirs[0]
    }
    pub fn current(&self) -> &Dir {
        if self.stack.is_empty() {
            &self.dirs[0]
        } else {
            let idx = self.stack.len() - 1;
            &self.dirs[self.stack[idx]]
        }
    }
    pub fn current_mut(&mut self) -> &mut Dir {
        if self.stack.is_empty() {
            &mut self.dirs[0]
        } else {
            let idx = self.stack.len() - 1;
            &mut self.dirs[self.stack[idx]]
        }
    }
    pub fn cd(&mut self, to: &str) {
        match to {
            "/" => {
                self.stack.clear();
            }
            ".." => {
                self.stack.pop();
            }
            _ => {
                if !self.current().contains(to) {
                    let idx = self.dirs.len();
                    self.dirs.push(Dir::new(to));
                    self.current_mut().subdirs.insert(to.to_string(), idx);
                }
                let idx = *self
                    .current()
                    .subdirs
                    .get(to)
                    .expect("subdir should exist");
                self.stack.push(idx);
            }
        };
        /*print!("/");
        for idx in &self.stack {
            print!("{}/", self.dirs[*idx].name);
        }
        println!();*/
    }
    pub fn total_sizes(
        &self,
        dir: &Dir,
        prefix: String,
        dirs: &mut Vec::<(String, usize)>,
    ) -> usize {
        let mut size = dir.self_size;
        dir.subdirs.iter().for_each(|(_, idx)| {
            let sdir = &self.dirs[*idx];
            size += self.total_sizes(
                sdir,
                format!("{}/{}", prefix, sdir.name),
                dirs,
            );
        });
        dirs.push((prefix,size));
        size
    }
}

fn main() {
    let content = fs::read_to_string("input/day7a.txt")
        .expect("Unable to read input file.");

    let lines = content
        .lines()
        .map(|line| line.parse::<Line>().expect("should be a valid line"))
        .collect::<Vec<_>>();

    let mut tree = DirTree::new();
    for line in lines {
        match line {
            Line::CD(to) => {
                tree.cd(to.as_str());
            }
            Line::File(_, size) => {
                tree.current_mut().self_size += size;
            }
            _ => (),
        }
    }
    let mut dirs = Vec::<(String, usize)>::new();
    let used = tree.total_sizes(tree.root(), "".to_string(), &mut dirs);
    const TOTAL : usize = 70000000;
    const NEEDED : usize = 30000000;
    let free = TOTAL - used;
    let remove = NEEDED - free;
    println!("used: {}", used);
    println!("remove: {}", remove);
    let mut candidates = dirs.iter()
        .filter(|(_, size)| {
            size >= &remove
        })
        .collect::<Vec<_>>();
    candidates.sort_by(|(_, a), (_, b)| {
        a.cmp(b)
    });
    let (name, size) = candidates.first().expect("there should be one");
    println!("{} : {}", name, size);
}
