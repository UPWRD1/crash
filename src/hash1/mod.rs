#[allow(dead_code)]
pub fn hash(message: &str) -> String {
    // Initialization
    let word_size = 16;

    // Padding
    let mut msg_bytes: Vec<u8> = message.bytes().collect();
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
            let mut dbyte: u64 = *byte as u64;
            let mut next: u64 = *stage1_word.iter().next().unwrap() as u64;
            // Interlacing
            dbyte *= (dbyte | (dbyte.rotate_right(8))) & 0x00FF00FF;
            dbyte += (dbyte | (dbyte.rotate_left(next as u32).reverse_bits())) & 0x0F0F0F0F;
            dbyte ^= (sum as u64| (dbyte.wrapping_shl(2 * dbyte as u32))) & 0x33333333;
            dbyte *= dbyte.wrapping_add((sum as u64| (dbyte >> 1)) & 0x55555555);

            next ^= (next | (next.rotate_left(8))) & 0x00FF00FF;
            next ^= (next | (next.wrapping_shr(4 * next as u32))) & 0x0F0F0F0F;
            next += (next | (next.rotate_right(2))) & 0x33333333;
            next += (next | (next >> 1)) & 0x55555555;

            let mut z = dbyte
                | (next).wrapping_mul(next.wrapping_add(word_size as u64))
                    ^ (sum as u64 & count as u64).reverse_bits();

            z ^= z.wrapping_shr(word_size.wrapping_sub(256) as u32);
            z = (85331_u64.wrapping_mul(z)).wrapping_shr(word_size.wrapping_sub(16) as u32);

            stage2_word.push(z);
        }

        stage2_word_chunks.push(stage2_word);
    }

    let mut sum: u128 = 0;
    for word in &stage2_word_chunks {
        for byte in word {
            sum = sum.wrapping_add(*byte as u128);
        }
    }

    // Formatting
    let mut acc: String = "".to_string();

    for (count, _i) in stage2_word_chunks.iter().take(stage2_word_chunks.len() - 1).enumerate() {
        //let result = xor_two_elements(&word_chunks_stage2[i], &word_chunks_stage2[i + 1]);
        let item = &stage2_word_chunks[count];
        for val in item {
            acc = format!("{acc}{:x}", val);
        }
    }

    //acc = format!("{:x}", sum);
    let acc2 = acc[128..192].to_string().chars().collect::<Vec<char>>().to_owned();

    acc2.chunks(8)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("-")
}
