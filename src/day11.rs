use core::borrow;
use std::{collections::HashMap, fs::File};

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<usize>,
    past: Option<usize>,
}

#[derive(Debug)]
struct FrontBack {
    front: usize,
    back: usize,
}

#[derive(Debug)]
pub struct VecLinkedList<T> {
    list: Vec<Option<Node<T>>>,
    frontback: Option<FrontBack>,
    empties: Vec<usize>,
}

#[derive(Debug)]
pub struct NodeRefMut<'a, T> {
    index: usize,
    list: &'a mut VecLinkedList<T>,
}

impl<T> std::ops::Deref for NodeRefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.list.list[self.index].as_ref().unwrap().val
    }
}
impl<T> std::ops::DerefMut for NodeRefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.list.list[self.index].as_mut().unwrap().val
    }
}

impl<T> NodeRefMut<'_, T> {
    pub fn next(self) -> Result<Self, Self> {
        if let Some(v) = self.list.list[self.index].as_ref().unwrap().next {
            Ok(Self {
                index: v,
                list: self.list,
            })
        } else {
            Err(self)
        }
    }
    pub fn past(self) -> Result<Self, Self> {
        if let Some(v) = self.list.list[self.index].as_ref().unwrap().past {
            Ok(Self {
                index: v,
                list: self.list,
            })
        } else {
            Err(self)
        }
    }

    pub fn remove(self) -> T {
        let Node { past, next, val } = self.list.list[self.index].take().unwrap();
        self.list.empties.push(self.index);
        if let Some(past) = past {
            self.list.list[past].as_mut().unwrap().next = next
        };
        if let Some(next) = next {
            self.list.list[next].as_mut().unwrap().past = past
        };

        if self.index == self.list.frontback.as_ref().unwrap().front {
            if let Some(past) = past {
                self.list.frontback.as_mut().unwrap().front = past;
            } else {
                // empty list
                self.list.frontback = None;
            }
        } else if self.index == self.list.frontback.as_ref().unwrap().front {
            if let Some(next) = next {
                self.list.frontback.as_mut().unwrap().back = next;
            } else {
                // ''
                self.list.frontback = None;
            }
        }
        val
    }

    pub fn append(&mut self, val: T) {
        let pos = self.list.empties.pop().unwrap_or_else(|| {
            self.list.list.push(None);
            self.list.list.len() - 1
        });

        let next = self.list.list[self.index].as_ref().unwrap().next;

        self.list.list[self.index].as_mut().unwrap().next = Some(pos);
        if let Some(next) = next {
            self.list.list[next].as_mut().unwrap().past = Some(pos);
        } else {
            self.list.frontback.as_mut().unwrap().back = pos;
        }

        self.list.list[pos] = Some(Node {
            val,
            next,
            past: Some(self.index),
        });
    }
    pub fn prepend(&mut self, val: T) {
        let pos = self.list.empties.pop().unwrap_or_else(|| {
            self.list.list.push(None);
            self.list.list.len() - 1
        });

        let past = self.list.list[self.index].as_ref().unwrap().past;

        self.list.list[self.index].as_mut().unwrap().past = Some(pos);
        if let Some(past) = past {
            self.list.list[past].as_mut().unwrap().next = Some(pos);
        } else {
            self.list.frontback.as_mut().unwrap().front = pos;
        }

        self.list.list[pos] = Some(Node {
            val,
            next: Some(self.index),
            past,
        });
    }
}

impl<T> VecLinkedList<T> {
    pub fn front_mut(&mut self) -> Option<NodeRefMut<'_, T>> {
        if self.frontback.is_some() {
            Some(NodeRefMut {
                index: self.frontback.as_ref().unwrap().front,
                list: self,
            })
        } else {
            None
        }
    }
    pub fn back_mut(&mut self) -> Option<NodeRefMut<'_, T>> {
        if self.frontback.is_some() {
            Some(NodeRefMut {
                index: self.frontback.as_ref().unwrap().back,
                list: self,
            })
        } else {
            None
        }
    }

    pub fn push_back(&mut self, val: T) {
        let past = self.frontback.as_ref().map(|e| e.back);

        let pos = if let Some(pos) = self.empties.pop() {
            pos
        } else {
            self.list.push(None);
            self.list.len() - 1
        };
        self.list[pos] = Some(Node {
            val,
            next: None,
            past,
        });

        if let Some(frontback) = self.frontback.as_mut() {
            self.list[frontback.back].as_mut().unwrap().next = Some(pos);
            frontback.back = pos;
        } else {
            self.frontback = Some(FrontBack {
                front: pos,
                back: pos,
            });
        }
    }

    pub fn push_front(&mut self, val: T) {
        let next = self.frontback.as_ref().map(|e| e.front);

        let pos = if let Some(pos) = self.empties.pop() {
            pos
        } else {
            self.list.push(None);
            self.list.len() - 1
        };
        self.list[pos] = Some(Node {
            val,
            next,
            past: None,
        });

        if self.frontback.is_none() {
            self.frontback = Some(FrontBack {
                front: pos,
                back: pos,
            });
        }
        if let Some(frontback) = self.frontback.as_mut() {
            self.list[frontback.front].as_mut().unwrap().past = Some(pos);
            frontback.front = pos;
        } else {
            self.frontback = Some(FrontBack {
                front: pos,
                back: pos,
            });
        }
    }

    pub fn new() -> Self {
        Self {
            list: Vec::new(),
            frontback: None,
            empties: Vec::new(),
        }
    }
}

