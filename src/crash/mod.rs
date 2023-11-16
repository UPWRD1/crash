#[allow(dead_code)]
pub struct K0 {
    pub msg: String,
    pub digest: String,
}

#[allow(dead_code)]
impl K0 {
    //eea433b148525d60cf756a0c3dee8faac7c2e6cb0578913166369ec0ce4b1cde
    #[inline]
    pub fn hash(input: &str) -> String {
        // Initialization
        const WORDSIZE: usize = 16; //16

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
        let stage1_words: Vec<&u8> = msg_bytes
            .chunks(WORDSIZE)
            .collect::<Vec<&[u8]>>()
            .into_iter()
            .flatten()
            .collect::<Vec<&u8>>();
        let sum: u32 = stage1_words.iter().map(|x| **x as u32).sum();

        // Operations
        let mut accumulator: String = "".to_string();
        let _: Vec<u64> = stage1_words
            .clone()
            .into_iter()
            .enumerate()
            .map(|(count, stage1_word)| {
                let mut curr_byte: u64 = *stage1_word as u64;
                let mut next_byte: u64 = **stage1_words.iter().next().unwrap() as u64;
                // Interlacing
                curr_byte *= (curr_byte | (curr_byte.rotate_right(8))) & 0x00FF00FF;
                curr_byte += (curr_byte | (curr_byte.rotate_left(next_byte as u32).reverse_bits()))
                    & 0x0F0F0F0F;
                curr_byte ^=
                    (sum as u64 | (curr_byte.wrapping_shl(2 * curr_byte as u32))) & 0x33333333;
                curr_byte *= curr_byte.wrapping_add((sum as u64 | (curr_byte >> 1)) & 0x55555555);

                next_byte ^= (next_byte | (next_byte.rotate_left(8))) & 0x00FF00FF;
                next_byte ^=
                    (next_byte | (next_byte.wrapping_shr(4 * next_byte as u32))) & 0x0F0F0F0F;
                next_byte += (next_byte | (next_byte.rotate_right(2))) & 0x33333333;
                next_byte += (next_byte | (next_byte >> 1)) & 0x55555555;

                let mut interlaced = curr_byte
                    | (next_byte).wrapping_mul(next_byte.wrapping_add(WORDSIZE as u64))
                        ^ (sum as u64 & count as u64).reverse_bits();

                interlaced ^= interlaced.wrapping_shr(WORDSIZE.wrapping_sub(256) as u32);
                interlaced = (85331_u64.wrapping_mul(!interlaced))
                    .wrapping_shr((WORDSIZE.wrapping_sub(16) as u32).reverse_bits());
                accumulator = format!("{accumulator}{:x}", interlaced);
                interlaced
            })
            .collect::<Vec<u64>>();

        let sum_acc: u32 = accumulator
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .map(|x| *x as u32)
            .sum::<u32>();
        let mut truncated: Vec<char>;
        let dlen: usize = 256 as usize;
        let dlen2: usize = (256 * 2) as usize;
        let dlen3: usize = (256 * 3) as usize;

        if sum_acc & 1 == 0 {
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
        truncated.rotate_right(sum_acc as usize & 127);
        truncated.truncate(64);
        truncated
            .chunks(8)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("")
    }
    
    //51593a79fffffdb30e2e82e7fffff451ec7f7a45fffffc8e1a7eec2efff67e4c
    #[inline]
    pub fn hash1(input: &str) -> String {
        // Initialization
        const WORDSIZE: usize = 16; //16
        const DLENX1: usize = 256 as usize;
        const DLENX2: usize = (DLENX1 * 2) as usize;
        const DLENX3: usize = (DLENX1 * 3) as usize;

        // Padding
        let mut msg_bytes: Vec<u8> = input.bytes().collect();
        let mut count1: usize = 0;
        if msg_bytes.len() < 512 {
            while msg_bytes.len() < 512 {
                msg_bytes.push(msg_bytes[count1].wrapping_add(17 ^ count1 as u8));
                //msg_bytes.push(0);
                count1 += 1;
            }
        }

        // Create words
        let stage1_words: Vec<&u8> = msg_bytes
            .chunks(WORDSIZE)
            .collect::<Vec<&[u8]>>()
            .into_iter()
            .flatten()
            .collect::<Vec<&u8>>();
        let sum: u32 = stage1_words.iter().map(|x| **x as u32).sum();

        // Operations
        let mut accumulator: String = "".to_string();
        let mut truncated: Vec<char>;
        if sum & 1 == 0 {
            truncated = process(&stage1_words, sum, &mut accumulator, WORDSIZE)
                [DLENX1 as usize..DLENX2]
                .to_string()
                .chars()
                .collect::<Vec<char>>()
                .to_owned();
        } else {
            truncated = process(&stage1_words, sum, &mut accumulator, WORDSIZE)[DLENX2..DLENX3]
                .to_string()
                .chars()
                .collect::<Vec<char>>()
                .to_owned();
        }
        truncated.rotate_right(sum as usize & 127);
        truncated.truncate(WORDSIZE * 4);
        truncated
            .chunks(8)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("")
    }
}

#[inline]
fn process(stage1_words: &Vec<&u8>, sum: u32, accumulator: &mut String, wordsize: usize) -> String {
    stage1_words
        .clone()
        .into_iter()
        .enumerate()
        .map(move |(count, stage1_word)| {
            let mut curr_byte: u64 = *stage1_word as u64;
            let mut next_byte: u64 = **stage1_words.iter().next().unwrap() as u64;
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
                | (next_byte).wrapping_mul(next_byte.wrapping_add(wordsize as u64))
                    ^ (sum as u64 & count as u64).reverse_bits();

            interlaced ^= interlaced.wrapping_shr(wordsize.wrapping_sub(256) as u32);
            interlaced = (85331_u64.wrapping_mul(!interlaced))
                .wrapping_shr((wordsize.wrapping_sub(16) as u32).reverse_bits());
            *accumulator = format!("{accumulator}{:x}", interlaced);
            let res = accumulator.to_owned();
            res
        })
        .collect::<String>()
}
