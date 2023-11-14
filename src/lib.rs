use num_bigint::BigUint;
mod hash1;

pub fn best_hash(message: &str) -> String {
    // byte conversion
    let msg_bytes: Vec<u8> = message.chars().map(|x| x as u8).collect();

    // padding
    let original_length = msg_bytes.len();
    let mut padded_message = msg_bytes.clone();
    padded_message.push(0x0);

    while padded_message.len() % 256 != (256 - 8) % 256 {
        padded_message.push(padded_message[padded_message.len().wrapping_rem(original_length)]);
    }

    let length_bits = (original_length as u64 * 8).to_be_bytes();
    padded_message.extend_from_slice(&length_bits);

    let bin_chunks: Vec<&[u8]> = padded_message.chunks(4).collect();

    let hash_values: [u32; 8] = [
        0xEBEDD428, 0x58598FED, 0x345DE15D, 0x1427889B, 0x44ECF241, 0x24683D1B, 0x3BBA9ED7,
        0x1C829AA6,
    ];
    let a = hash_values[0];

    let mut word_chunks: Vec<Vec<u8>> = vec![];
    for chunk in bin_chunks {
        let mut curr_word: Vec<u8> = chunk.to_vec();
        let mut count = 0;
        while curr_word.len() < 256 {
            curr_word.push(curr_word[count].wrapping_mul(107));
            count += 1;
        }
        word_chunks.push(curr_word);
    }
    let mut result1: Vec<u8> = vec![];
    let mut swapped: Vec<Vec<u8>> = vec![];
    for item in &word_chunks {
        let mut sum: u32 = 0;
        for el in &*item {
            sum += *el as u32;
        }
        if sum % 2 == 0 {
            item.to_vec().reverse();
            swapped.push(item.to_vec());
        }
    }

    for (index, item) in swapped.iter().enumerate() {
        let v3: Vec<u8> = item
            .iter()
            .zip(word_chunks.iter().next().iter())
            .map(|(&x1, &x2)| x1 ^ !x2[index + 1])
            .collect();
        result1.append(
            &mut v3
                .into_iter()
                .map(|x: u8| {
                    let mut xwares: u64 = x as u64;
                    if xwares % 2 == 0 {
                        xwares = xwares.wrapping_add(42);
                        xwares = xwares.reverse_bits();
                        xwares = !xwares;
                        xwares = xwares.rotate_left(a);
                        xwares ^= 0x00FF00FF00FF00FF;

                    } else {
                        xwares = xwares.rotate_right(5);
                        xwares ^= a as u64;
                        xwares = xwares.wrapping_mul(0x9E3779B97F4A7C15);
                        xwares = (xwares.rotate_left(5) ^ xwares.rotate_right(3)).wrapping_add(0x42);
                    }
                    
                    xwares ^= xwares.wrapping_shr(24);
                    xwares = (59123_u64.wrapping_mul(xwares)).wrapping_shr(24);

                    for mut i in hash_values {

                        xwares ^= (xwares | (xwares << 8)) & i as u64;
                        xwares &= (xwares | (xwares << 4)) & i as u64;
                        xwares ^= (xwares | (xwares << 2)) & i as u64;
                        xwares &= (xwares | (xwares << 1)) & i as u64;

                        i ^= (i | (i << 8)) & xwares as u32;
                        i &= (i | (i << 4)) & xwares as u32;
                        i ^= (i | (i << 2)) & xwares as u32;
                        i &= (i | (i << 1)) & xwares as u32;

                        /*
                        
                        xwares ^= (xwares | (xwares << 8)) & 0x00FF00FF;
                        xwares &= (xwares | (xwares << 4)) & 0x0F0F0F0F;
                        xwares ^= (xwares | (xwares << 2)) & 0x33333333;
                        xwares &= (xwares | (xwares << 1)) & 0x55555555;

                        i ^= (i | (i << 8)) & 0x00FF00FF;
                        i &= (i | (i << 4)) & 0x0F0F0F0F;
                        i ^= (i | (i << 2)) & 0x33333333;
                        i &= (i | (i << 1)) & 0x55555555;
                         
                         */
                        

                        let z = !(xwares ^ ((i as u64) << 1)) ^ x as u64;
                        xwares = z;
                        //xwares = xwares.rotate_right(z as u32);
                    }

                    return xwares as u8;
                })
                .collect(),
        );
    }

    //println!("{:#0x?}", result);
    let char_result: Vec<String> = result1
        .iter()
        .map(|x| format!("{:x}", *x))
        .collect::<Vec<String>>();
    let mut string_result: String = char_result.join("");

    let mut count: usize = 0;
    while string_result.len() < 128 {
        if string_result.len() % 2 == 0 {
            string_result = format!("{count}{}", string_result);
        } else {
            string_result = format!("{}{count}", string_result);
        }
        count += 1;
    }
    string_result
}

