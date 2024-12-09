const EMPTY: i32 = -1;

fn compute_empty_len(blocks: &Vec<i32>, index: usize) -> usize {
    let mut count = 0;
    while index + count < blocks.len() && blocks[index + count] == EMPTY {
        count += 1;
    }
    count
}

fn compute_nb_len(blocks: &Vec<i32>, index: usize) -> usize {
    let mut count = 0;
    while count <= index && blocks[index - count] == blocks[index] {
        count += 1;
    }
    count
}

fn get_next_nb_index(blocks: &Vec<i32>, mut index: usize) -> usize {
    while blocks[index] == EMPTY {
        index -= 1;
    }
    index
}

pub fn get_compact_p2(mut blocks: Vec<i32>) -> Vec<i32> {
    let mut j: usize = blocks.len() - 1;
    loop {
        let current_nb_len = compute_nb_len(&blocks, j);
        let mut i: usize = 0;
        while i < j {
            if blocks[i] == EMPTY {
                let empty_len = compute_empty_len(&blocks, i);
                if empty_len < current_nb_len {
                    i += empty_len;
                    continue;
                }
                let mut count = 0;
                while count < current_nb_len {
                    blocks[i + count] = blocks[j - count];
                    blocks[j - count] = -1;
                    count += 1;
                }
                break;
            }
            i += 1;
        }
        if j > current_nb_len {
            j = get_next_nb_index(&blocks, j - current_nb_len);
        } else {
            break;
        }
    }
    blocks
}

//pub fn get_compact_p2(mut blocks: Vec<i32>) -> Vec<i32> {
//let mut i: usize = 0;
//let mut j: usize = blocks.len() - 1;

//while i < blocks.len() {
//println!("{:?}", blocks);
//if blocks[i] == EMPTY {
//let empty_len = compute_empty_len(&blocks, i);
//let mut current_nb_len = compute_nb_len(&blocks, j);
//println!("current nb_len = {}", current_nb_len);
//println!("empy _len = {}", empty_len);
//println!(".. j corresponds to {}", j);
//while j > 0 && compute_nb_len(&blocks, j) > empty_len {
//j = get_next_nb_index(&blocks, j - current_nb_len);
//println!("searching for next nb.. j corresponds to {}", j);
//current_nb_len = compute_nb_len(&blocks, j);
//println!("current nb_len = {}", current_nb_len);
//}
//if j == 0 {
//return blocks;
//}
//let mut count = 0;
//while count < current_nb_len {
//blocks[i + count] = blocks[j - count];
//blocks[j - count] = -1;
//count += 1;
//println!("just been swapped");
//}
//i += empty_len;
//println!("i corresponds to {}", blocks[i]);
//j = get_next_nb_index(&blocks, j - current_nb_len);
//println!("j corresponds to {}", blocks[j]);
//}
//i += 1;
//}
//blocks
//}
