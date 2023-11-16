mod shade;

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
mod test_hash {
    use super::*;
    use shade::Shade;
    #[test]
    fn collision_test() {
        let inputs = vec!["input1", "jnput", "knput"];
        let mut hash_set = std::collections::HashSet::new();
        for input in inputs {
            let hash_value = Shade::hash(input, 256);
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
        let hash_value1 = Shade::hash("test_input", 256);
        let hash_value2 = Shade::hash("test_input", 256);
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
            let hash_value = Shade::hash(&input, 256);

            *hash_count.entry(hash_value).or_insert(0) += 1;
        }

        // Check that no hash value has a significantly higher count than others.
        let max_count = *hash_count.values().max().unwrap();
        let min_count = *hash_count.values().min().unwrap();
        assert!(
            max_count - min_count <= 2
            ,
            "Hash values are not evenly distributed!\n {max_count}\n\n {min_count}"
        );
    }

    #[test]
    fn performance_test() {
        let num_inputs = 10000;
        let input = "test";

        let start_time = std::time::Instant::now();

        for _ in 0..num_inputs {
            let _ = Shade::hash(input, 256);
        }

        let elapsed_time = start_time.elapsed();
        println!("Hashed {} inputs in {:?}", num_inputs, elapsed_time);
    }

    #[test]
    fn bulk() {
        let start = String::from("aa");
        let end = String::from("zz");
        let mut previous: Vec<String> = vec![];
        let mut current = start.clone();
        while current <= end {
            let z = Shade::hash(&current,256);
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
    fn qbf() {
            let x = Shade::hash("The quick brown fox jumps over the lazy dog".into(), 256); 
            println!("{x}\n");
            let y = Shade::hash("The quick brown fox jumps over the ØÂôò@ÈÞ".into(), 256); 
            println!("{y}\n");
            assert_ne!(x, y);
    }


    #[test]
    fn empty() {
        let res = Shade::hash("", 256);
        println!("{res}");
    }

    #[test]
    fn dump() {
        let start = String::from("a");
        let end = String::from("zz");
        let mut current = start.clone();
        while current <= end {
            let z = Shade::hash(&current,256);
            println!("{z}");
            current = increment_string(&current);
            //let ten_millis = std::time::Duration::from_millis(50);

            //std::thread::sleep(ten_millis);
        }
    }
}


#[cfg(test)]
mod test_salt {
    use super::*;
    use shade::Shade;
    #[test]
    fn collision_test() {
        let inputs = vec!["input1", "jnput", "knput"];
        let mut hash_set = std::collections::HashSet::new();
        for input in inputs {
            let hash_value = Shade::salt_hash(input, 256, ";adf 91874/2");
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
        let hash_value1 = Shade::salt_hash("test_input", 256, ";adf 91874/2");
        let hash_value2 = Shade::salt_hash("test_input", 256, ";adf 91874/2");
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
            let hash_value = Shade::salt_hash(&input, 256, ";adf 91874/2");

            *hash_count.entry(hash_value).or_insert(0) += 1;
        }

        // Check that no hash value has a significantly higher count than others.
        let max_count = *hash_count.values().max().unwrap();
        let min_count = *hash_count.values().min().unwrap();
        assert!(
            max_count - min_count <= 2
            ,
            "Hash values are not evenly distributed!\n {max_count}\n\n {min_count}"
        );
    }

    #[test]
    fn performance_test() {
        let num_inputs = 10000;
        let input = "test";

        let start_time = std::time::Instant::now();

        for _ in 0..num_inputs {
            let _ = Shade::salt_hash(input, 256, ";adf 91874/2");
        }

        let elapsed_time = start_time.elapsed();
        println!("Hashed {} inputs in {:?}", num_inputs, elapsed_time);
    }

    #[test]
    fn bulk() {
        let start = String::from("aa");
        let end = String::from("zz");
        let mut previous: Vec<String> = vec![];
        let mut current = start.clone();
        while current <= end {
            let z = Shade::salt_hash(&current,256, ";adf 91874/2");
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
    fn qbf() {
            let x = Shade::salt_hash("The quick brown fox jumps over the lazy dog".into(), 256, ";adf 91874/2"); 
            println!("{x}\n");
            let y = Shade::salt_hash("The quick brown fox jumps over the ØÂôò@ÈÞ".into(), 256, ";adf 91874/2"); 
            println!("{y}\n");
            assert_ne!(x, y);
    }


    #[test]
    fn empty() {
        let res = Shade::salt_hash("", 256, ";adf 91874/2");
        println!("{res}");
    }
}
