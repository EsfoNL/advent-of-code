use std::{
    fs::File,
    io::BufRead,
    ops::{Add, BitAnd, BitOr, BitOrAssign, Index, IndexMut, Mul, Sub},
    thread::{self, sleep},
    time::Duration,
};

#[derive(Clone, Debug)]
enum Dir {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Debug)]
struct DirBitMap(u8);

impl BitAnd<&Dir> for &DirBitMap {
    type Output = DirBitMap;

    fn bitand(self, rhs: &Dir) -> Self::Output {
        DirBitMap(self.0 & DirBitMap::from(rhs).0)
    }
}

impl BitOrAssign<&Dir> for DirBitMap {
    fn bitor_assign(&mut self, rhs: &Dir) {
        *self = &*self | rhs
    }
}
impl BitOr<&Dir> for &DirBitMap {
    type Output = DirBitMap;

    fn bitor(self, rhs: &Dir) -> Self::Output {
        DirBitMap(self.0 | DirBitMap::from(rhs).0)
    }
}

impl From<&Dir> for DirBitMap {
    fn from(value: &Dir) -> Self {
        Self(match value {
            North => 0b1000,
            East => 0b0100,
            South => 0b0010,
            West => 0b0001,
        })
    }
}

use Dir::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Coord<T = usize>(pub T, pub T);

impl<T> Index<&Coord<usize>> for Vec<Vec<T>> {
    type Output = T;

    fn index(&self, index: &Coord<usize>) -> &Self::Output {
        &self[index.0][index.1]
    }
}

impl<T> IndexMut<&Coord<usize>> for Vec<Vec<T>> {
    fn index_mut(&mut self, index: &Coord<usize>) -> &mut Self::Output {
        &mut self[index.0][index.1]
    }
}

impl Add<(isize, isize)> for &Coord<usize> {
    type Output = (isize, isize);

    fn add(self, rhs: (isize, isize)) -> Self::Output {
        (self.0 as isize + rhs.0, self.1 as isize + rhs.1)
    }
}

