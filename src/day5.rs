use std::{fs::File, io::BufRead, iter::Sum, ops::Add};

#[derive(Debug)]
struct Vec2<T>(T, T);

impl<T> Add for Vec2<T>
where
    T: Add,
{
    type Output = Vec2<T::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2::<T::Output>(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T> Sum for Vec2<T>
where
    T: Add<T, Output = T>,

    T: Default,
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vec2(T::default(), T::default()), |acc, e| acc + e)
    }
}

pub fn run() {
    let mut f = File::open_buffered("inputs/day5").unwrap().lines();

    let mut rules: Vec<(u32, u32)> = Vec::new();
    loop {
        let rule = f.next().unwrap().unwrap();
        if rule.is_empty() {
            break;
        }
        let (procede, succeed) = rule.split_once('|').unwrap();
        rules.push((procede.parse().unwrap(), succeed.parse().unwrap()));
    }
    let sum: Vec2<u32> = f
        .map(|e| {
            let mut pages: Vec<u32> = e.unwrap().split(',').map(|e| e.parse().unwrap()).collect();
            let mut failed = false;
            loop {
                let mut changed = false;
                for (procede, succeed) in rules.iter() {
                    let mut last = None;
                    for (i, e) in pages.iter().enumerate() {
                        if e == succeed {
                            last = Some(i);
                        }
                        if let (true, Some(last_i)) = (e == procede, last) {
                            pages.swap(i, last_i);
                            failed = true;
                            changed = true;
                            break;
                        }
                    }
                }
                if !changed {
                    break;
                }
            }

            if !failed {
                Vec2(pages[pages.len() / 2], 0)
            } else {
                Vec2(0, pages[pages.len() / 2])
            }
        })
        .sum();

    println!("sum: {sum:#?}");
}