pub fn hash_worker(inp: &str) -> String {
    let res1 = best_hash(inp);

    let res2 = best_hash(&res1);
    return res2;
}

//AI
pub fn custom_hash(message: &str) -> String {
    // Convert message to bytes
    let msg_bytes: Vec<u8> = message.chars().map(|c| c as u8).collect();

    // Padding
    let original_length = msg_bytes.len();
    let mut padded_message = msg_bytes.clone();
    padded_message.push(0x80); // Append 0b10000000

    while padded_message.len() % 64 != (64 - 8) % 64 {
        padded_message.push(padded_message[padded_message.len() - 1] % 7);
    }

    // Append original length as 64-bit big-endian
    let length_bytes = (original_length as u64 * 8).to_be_bytes();
    padded_message.extend_from_slice(&length_bytes);

    // XOR adjacent bytes
    let xor_result: Vec<u8> = padded_message
        .windows(2)
        .step_by(2)
        .map(|pair| pair[0] ^ pair[1])
        .collect();

    // Sum adjacent bytes
    let sum_result: Vec<u8> = xor_result
        .windows(2)
        .map(|pair| pair[0].wrapping_add(pair[1]))
        .collect();

    // Half and XOR
    let half_xor_result: Vec<u8> = sum_result
        .windows(2)
        .map(|pair| pair[0] & !pair[1])
        .collect();

    // XOR adjacent bytes
    let xor_result2: Vec<u8> = half_xor_result
        .windows(2)
        .step_by(2)
        .map(|pair| pair[0] ^ pair[1])
        .collect();

    // Convert to hexadecimal string
    let hex_result: String = xor_result2.iter().map(|&x| format!("{:02x}", x)).collect();

    hex_result
}

//AI
pub fn improved_hash(message: &str) -> String {
    let mut msg_bytes: Vec<u8> = message.bytes().collect();

    // Padding
    msg_bytes.push(0b10000000);
    while msg_bytes.len() % 64 != (64 - 8) % 64 {
        msg_bytes.push(0);
    }

    // Append the original length in bits
    let original_length_bits = (message.len() as u64 * 8).to_be_bytes();
    msg_bytes.extend_from_slice(&original_length_bits);

    // Constants for the hash function
    let mut hash_values: [u32; 8] = [
        0xEBEDD428, 0x58598FED, 0x345DE15D, 0x1427889B, 0x44ECF241, 0x24683D1B, 0x3BBA9ED7,
        0x1C829AA6,
    ];

    // Hash computation
    for chunk in msg_bytes.chunks(64) {
        let mut words: Vec<u32> = Vec::new();

        // Convert each 4 bytes to u32
        for i in (0..chunk.len()).step_by(4) {
            let word = u32::from_be_bytes([
                chunk[i],
                chunk[i + 1],
                chunk[i + 2],
                chunk[i + 3],
            ]);
            words.push(word);
        }

        for &word in &words {
            for _i in 0..8 {
                let temp = hash_values[7].wrapping_add(word.rotate_left(7));
                hash_values[7] = hash_values[6];
                hash_values[6] = hash_values[5];
                hash_values[5] = hash_values[4];
                hash_values[4] = hash_values[3].wrapping_add(temp);
                hash_values[3] = hash_values[2];
                hash_values[2] = hash_values[1];
                hash_values[1] = hash_values[0];
                hash_values[0] = temp.wrapping_add(hash_values[0].wrapping_add(0x9E3779B9));
            };
        }
    }

    // Convert hash values to a hex string
    let mut hash_str: String = hash_values.iter().flat_map(|&x| x.to_be_bytes()).map(|b| format!("{:02x}", b)).collect();

    hash_str.truncate(64);
    hash_str
}

