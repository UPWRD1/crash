mod crash;

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
    use crash::K0;
    #[test]
    fn collision_test() {
        let inputs = vec!["input", "jnput", "knput"];
        let mut hash_set = std::collections::HashSet::new();
        for input in inputs {
            let hash_value = K0::hash(input);
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
        let hash_value1 = K0::hash("test_input");
        let hash_value2 = K0::hash("test_input");
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

        let start_time = std::time::Instant::now();

        for i in 0_i32..num_inputs {
            let input = format!("input{}", i);
            let hash_value = K0::hash(&input);

            *hash_count.entry(hash_value).or_insert(0) += 1;
        }

        // Check that no hash value has a significantly higher count than others.
        let max_count = *hash_count.values().max().unwrap();
        let min_count = *hash_count.values().min().unwrap();
        let elapsed_time = start_time.elapsed();
        println!("Hashed {} inputs in {:?}", num_inputs, elapsed_time);
        assert!(
            max_count - min_count <= 1
            ,
            "Hash values are not evenly distributed!\n {max_count}\n\n {min_count}"
        );
        println!("Difference: {} ({}), ({})", max_count - min_count, max_count, min_count)
    }

    #[test]
    fn performance_test() {
        let num_inputs = 1000;
        let input = "test";

        let start_time = std::time::Instant::now();

        for _ in 0..num_inputs {
            let _ = K0::hash(input);
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
            let z = K0::hash(&current);
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
            let x = K0::hash("The quick brown fox jumps over the lazy dog".into()); 
            println!("{x}\n");
            let y = K0::hash("The quick brown fox jumps over the ØÂôò@ÈÞ".into()); 
            println!("{y}\n");
            assert_ne!(x, y);
    }

    #[test]
    fn empty() {
        let res = K0::hash("");
        println!("{res}");
    }

    #[test]
    fn dump() {
        let start = String::from("a");
        let end = String::from("z");
        let mut current = start.clone();
        while current <= end {
            let z = K0::hash(&current);
            println!("{z}");
            current = increment_string(&current);
            //let ten_millis = std::time::Duration::from_millis(50);

            //std::thread::sleep(ten_millis);
        }
    }

    #[test]
    fn dogfood() {
        let hashres = K0::hash("This is the starting hash. This is a long sentance that will be dogfooded back into the hash algorithm.");
        let hashres2 = K0::hash(&hashres);
        println!("{hashres}\n\n{hashres2}");
        assert_ne!(hashres, hashres2);
    }

    #[test]
    fn len_ext() {
        let hashres = K0::hash("count=10&lat=37.351&user_id=1&long=-119.827&waffle=eggo");
        let hashres2 = K0::hash("count=10&lat=37.351&user_id=1&long=-119.827&waffle=eggo\x7f\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02\x28\x28&waffle=liege");
        println!("{hashres}\n\n{hashres2}");
        assert_ne!(hashres, hashres2);
    }
}
