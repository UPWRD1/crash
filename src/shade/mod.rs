#[allow(dead_code)]
pub struct  Shade {
    pub msg: String,
    pub digest: String,
}

#[allow(dead_code)]
impl Shade {
    pub fn hash(input: &str, dlen: u32) -> String {
        // Initialization
    let word_size = 16;

    // Padding
    let mut msg_bytes: Vec<u8> = input.bytes().collect();
    let mut count1: usize = 0;
    if msg_bytes.len() < 512 {
        msg_bytes.push(0xff);
        while msg_bytes.len() < 512 {
            msg_bytes.push(msg_bytes[count1].wrapping_add(17 ^ count1 as u8));
            //msg_bytes.push(0);
            count1 += 1;
        }
    }

    // Create words
    let stage1_word_chunks: Vec<_> = msg_bytes.chunks(word_size).collect();
    let mut stage2_word_chunks: Vec<Vec<u64>> = vec![];

    // Operations
    for stage1_word in stage1_word_chunks.clone() {
        let mut stage2_word: Vec<u64> = vec![];
        let sum: u32 = stage1_word.iter().map(|x| *x as u32).sum();
        for (count, byte) in stage1_word.iter().enumerate() {
            let mut curr_byte: u64 = *byte as u64;
            let mut next_byte: u64 = *stage1_word.iter().next().unwrap() as u64;
            // Interlacing
            curr_byte *= (curr_byte | (curr_byte.rotate_right(8))) & 0x00FF00FF;
            curr_byte +=
                (curr_byte | (curr_byte.rotate_left(next_byte as u32).reverse_bits())) & 0x0F0F0F0F;
            curr_byte ^= (sum as u64 | (curr_byte.wrapping_shl(2 * curr_byte as u32))) & 0x33333333;
            curr_byte *= curr_byte.wrapping_add((sum as u64 | (curr_byte >> 1)) & 0x55555555);

            next_byte ^= (next_byte | (next_byte.rotate_left(8))) & 0x00FF00FF;
            next_byte ^= (next_byte | (next_byte.wrapping_shr(4 * next_byte as u32))) & 0x0F0F0F0F;
            next_byte += (next_byte | (next_byte.rotate_right(2))) & 0x33333333;
            next_byte += (next_byte | (next_byte >> 1)) & 0x55555555;

            let mut interlaced = curr_byte
                | (next_byte).wrapping_mul(next_byte.wrapping_add(word_size as u64))
                    ^ (sum as u64 & count as u64).reverse_bits();

            interlaced ^= interlaced.wrapping_shr(word_size.wrapping_sub(256) as u32);
            interlaced = (85331_u64.wrapping_mul(interlaced))
                .wrapping_shr(word_size.wrapping_sub(16) as u32);
            stage2_word.push(interlaced);
        }
        stage2_word_chunks.push(stage2_word);
    }

    let flat_stage2_chunks: Vec<&u64> = stage2_word_chunks.iter().flatten().collect();
    // Truncate and formatting
    let mut accumulator: String = "".to_string();
    for item in flat_stage2_chunks {
        accumulator = format!("{accumulator}{:x}", item);
    }

    let sum: u32 = accumulator
        .chars()
        .collect::<Vec<char>>()
        .iter()
        .map(|x| *x as u32)
        .sum::<u32>();
    let mut truncated: Vec<char>;
    let dlen2: usize = (dlen * 2) as usize;
    let dlen3: usize = (dlen * 3) as usize;

    if sum % 3 == 0 {
        truncated = accumulator[0..dlen as usize]
            .to_string()
            .chars()
            .collect::<Vec<char>>()
            .to_owned();
    } else if sum & 1 == 0 {
        truncated = accumulator[dlen as usize..dlen2]
            .to_string()
            .chars()
            .collect::<Vec<char>>()
            .to_owned();
    } else {
        truncated = accumulator[dlen2..dlen3]
            .to_string()
            .chars()
            .collect::<Vec<char>>()
            .to_owned();
    }
    truncated.rotate_right(sum as usize & 127);
    truncated.truncate(64);
    truncated
        .chunks(8)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("")
    }

    pub fn salt_hash(input: &str, dlen: u32, salt: &str) -> String {
        let to_hash = format!("{salt}{input}");
        Shade::hash(&to_hash, dlen)
    }

}