impl<T> Add for &Coord<T>
where
    T: Add + Copy,
{
    type Output = Coord<T::Output>;

    fn add(self, rhs: &Coord<T>) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T> Sub for &Coord<T>
where
    T: Sub + Copy,
{
    type Output = Coord<T::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T, W> Mul<T> for &Coord<W>
where
    W: Mul<T> + Copy,
    T: Copy,
{
    type Output = Coord<W::Output>;

    fn mul(self, rhs: T) -> Self::Output {
        Coord(self.0 * rhs, self.1 * rhs)
    }
}

impl TryFrom<(isize, isize)> for Coord<usize> {
    type Error = <usize as TryFrom<isize>>::Error;

    fn try_from(value: (isize, isize)) -> Result<Self, Self::Error> {
        Ok(Coord(value.0.try_into()?, value.1.try_into()?))
    }
}

impl TryFrom<Coord<isize>> for Coord<usize> {
    type Error = <usize as TryFrom<isize>>::Error;

    fn try_from(value: Coord<isize>) -> Result<Self, Self::Error> {
        Ok(Coord(value.0.try_into()?, value.1.try_into()?))
    }
}

fn print_map_context(
    context: usize,
    guard_pos: &Coord,
    guard_dir: &Dir,
    map: &[Vec<bool>],
    bitmaps: &[Vec<DirBitMap>],
) {
    for x in (guard_pos.0.saturating_sub(context))..=(guard_pos.0 + context).clamp(0, map.len() - 1)
    {
        for y in (guard_pos.1.saturating_sub(context))
            ..=(guard_pos.1 + context).clamp(0, map[0].len() - 1)
        {
            if guard_pos.0 == x && guard_pos.1 == y {
                print!(
                    "{}",
                    ansi_term::Color::Red.paint(match guard_dir {
                        North => "^",
                        East => ">",
                        South => "<",
                        West => "v",
                    })
                );
                continue;
            }
            if bitmaps[x][y].0 != 0 {
                print!(
                    "{}",
                    ansi_term::Color::Green.paint(match bitmaps[x][y].0 {
                        0b1000 => "^",
                        0b0100 => ">",
                        0b0010 => "v",
                        0b0001 => "<",
                        _ => "+",
                    })
                );
                continue;
            }
            print!("{}", if map[x][y] { '#' } else { '.' });
        }
        println!()
    }
    std::thread::sleep(Duration::from_millis(200));
    println!();
}

pub fn run() {
    let (map, guard_start_dir, guard_start_pos) = {
        let mut guard_pos = Coord(0, 0);
        let mut guard_dir = North;
        let map = File::open_buffered("inputs/day6")
            .unwrap()
            .lines()
            .enumerate()
            .map(|(i, e)| (i, e.unwrap()))
            .map(|(x, e)| {
                e.chars()
                    .enumerate()
                    .map(|(y, e)| match e {
                        '#' => true,
                        '.' => false,
                        dir @ ('^' | '>' | 'v' | '<') => {
                            guard_pos = Coord(x, y);
                            guard_dir = match dir {
                                '>' => East,
                                '<' => West,
                                '^' => North,
                                'v' => South,
                                _ => panic!(),
                            };
                            false
                        }
                        _ => panic!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        (map, guard_dir, guard_pos)
    };

    let mut guard_pos = guard_start_pos.clone();
    let mut guard_dir = guard_start_dir.clone();
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    visited[&guard_pos] = true;

    loop {
        let try_guard_pos = &guard_pos
            + match guard_dir {
                North => (-1, 0),
                South => (1, 0),
                West => (0, -1),
                East => (0, 1),
            };
        let Ok(new_guard_pos) = Coord::try_from(try_guard_pos) else {
            break;
        };
        if new_guard_pos.0 >= map.len() || new_guard_pos.1 >= map[0].len() {
            break;
        }
        if map[&new_guard_pos] {
            guard_dir = match guard_dir {
                North => East,
                South => West,
                West => North,
                East => South,
            };
            continue;
        }
        guard_pos = new_guard_pos;
        visited[&guard_pos] = true;
    }

    println!(
        "visited: {}",
        visited
            .iter()
            .map(|e| e.iter().map(|e| *e as usize).sum::<usize>())
            .sum::<usize>()
    );
    let mut count = 0;
    for i in visited.iter().enumerate().flat_map(|(x, e)| {
        e.iter()
            .enumerate()
            .filter_map(move |(y, e)| if *e { Some(Coord(x, y)) } else { None })
    }) {
        let mut new_map = map.clone();
        new_map[&i] = true;
        guard_dir = guard_start_dir.clone();
        guard_pos = guard_start_pos.clone();
        let mut visited_dir = vec![vec![DirBitMap(0); map[0].len()]; map.len()];
        loop {
            // sleep(Duration::from_millis(200));
            let try_guard_pos = &guard_pos
                + match guard_dir {
                    North => (-1, 0),
                    South => (1, 0),
                    West => (0, -1),
                    East => (0, 1),
                };
            let Ok(new_guard_pos) = Coord::try_from(try_guard_pos) else {
                break;
            };
            if new_guard_pos.0 >= new_map.len() || new_guard_pos.1 >= new_map[0].len() {
                break;
            }
            if new_map[&new_guard_pos] {
                guard_dir = match guard_dir {
                    North => East,
                    South => West,
                    West => North,
                    East => South,
                };
                continue;
            }
            guard_pos = new_guard_pos;
            if (&visited_dir[&guard_pos] & &guard_dir).0 != 0 {
                count += 1;
                // print_map_context(10000, &guard_pos, &guard_dir, &new_map, &visited_dir);
                break;
            }
            visited_dir[&guard_pos] |= &guard_dir;
        }
    }

    println!("count: {count}");
}
