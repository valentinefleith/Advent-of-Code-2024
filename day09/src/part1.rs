pub fn line_to_blocks(line: String) -> Vec<i32> {
    let vec_line: Vec<i32> = line.as_bytes().iter().map(|&c| (c - b'0') as i32).collect();
    let mut blocks: Vec<i32> = vec![];
    for (i, nb) in vec_line.iter().enumerate() {
        if i % 2 == 0 {
            for _ in 0..*nb {
                blocks.push((i / 2) as i32);
            }
        } else {
            for _ in 0..*nb {
                blocks.push(-1);
            }
        }
    }
    blocks
}

pub fn get_compact_files(mut blocks: Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    while i < blocks.len() - 1 {
        while blocks[i] == -1 {
            blocks[i] = blocks.pop().unwrap();
        }
        i += 1;
    }
    blocks
}

pub fn get_final_result(compact_blocks: Vec<i32>) -> i64 {
    let mut result: i64 = 0;
    for (i, nb) in compact_blocks.iter().enumerate() {
        if *nb > 0 {
            result += (i as i64) * (*nb as i64);
        }
    }
    result
}
