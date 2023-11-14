#[allow(dead_code)]
pub fn hash(message: &str) -> String {
    // Initialization
    let word_size = 8;

    // Padding
    let mut msg_bytes: Vec<u8> = message.bytes().collect();
    let mut count1 = 0;
    if msg_bytes.len() < 512 {
        msg_bytes.push(0xff);
        while msg_bytes.len() < 512 {
            msg_bytes.push(msg_bytes[count1].wrapping_mul(31 ^ count1 as u8));
            count1 += 1;
        }
    }
    

    // Create words
    let word_chunks_stage1: Vec<_> = msg_bytes.chunks(word_size).collect();
    let mut word_chunks_stage2: Vec<Vec<u8>> = vec![];

    let folded_result = word_chunks_stage1.clone().into_iter().flatten().collect::<Vec<&u8>>().iter().fold(0u8, |acc, &value| {
        acc ^ value
    });
    for word_stage1 in word_chunks_stage1.clone() {
        let mut word_stage2: Vec<u8> = vec![];
        for byte in word_stage1 {
            let next: &u8 = word_stage1.iter().next().unwrap();
            let mut z: u8 = 0;
            // Interlace each value, with xor for fun
            for i  in 0..std::mem::size_of_val(&byte) * word_size {
                let mut next_xor: u8 = *next;
                if (i ^ 1) == (i + 1) {
                    next_xor = next ^ &folded_result;
                }
                z |= (byte & (1_u8.wrapping_shl(i as u32))).wrapping_shl(i as u32) | (next_xor & (1_u8.wrapping_shl(i as u32))).wrapping_shl((i + 1) as u32);
            }
            word_stage2.push(z);
        }
        word_chunks_stage2.push(word_stage2);
    }

    let mut acc: String = "".to_string();
    for i in 0..word_chunks_stage2.len() - 1 {
        //let result = xor_two_elements(&word_chunks_stage2[i], &word_chunks_stage2[i + 1]);
        let result = &word_chunks_stage2[i];
        for byte in result {
            acc = format!("{acc}{:x}", byte);
        }
        
    }

    acc.truncate(word_size * word_size);
    acc
}

fn xor_two_elements(vec1: &Vec<u8>, vec2: &Vec<u8>) -> Vec<u8> {
    vec1.iter().zip(vec2.iter()).map(|(a, b)| a ^ !b).collect()
}