pub fn hash3(message: &str) -> String {
    //byte conversion
    let msg = message
        .chars()
        .into_iter()
        .map(|x| x as u8)
        .collect::<Vec<u8>>()
        .iter()
        .map(|x| format!("{:x}", x))
        .collect::<Vec<String>>()
        .join("");
    let msg2 = msg.as_bytes();

    //padding
    let original_length = msg2.len();
    let mut padded_message = msg2.to_vec();
    padded_message.push(0b10000000);

    let mut count1 = 0;
    while padded_message.len() % 64 != (64 - 8) % 64 {
        padded_message.push(padded_message[count1] % 7);
        count1 += 1;
    }
    let length_bits = (original_length as u64 * 8).to_be_bytes();
    padded_message.extend_from_slice(&length_bits);

    //first split and xor
    let chunks: Vec<_> = padded_message.chunks(2).collect();
    let mut result: Vec<u8> = vec![];
    for chunk in chunks {
        let midpoint = chunk.len() / 2;
        let (first_half, second_half) = chunk.split_at(midpoint);
        let mut processed: Vec<u8> = first_half
            .iter()
            .zip(second_half.iter())
            .map(|(&a, &b)| a ^ b)
            .collect();
        result.append(&mut processed);
    }

    //sum of items
    let mut sum: Vec<u8> = vec![];
    for i in (0..result.len()).step_by(2) {
        sum.push(result[i] + result[i + 1]);
    }

    // half and xor 2
    let chunks2: Vec<_> = sum.chunks(2).collect();
    let mut result2: Vec<u8> = vec![];
    for chunk in chunks2 {
        let midpoint = chunk.len() / 2;
        let (first_half, second_half) = chunk.split_at(midpoint);
        let mut processed: Vec<u8> = first_half
            .iter()
            .zip(second_half.iter())
            .map(|(&a, &b)| a & !b)
            .collect();
        result2.append(&mut processed);
    }

    let sum2: Vec<String> = result2
        .into_iter()
        .map(|x| format!("{:x}", x as u8))
        .collect();
    let sum3 = sum2.join("");
    let chunks3: Vec<_> = sum3.as_bytes().chunks(16).collect();
    let mut result3: Vec<u8> = vec![];
    for chunk in chunks3 {
        let midpoint = chunk.len() / 2;
        let (first_half, second_half) = chunk.split_at(midpoint);
        let mut processed: Vec<u8> = first_half
            .iter()
            .zip(second_half.iter())
            .map(|(&a, &b)| a ^ b)
            .collect();
        result3.append(&mut processed);
    }
    let mut count2 = 0;
    let result4 = result3
        .into_iter()
        .map(|x| format!("{:x}", x as u8))
        .collect::<Vec<String>>();
    let mut result5 = result4.join("").chars().collect::<Vec<char>>();

    while result5.len() % 64 != 0 {
        result5.push((result5[count2] as u8 & result5[count2 + 1] as u8) as char);
        count2 += 1;
    }
    let result6_i = BigUint::parse_bytes(
        result5
            .into_iter()
            .map(|x| format!("{:x}", x as u8))
            .collect::<String>()
            .as_bytes(),
        10,
    )
    .unwrap();
    let mut result6: Vec<char> = format!("{:x}", result6_i).chars().collect();

    let mut count3 = 0;
    while result6.len() % 32 != 0 {
        //result6.push('0');
        result6.push((result6[count3] as u8 ^ 7) as char);
        count3 += 1;
    }
    let res7: String = result6.iter().collect();
    res7
}

pub fn fnv1a(key: &str) -> u32 {
    let mut hash: u32 = 2166136261;
    let x = key.chars().collect::<Vec<char>>();
    for i in x.iter().take(key.len()) {
        let y = i.to_owned();
        hash ^= (y as u8) as u32;
        hash = hash.overflowing_mul(16777619).0;
    }
    hash
}

pub fn increment_string(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let mut carry = true;

    for i in (0..chars.len()).rev() {
        if carry {
            if chars[i] == 'z' {
                chars[i] = 'a';
            } else {
                chars[i] = (chars[i] as u8 + 1) as char;
                carry = false;
            }
        }
    }

    if carry {
        chars.insert(0, 'a');
    }

    chars.into_iter().collect()
}

#[cfg(test)]
mod tests1 {
    use super::*;

    /*
        #[test]
    fn it_works() {
        let handle3 = std::thread::spawn(move || {
            let start = String::from("a");
            let end = String::from("zzz");
            let mut previous: Vec<String> = vec![];
            let mut current = start.clone();
            while current <= end {
                let z = improved_hash(&current.clone());
                println!("{current}\t-\t{z}");
                for (index, _item) in previous.iter().enumerate() {
                    assert_ne!(z, previous[index])
                }
                previous.push(z);
                current = increment_string(&current);
            }
        });

        handle3.join().unwrap();
    }
        */
    #[test]
    fn collision_test1() {
        let inputs = vec!["input1", "input2", "input3"];
        let mut hash_set = std::collections::HashSet::new();

        for input in inputs {
            let hash_value = improved_hash(input);
            assert!(
                !hash_set.contains(&hash_value),
                "Collision detected for input: {}",
                input
            );
            hash_set.insert(hash_value);
        }
    }

