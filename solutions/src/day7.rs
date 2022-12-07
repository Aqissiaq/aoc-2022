enum DirEntry<'a> {
    Dir (Directory<'a>),
    File {name : &'a str, size: usize}
}

struct Directory<'a> {
    name : &'a str,
    content : Vec<DirEntry<'a>>
}

enum Line<'a> {
    Cd (&'a str),
    Ls,
    Dir (&'a str),
    File(&'a str, usize)
}

fn parse_line(line: &str) -> Option<Line<'_>> {
    match line.split_whitespace().collect::<Vec<_>>()[..] {
        ["$", "cd", dir] => Some(Line::Cd(dir)),
        ["$", "ls"] => Some(Line::Ls),
        ["dir", name] => Some(Line::Dir(name)),
        [size, name] => Some(Line::File(name, size.parse().unwrap())),
        _ => None
    }
}

fn find_dir<'a>(root: &'a Directory, name: &'a str) -> Option<&'a Directory<'a>> {
    for i in root.content.iter() {
        match i {
            DirEntry::Dir(d) => if d.name == name {
                return Some(&d);
            }
            _ => continue
        }
    }

    None
}

fn build_fs(history:&String) -> Directory {
    let mut fs = Directory {
        name: "/",
        content: vec![]
    };
    let mut cwd = vec![& mut fs];

    for line in history.lines() {
        match parse_line(line).unwrap() {
            Line::Cd("..") => _ = cwd.pop(),
            Line::Cd(dir) => {
                let dir = find_dir(cwd[0], dir).unwrap();
                cwd.push(dir);
            },
            Line::Ls => continue,
            Line::Dir(name) => {
                let mut newDir = Directory {name: name, content: vec![]};
                fs.content.push(DirEntry::Dir(newDir));
            },
            Line::File(name, size) => {
                let newFile = DirEntry::File{name: name, size: size};
            fs.content.push(newFile)}
        }
    }
    return fs;
}

pub fn solve1(input:&String) -> String {

    return "WIP".to_string();
}

pub fn solve2(input:&String) -> String {
    return "WIP".to_string();

}
