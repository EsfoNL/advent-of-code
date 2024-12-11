use std::io::Read;

// fn expand(input: &str) -> impl Iterator<>

pub fn run() {
    let mut map = std::fs::File::open_buffered("inputs/day9")
        .unwrap()
        .bytes()
        .map(Result::unwrap)
        .filter(u8::is_ascii_digit)
        .map(|e| e - b'0')
        .collect::<Vec<_>>();
    // let mut index_1 = 0;
    // let mut index_2 = map.len() - 1;
    // let mut pos = 0;
    // let mut sum = 0;
    let mut file_index = 0;
    let mut rev_file_index = map.len() / 2;
    let mut left_index = 0;
    let mut right_index = map.len()
        - 1
        - if map.len() & 1 > 0 {
            // even, file ends at empty
            0
        } else {
            // unenven, file ends at file
            1
        };
    let mut empty = false;
    let mut sum = 0;
    let mut index = 0;
    // println!("{rev_file_index} {file_index}");
    while left_index <= right_index {
        let file_id = if empty {
            if map[left_index] < 1 {
                empty = false;
                left_index += 1;
                continue;
            }

            if map[right_index] < 1 {
                right_index -= 2;
                rev_file_index -= 1;
                continue;
            }
            map[left_index] -= 1;
            map[right_index] -= 1;
            rev_file_index
        } else {
            if map[left_index] < 1 {
                empty = true;
                left_index += 1;
                file_index += 1;
                continue;
            }
            map[left_index] -= 1;
            file_index
        };

        sum += index * file_id;
        // println!("{index} * {file_id} = {}", index * file_id);
        index += 1;
    }

    println!("checksum: {sum}");

    let map = std::fs::File::open_buffered("inputs/day9")
        .unwrap()
        .bytes()
        .map(Result::unwrap)
        .filter(u8::is_ascii_digit)
        .map(|e| e - b'0')
        .collect::<Vec<_>>();
    // fileid => (location, size)
    let mut files = Vec::new();
    // some => (location, size)
    // disk-order preserved
    let mut empties = Vec::new();
    empty = true;
    let mut index: usize = 0;

    for i in map.iter() {
        empty = !empty;
        if *i == 0 {
            continue;
        }
        // println!("empty: {empty}");
        if empty {
            // (where, len)
            empties.push((index, *i as usize));
        } else {
            // (where, len)
            files.push((index, *i as usize, files.len()));
        }
        index += *i as usize;
    }

    // println!("files: {files:?}");

    for file in files.iter_mut().rev() {
        for empty in empties.iter_mut() {
            if empty.1 < file.1 {
                continue;
            }

            if empty.0 > file.0 {
                break;
            }

            // println!("{} -> {}", file.0, empty.0);
            file.0 = empty.0;
            // empties may become zero sized
            empty.0 += file.1;
            empty.1 -= file.1;
            // println!("empty-after: {empty:?}, file-after: {file:?}");

            break;
        }
    }

    println!("computed!");

    // files.sort_unstable_by_key(|e| e.0);
    // println!("empties: {files:?}");
    // (id, len)  id = 0 if empty
    let checksum: usize = files
        .iter()
        .map(|(index, len, id)| (*index..(*index + len)).map(|e| e * id).sum::<usize>())
        .sum();
    println!("checksum: {checksum}");
}