    #[test]
    fn security_test1() {
        let input = "test_input";
        let hash_value1 = improved_hash(input);
        let hash_value2 = improved_hash(input);
        println!("{hash_value1}\n\n{hash_value2}");
        assert_eq!(
            hash_value1, hash_value2,
            "Hash values for the same input are not consistent"
        );
    }

    #[test]
    fn distribution_test1() {
        let num_inputs = 1000;
        let mut hash_count = std::collections::HashMap::new();

        for i in 0..num_inputs {
            let input = format!("input{}", i);
            let hash_value = improved_hash(&input);

            *hash_count.entry(hash_value).or_insert(0) += 1;
        }

        // Check that no hash value has a significantly higher count than others.
        let max_count = *hash_count.values().max().unwrap();
        let min_count = *hash_count.values().min().unwrap();
        assert!(
            max_count - min_count <= 1,
            "Hash values are not evenly distributed"
        );
    }

    #[test]
    fn performance_test1() {
        let num_inputs = 100000;
        let input = "test_input";

        let start_time = std::time::Instant::now();

        for _ in 0..num_inputs {
            improved_hash(input);
        }

        let elapsed_time = start_time.elapsed();
        println!("Hashed {} inputs in {:?}", num_inputs, elapsed_time);
    }
    #[test]
    fn it_works() {
        let start = String::from("a");
        let end = String::from("zz");
        let mut previous: Vec<String> = vec![];
        let mut current = start.clone();
        while current <= end {
            let z = improved_hash(&current.clone());
            println!("{current}\t-\t{z}");
            for (index, _item) in previous.iter().enumerate() {
                assert_ne!(z, previous[index])
            }
            previous.push(z);
            current = increment_string(&current);
        }
    }

    #[test]
    fn qbc() {
        let handle3 = std::thread::spawn(move || {
            let x = improved_hash("The quick brown fox jumps over the lazy dog");
            println!("{x}");
            let y = improved_hash("The quick brown fox jumps over the lazy dog.");
            println!("{y}");
            let z = improved_hash("The quick brown fox jumps over the lazy bog");
            println!("{z}");
            assert_ne!(x, y);
            assert_ne!(x, z);
        });

        handle3.join().unwrap();
    }

    #[test]
    fn empty() {
        let res = improved_hash("");
        println!("{res}");
    }

}

#[cfg(test)]
mod tests2 {
    use super::*;
    #[test]
    fn collision_test2() {
        let inputs = vec!["input1", "input2", "input3"];
        let mut hash_set = std::collections::HashSet::new();

        for input in inputs {
            let hash_value = hash3(input);
            assert!(
                !hash_set.contains(&hash_value),
                "Collision detected for input: {}",
                input
            );
            hash_set.insert(hash_value);
        }
    }

    #[test]
    fn security_test2() {
        let input = "test_input";
        let hash_value1 = hash3(input);
        let hash_value2 = hash3(input);
        println!("{hash_value1}\n\n{hash_value2}");
        assert_eq!(
            hash_value1, hash_value2,
            "Hash values for the same input are not consistent"
        );
    }

    #[test]
    fn distribution_test2() {
        let num_inputs = 1000;
        let mut hash_count = std::collections::HashMap::new();

        for i in 0..num_inputs {
            let input = format!("input{}", i);
            let hash_value = hash3(&input);

            *hash_count.entry(hash_value).or_insert(0) += 1;
        }

        // Check that no hash value has a significantly higher count than others.
        let max_count = *hash_count.values().max().unwrap();
        let min_count = *hash_count.values().min().unwrap();
        assert!(
            max_count - min_count <= 1,
            "Hash values are not evenly distributed!\n {max_count}\n\n {min_count}"
        );
    }

    #[test]
    fn performance_test2() {
        let num_inputs = 100000;
        let input = "test_input";

        let start_time = std::time::Instant::now();

        for _ in 0..num_inputs {
            hash3(input);
        }

        let elapsed_time = start_time.elapsed();
        println!("Hashed {} inputs in {:?}", num_inputs, elapsed_time);
    }

