use anyhow::Result;

struct Dir {
    size: usize,
    entries: Vec<Dir>,
}

impl Dir {
    fn new<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Dir {
        let mut dir = Dir {
            size: 0,
            entries: vec![],
        };

        while let Some(line) = lines.next() {
            if ["$ cd /", "dir", "$ ls"]
                .iter()
                .any(|s| line.starts_with(s))
            {
                continue;
            } else if line == "$ cd .." {
                break;
            }

            if let Ok(size) = line.split_once(' ').unwrap().0.parse::<usize>() {
                dir.size += size;
            } else {
                dir.entries.push(Self::new(lines));
                dir.size += dir.entries.last().unwrap().size;
            }
        }
        dir
    }

    fn recurse(&self) -> Box<dyn Iterator<Item = &Self> + '_> {
        Box::new(std::iter::once(self).chain(self.entries.iter().flat_map(Self::recurse)))
    }
}

fn main() -> Result<()> {
    let root = Dir::new(&mut include_str!("./day7.txt").lines());
    let sizes: Vec<usize> = root.recurse().map(|dir| dir.size).collect();
    let required = 30000000 - (70000000 - root.size);

    println!(
        "part1 is :{:?}",
        sizes.iter().filter(|size| **size <= 100000).sum::<usize>()
    );
    println!(
        "part2 is :{:?}",
        sizes
            .iter()
            .filter(|size| **size >= required)
            .min()
            .unwrap()
    );
    Ok(())
}
