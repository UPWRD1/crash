mod hash1;

pub fn best_hash(message: &str) -> String {
    // byte conversion
    let msg_bytes: Vec<u8> = message.chars().map(|x| x as u8).collect();

    // padding
    let original_length = msg_bytes.len();
    let mut padded_message = msg_bytes.clone();
    padded_message.push(0x0);

    while padded_message.len() % 256 != (256 - 8) {
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
        for el in &**item {
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
            .zip(word_chunks.first())
            .map(|(&x1, x2)| x1 ^ !x2[index + 1])
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
                        xwares =
                            (xwares.rotate_left(5) ^ xwares.rotate_right(3)).wrapping_add(0x42);
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

                    xwares as u8
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
    string_result.truncate(64);

    string_result
        .chars()
        .collect::<Vec<char>>()
        .chunks(8)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("-")
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

        for i in 0_i32..num_inputs {
            let input = format!("input{}", i);
            let hash_value = hash1::hash(&input);

            *hash_count.entry(hash_value).or_insert(0) += 1;
        }

        // Check that no hash value has a significantly higher count than others.
        let max_count = *hash_count.values().max().unwrap();
        let min_count = *hash_count.values().min().unwrap();
        assert!(
            max_count - min_count <= 2,
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
        let start = String::from("aa");
        let end = String::from("z");
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
            //let ten_millis = std::time::Duration::from_millis(50);

            //std::thread::sleep(ten_millis);
        }
    }

    #[test]
    fn qbc() {
            let x = hash1::hash("The quick brown fox jumps over the lazy dog");
            println!("{x}\n");
            let y = hash1::hash("The quick brown fox jumps over the lazy dog.");
            println!("{y}\n");
            let z = hash1::hash("The quick brown fox jumps over the lazy bog");
            println!("{z}\n");
            assert_ne!(x, y);
            assert_ne!(x, z);
    }

    #[test]
    fn empty() {
        let res = hash1::hash("");
        println!("{res}");
    }
}
