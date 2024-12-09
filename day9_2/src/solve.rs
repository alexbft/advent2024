use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

struct FileBlock {
    id: usize,
    start_pos: usize,
    length: usize,
}

pub fn solve(input: &str) -> usize {
    let mut blocks = vec![];
    let mut empty_spaces_by_len: HashMap<usize, BinaryHeap<Reverse<usize>>> = HashMap::new();
    let mut is_space = true;
    let mut pos = 0usize;
    let mut next_block_id = 0usize;
    for c in input.trim().chars() {
        is_space = !is_space;
        let len = c.to_digit(10u32).unwrap() as usize;
        // if !is_space && len == 0 {
        //     println!("Empty file at position {}", pos);
        // }
        if is_space {
            empty_spaces_by_len
                .entry(len)
                .or_insert_with(BinaryHeap::new)
                .push(Reverse(pos));
        } else {
            blocks.push(FileBlock {
                id: next_block_id,
                start_pos: pos,
                length: len,
            });
            next_block_id += 1;
        }
        pos += len;
    }
    for block in blocks.iter_mut().rev() {
        let best_candidate = (block.length..=9)
            .filter_map(|len| {
                empty_spaces_by_len
                    .get(&len)
                    .and_then(|spaces| spaces.peek())
                    .map(|Reverse(space_pos)| (*space_pos, len))
            })
            .min_by_key(|(space_pos, _)| *space_pos);
        if let Some((space_pos, space_len)) = best_candidate {
            if block.start_pos <= space_pos {
                continue;
            }
            block.start_pos = space_pos;
            empty_spaces_by_len.get_mut(&space_len).unwrap().pop();
            if space_len > block.length {
                let new_space_len = space_len - block.length;
                let new_spaces = empty_spaces_by_len
                    .entry(new_space_len)
                    .or_insert_with(BinaryHeap::new);
                new_spaces.push(Reverse(space_pos + block.length));
            }
        }
    }
    blocks
        .iter()
        .map(|block| {
            // println!(
            //     "Block {} starts at {} and has length {}",
            //     block.id, block.start_pos, block.length
            // );
            let n = block.length;
            let arith_sum = n * (n - 1) / 2;
            block.id * (block.start_pos * n + arith_sum)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::solve::solve;

    #[test]
    fn test_solve() {
        let input = "2333133121414131402";
        assert_eq!(solve(input), 2858usize);
    }
}