    #[test]
    fn it_works() {
        let handle3 = std::thread::spawn(move || {
            let start = String::from("a");
            let end = String::from("zz");
            let mut previous: Vec<String> = vec![];
            let mut current = start.clone();
            while current <= end {
                let z = hash3(&current.clone());
                println!("{current}\t-\t{z}");
                for (index, _item) in previous.iter().enumerate() {
                    assert_ne!(z, previous[index])
                }
                previous.push(z);
                current = increment_string(&current);
            }
        });

        handle3.join().unwrap();
    }

    #[test]
    fn qbc() {
        let handle3 = std::thread::spawn(move || {
            let x = hash3("The quick brown fox jumps over the lazy dog");

            println!("{x}");
            let y = hash3("The quick brown fox jumps over the lazy dog.");
            println!("{y}");
            let z = hash3("The quick brown fox jumps over the lazy bog");
            println!("{z}");
            assert_ne!(x, y);
            assert_ne!(x, z);
        });

        handle3.join().unwrap();
    }
}

#[cfg(test)]
mod tests3 {
    use super::*;
    #[test]
    fn collision_test() {
        let inputs = vec!["input1", "input2", "input3"];
        let mut hash_set = std::collections::HashSet::new();

        for input in inputs {
            let hash_value = custom_hash(input);
            assert!(
                !hash_set.contains(&hash_value),
                "Collision detected for input: {}",
                input
            );
            hash_set.insert(hash_value);
        }
    }

    #[test]
    fn security_test() {
        let input = "test_input";
        let hash_value1 = custom_hash(input);
        let hash_value2 = custom_hash(input);
        println!("{hash_value1}\n\n{hash_value2}");
        assert_eq!(
            hash_value1, hash_value2,
            "Hash values for the same input are not consistent"
        );
    }

    #[test]
    fn distribution_test() {
        let num_inputs = 1000;
        let mut hash_count = std::collections::HashMap::new();

        for i in 0..num_inputs {
            let input = format!("input{}", i);
            let hash_value = custom_hash(&input);

            *hash_count.entry(hash_value).or_insert(0) += 1;
        }

        // Check that no hash value has a significantly higher count than others.
        let max_count = *hash_count.values().max().unwrap();
        let min_count = *hash_count.values().min().unwrap();
        assert!(
            max_count - min_count <= 1,
            "Hash values are not evenly distributed!\n {max_count}\n\n {min_count}"
        );
    }

    #[test]
    fn performance_test() {
        let num_inputs = 100000;
        let input = "test_input";

        let start_time = std::time::Instant::now();

        for _ in 0..num_inputs {
            custom_hash(input);
        }

        let elapsed_time = start_time.elapsed();
        println!("Hashed {} inputs in {:?}", num_inputs, elapsed_time);
    }

    #[test]
    fn it_works() {
        let handle3 = std::thread::spawn(move || {
            let start = String::from("a");
            let end = String::from("zz");
            let mut previous: Vec<String> = vec![];
            let mut current = start.clone();
            while current <= end {
                let z = custom_hash(&current.clone());
                println!("{current}\t-\t{z}");
                for (index, _item) in previous.iter().enumerate() {
                    assert_ne!(z, previous[index])
                }
                previous.push(z);
                current = increment_string(&current);
            }
        });

        handle3.join().unwrap();
    }
}

#[cfg(test)]
mod tests4 {
    use super::*;
    #[test]
    fn collision_test() {
        let inputs = vec!["input1", "input2", "input3"];
        let mut hash_set = std::collections::HashSet::new();

        for input in inputs {
            let hash_value = best_hash(input);
            assert!(
                !hash_set.contains(&hash_value),
                "Collision detected for input: {}",
                input
            );
            hash_set.insert(hash_value);
        }
    }

    #[test]
    fn security_test() {
        let input = "test_input";
        let hash_value1 = best_hash(input);
        let hash_value2 = best_hash(input);
        println!("{hash_value1}\n\n{hash_value2}");
        assert_eq!(
            hash_value1, hash_value2,
            "Hash values for the same input are not consistent"
        );
    }

    #[test]
    fn distribution_test() {
        let num_inputs = 1000;
        let mut hash_count = std::collections::HashMap::new();

        for i in 0..num_inputs {
            let input = format!("input{}", i);
            let hash_value = best_hash(&input);

            *hash_count.entry(hash_value).or_insert(0) += 1;
        }

        // Check that no hash value has a significantly higher count than others.
        let max_count = *hash_count.values().max().unwrap();
        let min_count = *hash_count.values().min().unwrap();
        assert!(
            max_count - min_count <= 1,
            "Hash values are not evenly distributed!\n {max_count}\n\n {min_count}"
        );
    }

