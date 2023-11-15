mod hash1;

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
mod tests {
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