impl<T> FromIterator<T> for VecLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut linked_list = Self::new();
        for i in iter {
            linked_list.push_back(i);
        }
        linked_list
    }
}

struct NodeIter<'a, T> {
    list: &'a VecLinkedList<T>,
    index: Option<usize>,
    ended: bool,
    // rev: bool,
}

impl<'a, T> Iterator for NodeIter<'a, T>
where
    T: 'static,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ended {
            return None;
        }
        self.list.frontback.as_ref()?;
        let pos = self
            .index
            .unwrap_or(self.list.frontback.as_ref().unwrap().front);
        self.index = self.list.list[pos].as_ref().unwrap().next;
        if self.index.is_none() {
            self.ended = true;
        }
        Some(&self.list.list[pos].as_ref().unwrap().val)
    }
}
impl<'a, T> IntoIterator for &'a VecLinkedList<T>
where
    T: 'static,
{
    type Item = &'a T;

    type IntoIter = NodeIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        NodeIter {
            list: self,
            index: None,
            ended: false,
        }
    }
}

fn resolve_len(
    mut val: usize,
    mut depth: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> usize {
    // println!("depth: {depth}");
    while depth > 0 {
        // println!("end");

        match val {
            0 => {
                val = 1;
                depth -= 1;
            }
            e if e.ilog10() & 1 > 0 => {
                let halften = 10usize.pow((e.ilog10() + 1) / 2);
                let top = e / halften;
                let bottom = e % halften;
                // println!("{e} => {top}, {bottom}");

                let top_res = cache
                    .get(&(top, depth - 1))
                    .cloned()
                    .unwrap_or_else(|| resolve_len(top, depth - 1, cache));
                cache.insert((top, depth - 1), top_res);

                let bottom_res = cache
                    .get(&(bottom, depth - 1))
                    .cloned()
                    .unwrap_or_else(|| resolve_len(bottom, depth - 1, cache));
                cache.insert((bottom, depth - 1), bottom_res);
                return bottom_res + top_res;
            }
            e => {
                val = e * 2024;
                depth -= 1;
            }
        }
    }
    1
}

pub fn run() {
    let mut nums: VecLinkedList<_> =
        std::io::read_to_string(File::open_buffered("inputs/day11").unwrap())
            .unwrap()
            .split_whitespace()
            .map(|e| e.parse::<usize>().unwrap())
            .collect();
    println!("{nums:?}");
    // print!("[ ");
    // for i in &nums {
    //     print!("{i}, ");
    // }
    // println!("]");

    for i in 0..25 {
        let mut index = nums.front_mut().unwrap();
        // println!("blink: {}", i + 1);
        loop {
            match *index {
                0 => *index = 1,
                e if e.ilog10() & 1 > 0 => {
                    let halften = 10usize.pow((e.ilog10() + 1) / 2);
                    let top = e / halften;
                    let bottom = e % halften;
                    // println!("{e} => {top}, {bottom}");
                    *index = top;
                    index.append(bottom);
                    index = index.next().unwrap();
                }
                e => *index = e * 2024,
            }
            if let Ok(v) = index.next() {
                index = v;
            } else {
                break;
            };
        }
        // print!("[ ");
        // for i in &nums {
        //     print!("{i}, ");
        // }
        // println!("]");
    }
    let len = nums.into_iter().count();
    let nums: VecLinkedList<_> =
        std::io::read_to_string(File::open_buffered("inputs/day11").unwrap())
            .unwrap()
            .split_whitespace()
            .map(|e| e.parse::<usize>().unwrap())
            .collect();
    let mut len2 = 0;
    let mut cache = HashMap::new();
    for i in &nums {
        len2 += resolve_len(*i, 25, &mut cache)
    }

    println!("len: {}, {}", len, len2);
    let mut sum = 0;
    for i in &nums {
        sum += resolve_len(*i, 75, &mut cache)
    }

    println!("sum: {sum}");
}
