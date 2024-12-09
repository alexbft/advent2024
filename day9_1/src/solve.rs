use std::collections::VecDeque;

struct Block {
    id: Option<usize>,
    length: usize,
}

pub fn solve(input: &str) -> usize {
    let mut blocks = VecDeque::new();
    let mut is_space = true;
    let mut next_block_id = 0usize;
    for c in input.trim().chars() {
        is_space = !is_space;
        let id = if is_space {
            None
        } else {
            let cur_block_id = next_block_id;
            next_block_id += 1;
            Some(cur_block_id)
        };
        blocks.push_back(Block {
            id,
            length: c.to_digit(10u32).unwrap() as usize,
        });
    }
    let mut checksum = 0usize;
    let mut step = 0usize;
    while blocks.len() > 0 {
        let first_block = blocks.front_mut().unwrap();
        if first_block.length == 0 {
            blocks.pop_front();
            continue;
        }
        first_block.length -= 1;
        if let Some(id) = first_block.id {
            checksum += step * id;
            step += 1;
            continue;
        }
        let last_block_id = take_last_block_id(&mut blocks);
        checksum += step * last_block_id;
        step += 1;
    }
    checksum
}

fn take_last_block_id(blocks: &mut VecDeque<Block>) -> usize {
    while blocks.len() > 0 {
        let last_block = blocks.back_mut().unwrap();
        if last_block.length == 0 {
            blocks.pop_back();
            continue;
        }
        last_block.length -= 1;
        if let Some(id) = last_block.id {
            return id;
        }
    }
    0usize
}

#[cfg(test)]
mod tests {
    use crate::solve::solve;

    #[test]
    fn test_solve() {
        let input = "2333133121414131402";
        assert_eq!(solve(input), 1928usize);
    }
}