    #[test]
    fn performance_test() {
        let num_inputs = 1000;
        let input = "test";

        let start_time = std::time::Instant::now();

        for _ in 0..num_inputs {
            best_hash(input);
        }

        let elapsed_time = start_time.elapsed();
        println!("Hashed {} inputs in {:?}", num_inputs, elapsed_time);
    }

    #[test]
    fn it_works() {
        let start = String::from("a");
        let end = String::from("zz");
        let mut previous: Vec<String> = vec![];
        let mut current = start.clone();
        while current <= end {
            let z = best_hash(&current.clone());
            println!("{current}\t-\t{z}");
            for (index, _item) in previous.iter().enumerate() {
                assert_ne!(z, previous[index])
            }
            previous.push(z);
            current = increment_string(&current);
        }
    }

    #[test]
    fn qbc() {
        let handle3 = std::thread::spawn(move || {
            let x = best_hash("The quick brown fox jumps over the lazy dog");
            println!("{x}");
            let y = best_hash("The quick brown fox jumps over the lazy dog.");
            println!("{y}");
            let z = best_hash("The quick brown fox jumps over the lazy bog");
            println!("{z}");
            assert_ne!(x, y);
            assert_ne!(x, z);
        });

        handle3.join().unwrap();
    }

    #[test]
    fn empty() {
        let res = best_hash("");
        println!("{res}");
    }
}


#[cfg(test)]
mod tests5 {
    use super::*;
    #[test]
    fn collision_test() {
        let inputs = vec!["input1", "input2", "input3"];
        let mut hash_set = std::collections::HashSet::new();

        for input in inputs {
            let hash_value = hash1::hash(input);
            assert!(
                !hash_set.contains(&hash_value),
                "Collision detected for input: {}",
                input
            );
            hash_set.insert(hash_value);
        }
    }

    #[test]
    fn security_test() {
        let input = "test_input";
        let hash_value1 = hash1::hash(input);
        let hash_value2 = hash1::hash(input);
        println!("{hash_value1}\n\n{hash_value2}");
        assert_eq!(
            hash_value1, hash_value2,
            "Hash values for the same input are not consistent"
        );
    }

    #[test]
    fn distribution_test() {
        let num_inputs = 1000;
        let mut hash_count = std::collections::HashMap::new();

        for i in 0..num_inputs {
            let input = format!("input{}", i);
            let hash_value = hash1::hash(&input);

            *hash_count.entry(hash_value).or_insert(0) += 1;
        }

        // Check that no hash value has a significantly higher count than others.
        let max_count = *hash_count.values().max().unwrap();
        let min_count = *hash_count.values().min().unwrap();
        assert!(
            max_count - min_count <= 1,
            "Hash values are not evenly distributed!\n {max_count}\n\n {min_count}"
        );
    }

    #[test]
    fn performance_test() {
        let num_inputs = 1000;
        let input = "test";

        let start_time = std::time::Instant::now();

        for _ in 0..num_inputs {
            hash1::hash(input);
        }

        let elapsed_time = start_time.elapsed();
        println!("Hashed {} inputs in {:?}", num_inputs, elapsed_time);
    }

    #[test]
    fn it_works() {
        let start = String::from("a");
        let end = String::from("zz");
        let mut previous: Vec<String> = vec![];
        let mut current = start.clone();
        while current <= end {
            let z = hash1::hash(&current.clone());
            println!("{current}\t-\t{z}");
            for (index, _item) in previous.iter().enumerate() {
                assert_ne!(z, previous[index])
            }
            previous.push(z);
            current = increment_string(&current);
        }
    }

    #[test]
    fn qbc() {
        let handle3 = std::thread::spawn(move || {
            let x = hash1::hash("The quick brown fox jumps over the lazy dog");
            println!("{x}\n");
            let y = hash1::hash("The quick brown fox jumps over the lazy dog.");
            println!("{y}\n");
            let z = hash1::hash("The quick brown fox jumps over the lazy bog");
            println!("{z}\n");
            assert_ne!(x, y);
            assert_ne!(x, z);
        });

        handle3.join().unwrap();
    }

    #[test]
    fn empty() {
        let res = hash1::hash("");
        println!("{res}");
    }
}